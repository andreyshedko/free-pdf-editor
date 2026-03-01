import React, { useEffect, useRef, useState, useCallback } from 'react';
import { usePdfDocumentStore } from '@store/pdfDocumentStore';
import { usePageManagementStore } from '@store/pageManagementStore';
import { getOrLoadPdfDocument } from '@core/pdf/pdfjsService';
import { AnnotationOverlay } from './AnnotationOverlay';

interface PageDimensions {
  width: number;
  height: number;
}

const PdfSinglePage: React.FC<{
  fileData: ArrayBuffer;
  pageNumber: number;
  pageIndex: number;
  scale: number;
  onDimensionsKnown: (dims: PageDimensions) => void;
}> = ({ fileData, pageNumber, pageIndex, scale, onDimensionsKnown }) => {
  const canvasRef = useRef<HTMLCanvasElement>(null);
  const [dims, setDims] = useState<PageDimensions | null>(null);
  const [error, setError] = useState<string | null>(null);

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
        const d = { width: viewport.width, height: viewport.height };
        setDims(d);
        onDimensionsKnown(d);
        await page.render({ canvasContext: ctx, viewport } as any).promise;
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
};

export const PdfPageViewer: React.FC = () => {
  const { fileData, pageCount, setPageCount, setCurrentPageIndex, setDocument, zoom } = usePdfDocumentStore();
  const { pageOrder, initPages } = usePageManagementStore();
  const containerRef = useRef<HTMLDivElement | null>(null);
  const pageRefs = useRef<Map<number, HTMLDivElement>>(new Map());

  const scale = zoom / 100;

  useEffect(() => {
    if (fileData) {
      getOrLoadPdfDocument(fileData).then((pdf) => {
        setPageCount(pdf.numPages);
        initPages(pdf.numPages);
      }).catch(() => {/* ignore */});
    }
  }, [fileData, setPageCount, initPages]);

  useEffect(() => {
    const container = containerRef.current;
    if (!container || !pageCount) return;
    const observer = new IntersectionObserver(
      (entries) => {
        entries.forEach((entry) => {
          if (entry.isIntersecting) {
            const idx = Number((entry.target as HTMLElement).dataset.pageIndex ?? -1);
            if (idx >= 0) setCurrentPageIndex(idx);
          }
        });
      },
      { root: container, threshold: 0.5 }
    );
    pageRefs.current.forEach((el) => observer.observe(el));
    return () => observer.disconnect();
  }, [pageCount, setCurrentPageIndex]);

  const handleDims = useCallback(() => {/* no-op */}, []);

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
      {pages.map((origIndex, listIndex) => (
        <div
          key={`${origIndex}-${listIndex}`}
          data-page-index={listIndex}
          ref={(el) => { if (el) pageRefs.current.set(listIndex, el); else pageRefs.current.delete(listIndex); }}
          className="page-holder"
        >
          <PdfSinglePage
            fileData={fileData}
            pageNumber={origIndex + 1}
            pageIndex={listIndex}
            scale={scale}
            onDimensionsKnown={handleDims}
          />
        </div>
      ))}
    </div>
  );
};
