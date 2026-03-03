import React, { useEffect, useRef } from 'react';
import { getOrLoadPdfDocument } from '@core/pdf/pdfjsService';

const THUMB_SCALE = 0.2;

interface PdfThumbnailProps {
  fileData: ArrayBuffer;
  pageNumber: number;
  pageIndex: number;
  isActive: boolean;
  isDragOver: boolean;
  onClick: () => void;
  onDragStart: (index: number) => void;
  onDragOver: (e: React.DragEvent, index: number) => void;
  onDrop: (index: number) => void;
  onDragEnd: () => void;
}

export const PdfThumbnail: React.FC<PdfThumbnailProps> = ({
  fileData,
  pageNumber,
  pageIndex,
  isActive,
  isDragOver,
  onClick,
  onDragStart,
  onDragOver,
  onDrop,
  onDragEnd,
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
    <div
      role="button"
      tabIndex={0}
      draggable
      className={`thumb-card ${isActive ? 'active' : ''} ${isDragOver ? 'drag-over' : ''}`}
      onClick={onClick}
      onKeyDown={(e) => { if (e.key === 'Enter' || e.key === ' ') onClick(); }}
      onDragStart={() => onDragStart(pageIndex)}
      onDragOver={(e) => { e.preventDefault(); onDragOver(e, pageIndex); }}
      onDrop={(e) => { e.preventDefault(); onDrop(pageIndex); }}
      onDragEnd={onDragEnd}
      aria-label={`Page ${pageNumber}`}
    >
      <div className="thumb-number">Page {pageNumber}</div>
      <div className="thumb-preview">
        <canvas ref={canvasRef} />
      </div>
    </div>
  );
};
