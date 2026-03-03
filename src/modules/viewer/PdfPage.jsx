import React, { useEffect, useRef, useState } from 'react';
import { getOrLoadPdfDocument } from '@core/pdf/pdfjsService';

export const PdfPage = ({ fileData, pageNumber, scale }) => {
  const canvasRef = useRef(null);
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

        const renderContext = {
          canvasContext: ctx,
          viewport
        };

        await page.render(renderContext).promise;
      } catch (e) {
        if (!cancelled) setError(e instanceof Error ? e.message : 'Failed to render');
      }
    };

    void render();
    return () => {
      cancelled = true;
    };
  }, [fileData, pageNumber, scale]);

  if (error) {
    return (
      <div className="page-wrapper page-error">
        <span className="muted">Page {pageNumber}: {error}</span>
      </div>
    );
  }

  return (
    <div className="page-wrapper">
      <canvas ref={canvasRef} className="page-canvas" />
    </div>
  );
};
