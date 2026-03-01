import React, { useEffect, useRef, useCallback } from 'react';
import * as fabric from 'fabric';
import { useAnnotationStore } from '@store/annotationStore';

interface AnnotationOverlayProps {
  pageIndex: number;
  width: number;
  height: number;
}

const HIGHLIGHT_WIDTH_MULTIPLIER = 8;

export const AnnotationOverlay: React.FC<AnnotationOverlayProps> = ({ pageIndex, width, height }) => {
  const canvasRef = useRef<HTMLCanvasElement>(null);
  const fabricRef = useRef<fabric.Canvas | null>(null);
  const { activeTool, currentColor, strokeWidth, annotationsByPage, setAnnotationsForPage } = useAnnotationStore();

  // Initialize fabric canvas
  useEffect(() => {
    const canvasEl = canvasRef.current;
    if (!canvasEl) return;

    const fc = new fabric.Canvas(canvasEl, {
      width,
      height,
      isDrawingMode: false,
      selection: true,
    });

    fabricRef.current = fc;

    // Restore saved annotations
    const saved = annotationsByPage[pageIndex];
    if (saved && saved.length > 0) {
      const lastAnn = saved[saved.length - 1];
      try {
        fc.loadFromJSON(JSON.parse(lastAnn.fabricJSON), () => fc.renderAll());
      } catch (err) {
        console.error('Failed to restore annotations for page', pageIndex, err);
      }
    }

    return () => {
      fc.dispose();
      fabricRef.current = null;
    };
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [pageIndex, width, height]);

  // Save canvas state on object modifications
  useEffect(() => {
    const fc = fabricRef.current;
    if (!fc) return;

    const save = () => {
      const json = JSON.stringify(fc.toJSON());
      setAnnotationsForPage(pageIndex, [{
        id: `page-${pageIndex}`,
        pageIndex,
        type: activeTool,
        fabricJSON: json,
      }]);
    };

    fc.on('object:added', save);
    fc.on('object:modified', save);
    fc.on('object:removed', save);
    return () => {
      fc.off('object:added', save);
      fc.off('object:modified', save);
      fc.off('object:removed', save);
    };
  }, [pageIndex, activeTool, setAnnotationsForPage]);

  // Update drawing mode based on active tool
  useEffect(() => {
    const fc = fabricRef.current;
    if (!fc) return;

    const brush = new fabric.PencilBrush(fc);

    switch (activeTool) {
      case 'draw':
        fc.isDrawingMode = true;
        brush.color = currentColor;
        brush.width = strokeWidth;
        fc.freeDrawingBrush = brush;
        break;
      case 'highlight': {
        fc.isDrawingMode = true;
        const hBrush = new fabric.PencilBrush(fc);
        hBrush.color = currentColor;
        hBrush.width = strokeWidth * HIGHLIGHT_WIDTH_MULTIPLIER;
        fc.freeDrawingBrush = hBrush;
        break;
      }
      default:
        fc.isDrawingMode = false;
        break;
    }
  }, [activeTool, currentColor, strokeWidth]);

  // Add rect / ellipse / text on click
  const handleCanvasClick = useCallback(
    (e: React.MouseEvent<HTMLCanvasElement>) => {
      const fc = fabricRef.current;
      if (!fc) return;
      if (activeTool !== 'rect' && activeTool !== 'ellipse' && activeTool !== 'text') return;

      const rect = canvasRef.current!.getBoundingClientRect();
      const x = e.clientX - rect.left;
      const y = e.clientY - rect.top;

      if (activeTool === 'rect') {
        const shape = new fabric.Rect({
          left: x - 40,
          top: y - 25,
          width: 80,
          height: 50,
          fill: 'transparent',
          stroke: currentColor,
          strokeWidth,
        });
        fc.add(shape);
      } else if (activeTool === 'ellipse') {
        const shape = new fabric.Ellipse({
          left: x - 40,
          top: y - 25,
          rx: 40,
          ry: 25,
          fill: 'transparent',
          stroke: currentColor,
          strokeWidth,
        });
        fc.add(shape);
      } else if (activeTool === 'text') {
        const text = new fabric.IText('Text', {
          left: x,
          top: y,
          fill: currentColor,
          fontSize: 16,
        });
        fc.add(text);
        fc.setActiveObject(text);
        fc.renderAll();
      }
    },
    [activeTool, currentColor, strokeWidth]
  );

  return (
    <canvas
      ref={canvasRef}
      onClick={handleCanvasClick}
      className="page-annotation-layer"
      style={{ width, height }}
    />
  );
};
