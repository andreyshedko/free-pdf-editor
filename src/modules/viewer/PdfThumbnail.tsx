import React, { useEffect, useRef } from 'react';
import { getOrLoadPdfDocument } from '@core/pdf/pdfjsService';

const THUMB_SCALE = 0.2;

interface PdfThumbnailProps {
  fileData: ArrayBuffer;
  pageNumber: number;
  isActive: boolean;
  onClick: () => void;
}

export const PdfThumbnail: React.FC<PdfThumbnailProps> = ({
  fileData,
  pageNumber,
  isActive,
  onClick
}) => {
  const canvasRef = useRef<HTMLCanvasElement>(null);

  useEffect(() => {
    let cancelled = false;

    const render = async () => {
      try {
        const pdf = await getOrLoadPdfDocument(fileData);
        if (cancelled) return;

        const page = await pdf.getPage(pageNumber);
        if (cancelled) return;

        const viewport = page.getViewport({ scale: THUMB_SCALE });
        const canvas = canvasRef.current;
        if (!canvas) return;

        const ctx = canvas.getContext('2d');
        if (!ctx) return;

        canvas.width = viewport.width;
        canvas.height = viewport.height;

        await page.render({
          canvasContext: ctx,
          viewport
        } as Parameters<typeof page.render>[0]).promise;
      } catch {
        // ignore
      }
    };

    void render();
    return () => {
      cancelled = true;
    };
  }, [fileData, pageNumber]);

  return (
    <button
      type="button"
      className={`thumb-card ${isActive ? 'active' : ''}`}
      onClick={onClick}
      aria-label={`Page ${pageNumber}`}
    >
      <div className="thumb-number">Page {pageNumber}</div>
      <div className="thumb-preview">
        <canvas ref={canvasRef} />
      </div>
    </button>
  );
};
