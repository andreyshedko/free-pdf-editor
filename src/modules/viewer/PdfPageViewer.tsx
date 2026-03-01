import React, { useEffect, useRef } from 'react';
import { usePdfDocumentStore } from '@store/pdfDocumentStore';
import { getOrLoadPdfDocument } from '@core/pdf/pdfjsService';

export const PdfPageViewer: React.FC = () => {
  const { fileData, currentPageIndex, pageCount, setPageCount, setDocument } = usePdfDocumentStore();
  const containerRef = useRef<HTMLDivElement | null>(null);

  useEffect(() => {
    if (!fileData || currentPageIndex < 0) {
      if (containerRef.current) {
        containerRef.current.innerHTML = '';
      }
      return;
    }

    let cancelled = false;

    const render = async () => {
      const container = containerRef.current;
      if (!container) return;

      container.innerHTML = '';

      const pdf = await getOrLoadPdfDocument(fileData);
      if (cancelled) return;

      if (!pageCount) {
        setPageCount(pdf.numPages);
      }

      const page = await pdf.getPage(currentPageIndex + 1);
      if (cancelled) return;

      const viewport = page.getViewport({ scale: 1.2 });
      const wrapper = document.createElement('div');
      wrapper.className = 'page-wrapper';

      const canvas = document.createElement('canvas');
      canvas.className = 'page-canvas';
      const context = canvas.getContext('2d');
      if (!context) return;

      const outputScale = window.devicePixelRatio || 1;
      canvas.width = viewport.width * outputScale;
      canvas.height = viewport.height * outputScale;
      canvas.style.width = `${viewport.width}px`;
      canvas.style.height = `${viewport.height}px`;

      const renderContext = {
        canvasContext: context,
        viewport
      } as any;

      wrapper.appendChild(canvas);
      container.appendChild(wrapper);

      await page.render(renderContext).promise;
    };

    void render();

    return () => {
      cancelled = true;
    };
  }, [fileData, currentPageIndex, pageCount, setPageCount]);

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
          <button className="btn btn-ghost">
            <span className="btn-icon">💡</span>
            See roadmap
          </button>
        </div>
      </div>
    );
  }

  return <div ref={containerRef} className="viewer-scroll" aria-label="Current PDF page" />;
};

