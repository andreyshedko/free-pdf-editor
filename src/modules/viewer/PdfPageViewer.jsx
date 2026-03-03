import React, { useEffect, useRef, useState, useCallback, memo } from 'react';
import { usePdfDocumentStore } from '@store/pdfDocumentStore';
import { usePageManagementStore } from '@store/pageManagementStore';
import { getOrLoadPdfDocument } from '@core/pdf/pdfjsService';
import { AnnotationOverlay } from './AnnotationOverlay';

const ESTIMATED_PAGE_HEIGHT_PX = 900;
const PRELOAD_MARGIN = '600px 0px';

const PdfSinglePage = memo(({ fileData, pageNumber, pageIndex, scale, onDimensionsKnown }) => {
  const canvasRef = useRef(null);
  const [dims, setDims] = useState(null);
  const [error, setError] = useState(null);

  useEffect(() => {
    let cancelled = false;
    const render = async () => {
      try {
        const pdf = await getOrLoadPdfDocument(fileData);
        if (cancelled) return;
        const page = await pdf.getPage(pageNumber);
        if (cancelled) return;
        const viewport = page.getViewport({ scale });
        const canvas = canvasRef.current;
        if (!canvas) return;
        const ctx = canvas.getContext('2d');
        if (!ctx) return;
        const dpr = window.devicePixelRatio || 1;
        canvas.width = viewport.width * dpr;
        canvas.height = viewport.height * dpr;
        canvas.style.width = `${viewport.width}px`;
        canvas.style.height = `${viewport.height}px`;
        ctx.scale(dpr, dpr);
        const d = { width: viewport.width, height: viewport.height };
        setDims(d);
        onDimensionsKnown(d);
        await page.render({ canvasContext: ctx, viewport }).promise;
      } catch (e) {
        if (!cancelled) setError(e instanceof Error ? e.message : 'Render error');
      }
    };
    void render();
    return () => { cancelled = true; };
  }, [fileData, pageNumber, scale, onDimensionsKnown]);

  if (error) {
    return (
      <div className="page-wrapper page-error" style={{ minHeight: 120 }}>
        <span className="muted">Page {pageNumber}: {error}</span>
      </div>
    );
  }

  return (
    <div className="page-wrapper" style={{ position: 'relative' }}>
      <canvas ref={canvasRef} className="page-canvas" />
      {dims && (
        <AnnotationOverlay
          pageIndex={pageIndex}
          width={dims.width}
          height={dims.height}
        />
      )}
    </div>
  );
});

export const PdfPageViewer = () => {
  const { fileData, pageCount, setPageCount, setCurrentPageIndex, setDocument, zoom } = usePdfDocumentStore();
  const { pageOrder, initPages } = usePageManagementStore();
  const containerRef = useRef(null);
  const pageRefs = useRef(new Map());

  const [renderSet, setRenderSet] = useState(new Set());
  const [dimCache, setDimCache] = useState(new Map());

  const scale = zoom / 100;

  useEffect(() => {
    if (fileData) {
      getOrLoadPdfDocument(fileData).then((pdf) => {
        setPageCount(pdf.numPages);
        initPages(pdf.numPages);
        setRenderSet(new Set(Array.from({ length: Math.min(3, pdf.numPages) }, (_, i) => i)));
        setDimCache(new Map());
      }).catch(() => {/* ignore */});
    }
  }, [fileData, setPageCount, initPages]);

  useEffect(() => {
    const container = containerRef.current;
    if (!container || !pageCount) return;

    const visibleObserver = new IntersectionObserver(
      (entries) => {
        entries.forEach((entry) => {
          if (entry.isIntersecting) {
            const idx = Number(entry.target.dataset.pageIndex ?? -1);
            if (idx >= 0) setCurrentPageIndex(idx);
          }
        });
      },
      { root: container, threshold: 0.5 }
    );

    const preloadObserver = new IntersectionObserver(
      (entries) => {
        const toAdd = [];
        entries.forEach((entry) => {
          if (entry.isIntersecting) {
            const idx = Number(entry.target.dataset.pageIndex ?? -1);
            if (idx >= 0) toAdd.push(idx);
          }
        });
        if (toAdd.length > 0) {
          setRenderSet((prev) => {
            const next = new Set(prev);
            toAdd.forEach((i) => next.add(i));
            return next;
          });
        }
      },
      { root: container, rootMargin: PRELOAD_MARGIN }
    );

    pageRefs.current.forEach((el) => {
      visibleObserver.observe(el);
      preloadObserver.observe(el);
    });

    return () => {
      visibleObserver.disconnect();
      preloadObserver.disconnect();
    };
  }, [pageCount, setCurrentPageIndex]);

  const handleDimsKnown = useCallback((listIndex, dims) => {
    setDimCache((prev) => {
      const existing = prev.get(listIndex);
      if (existing && existing.width === dims.width && existing.height === dims.height) return prev;
      const next = new Map(prev);
      next.set(listIndex, dims);
      return next;
    });
  }, []);

  if (!fileData) {
    return (
      <div className="empty-state">
        <div className="empty-orbit" aria-hidden="true">
          <div className="empty-orbit-inner">
            <div className="empty-orbit-glow" />
            <div className="empty-orbit-badge">Local-only · Private</div>
          </div>
        </div>
        <div className="empty-caption">
          <h2>Open a PDF to get started</h2>
          <p>
            Work entirely on your device with a modern, focused interface. Editing, annotations, OCR, and export tools
            will light up here.
          </p>
        </div>
        <div className="empty-actions">
          <button
            className="btn btn-strong"
            onClick={async () => {
              const opened = await window.electronAPI?.openPdf?.();
              if (opened) setDocument({ data: opened.data, name: opened.name });
            }}
          >
            <span className="btn-icon">＋</span>
            Open PDF
          </button>
        </div>
      </div>
    );
  }

  const pages = pageOrder.length > 0 ? pageOrder : (pageCount ? Array.from({ length: pageCount }, (_, i) => i) : []);

  return (
    <div ref={containerRef} className="viewer-scroll" aria-label="PDF pages">
      {pages.map((origIndex, listIndex) => {
        const isRendered = renderSet.has(listIndex);
        const cached = dimCache.get(listIndex);
        const placeholderHeight = cached ? cached.height : ESTIMATED_PAGE_HEIGHT_PX * scale;

        return (
          <div
            key={`${origIndex}-${listIndex}`}
            data-page-index={listIndex}
            ref={(el) => { if (el) pageRefs.current.set(listIndex, el); else pageRefs.current.delete(listIndex); }}
            className="page-holder"
          >
            {isRendered ? (
              <PdfSinglePage
                fileData={fileData}
                pageNumber={origIndex + 1}
                pageIndex={listIndex}
                scale={scale}
                onDimensionsKnown={(dims) => handleDimsKnown(listIndex, dims)}
              />
            ) : (
              <div
                className="page-wrapper page-placeholder"
                style={{ height: placeholderHeight }}
                aria-label={`Page ${origIndex + 1} loading`}
              />
            )}
          </div>
        );
      })}
    </div>
  );
};
