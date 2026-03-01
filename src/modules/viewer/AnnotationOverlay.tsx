import React, { useEffect, useRef } from 'react';
import * as fabric from 'fabric';
import { useAnnotationStore } from '@store/annotationStore';

interface AnnotationOverlayProps {
  pageIndex: number;
  width: number;
  height: number;
}

const HIGHLIGHT_WIDTH_MULTIPLIER = 8;

export const AnnotationOverlay: React.FC<AnnotationOverlayProps> = ({ pageIndex, width, height }) => {
  const containerRef = useRef<HTMLDivElement>(null);
  const canvasRef = useRef<HTMLCanvasElement>(null);
  const fabricRef = useRef<fabric.Canvas | null>(null);
  const { activeTool, currentColor, strokeWidth, annotationsByPage, setAnnotationsForPage } = useAnnotationStore();

  // Keep a ref that always reflects the latest activeTool/color/strokeWidth so event
  // handlers that close over it capture the value at the time of the event, not at
  // the time the listener was attached.
  const activeToolRef = useRef(activeTool);
  activeToolRef.current = activeTool;
  const currentColorRef = useRef(currentColor);
  currentColorRef.current = currentColor;
  const strokeWidthRef = useRef(strokeWidth);
  strokeWidthRef.current = strokeWidth;

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

    // Restore saved annotations – pass the JSON string directly (not pre-parsed)
    const saved = annotationsByPage[pageIndex];
    if (saved && saved.length > 0) {
      const lastAnn = saved[saved.length - 1];
      try {
        fc.loadFromJSON(lastAnn.fabricJSON, () => fc.renderAll());
      } catch (err) {
        console.error('Failed to restore annotations for page', pageIndex, err);
      }
    }

    // Save canvas state whenever objects change; use activeToolRef to capture the
    // current tool at the time of the event rather than the stale closure value.
    const save = () => {
      const json = JSON.stringify(fc.toJSON());
      setAnnotationsForPage(pageIndex, [{
        id: `page-${pageIndex}`,
        pageIndex,
        type: activeToolRef.current,
        fabricJSON: json,
      }]);
    };
    fc.on('object:added', save);
    fc.on('object:modified', save);
    fc.on('object:removed', save);

    // Handle shape placement (rect / ellipse / text) via Fabric's mouse:down so that
    // coordinates are resolved by Fabric's own pointer transformation (which accounts
    // for the upper-canvas offset) rather than raw React mouse event coordinates.
    const handleMouseDown = (opt: fabric.TPointerEventInfo<MouseEvent>) => {
      const tool = activeToolRef.current;
      if (tool !== 'rect' && tool !== 'ellipse' && tool !== 'text') return;
      const pointer = fc.getPointer(opt.e);
      const color = currentColorRef.current;
      const sw = strokeWidthRef.current;

      if (tool === 'rect') {
        const shape = new fabric.Rect({
          left: pointer.x - 40,
          top: pointer.y - 25,
          width: 80,
          height: 50,
          fill: 'transparent',
          stroke: color,
          strokeWidth: sw,
        });
        fc.add(shape);
      } else if (tool === 'ellipse') {
        const shape = new fabric.Ellipse({
          left: pointer.x - 40,
          top: pointer.y - 25,
          rx: 40,
          ry: 25,
          fill: 'transparent',
          stroke: color,
          strokeWidth: sw,
        });
        fc.add(shape);
      } else if (tool === 'text') {
        const text = new fabric.IText('Text', {
          left: pointer.x,
          top: pointer.y,
          fill: color,
          fontSize: 16,
        });
        fc.add(text);
        fc.setActiveObject(text);
        fc.renderAll();
      }
    };
    fc.on('mouse:down', handleMouseDown);

    return () => {
      fc.off('object:added', save);
      fc.off('object:modified', save);
      fc.off('object:removed', save);
      fc.off('mouse:down', handleMouseDown);
      fc.dispose();
      fabricRef.current = null;
    };
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [pageIndex, width, height]);

  // Update drawing mode based on active tool
  useEffect(() => {
    const fc = fabricRef.current;
    if (!fc) return;

    switch (activeTool) {
      case 'draw': {
        const brush = new fabric.PencilBrush(fc);
        brush.color = currentColor;
        brush.width = strokeWidth;
        fc.freeDrawingBrush = brush;
        fc.isDrawingMode = true;
        break;
      }
      case 'highlight': {
        const hBrush = new fabric.PencilBrush(fc);
        hBrush.color = currentColor;
        hBrush.width = strokeWidth * HIGHLIGHT_WIDTH_MULTIPLIER;
        fc.freeDrawingBrush = hBrush;
        fc.isDrawingMode = true;
        break;
      }
      default:
        fc.isDrawingMode = false;
        break;
    }
  }, [activeTool, currentColor, strokeWidth]);

  return (
    <div ref={containerRef} className="page-annotation-layer">
      <canvas ref={canvasRef} />
    </div>
  );
};
