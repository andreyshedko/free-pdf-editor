import React, { useRef, useState, useEffect } from 'react';

interface SignaturePanelProps {
  onClose: () => void;
  onConfirm: (dataUrl: string) => void;
}

export const SignaturePanel: React.FC<SignaturePanelProps> = ({ onClose, onConfirm }) => {
  const canvasRef = useRef<HTMLCanvasElement>(null);
  const [isDrawing, setIsDrawing] = useState(false);

  // The canvas logical size in CSS pixels
  const CSS_WIDTH = 400;
  const CSS_HEIGHT = 200;

  useEffect(() => {
    const canvas = canvasRef.current;
    if (!canvas) return;
    const dpr = window.devicePixelRatio || 1;
    // Set the backing store to device pixels so strokes are sharp on HiDPI screens
    canvas.width = CSS_WIDTH * dpr;
    canvas.height = CSS_HEIGHT * dpr;
    canvas.style.width = `${CSS_WIDTH}px`;
    canvas.style.height = `${CSS_HEIGHT}px`;
    const ctx = canvas.getContext('2d');
    if (!ctx) return;
    ctx.scale(dpr, dpr);
    ctx.fillStyle = '#ffffff';
    ctx.fillRect(0, 0, CSS_WIDTH, CSS_HEIGHT);
    ctx.strokeStyle = '#1e3a8a';
    ctx.lineWidth = 2;
    ctx.lineCap = 'round';
  }, []);

  // Convert a mouse event to canvas-local logical coordinates accounting for DPR
  const getPos = (e: React.MouseEvent, canvas: HTMLCanvasElement) => {
    const r = canvas.getBoundingClientRect();
    const scaleX = CSS_WIDTH / r.width;
    const scaleY = CSS_HEIGHT / r.height;
    return {
      x: (e.clientX - r.left) * scaleX,
      y: (e.clientY - r.top) * scaleY,
    };
  };

  const startDraw = (e: React.MouseEvent) => {
    const canvas = canvasRef.current;
    if (!canvas) return;
    const ctx = canvas.getContext('2d');
    if (!ctx) return;
    setIsDrawing(true);
    const { x, y } = getPos(e, canvas);
    ctx.beginPath();
    ctx.moveTo(x, y);
  };

  const draw = (e: React.MouseEvent) => {
    if (!isDrawing) return;
    const canvas = canvasRef.current;
    if (!canvas) return;
    const ctx = canvas.getContext('2d');
    if (!ctx) return;
    const { x, y } = getPos(e, canvas);
    ctx.lineTo(x, y);
    ctx.stroke();
  };

  const endDraw = () => setIsDrawing(false);

  const clear = () => {
    const canvas = canvasRef.current;
    if (!canvas) return;
    const ctx = canvas.getContext('2d');
    if (!ctx) return;
    ctx.fillStyle = '#ffffff';
    ctx.fillRect(0, 0, CSS_WIDTH, CSS_HEIGHT);
  };

  const confirm = () => {
    const canvas = canvasRef.current;
    if (!canvas) return;
    onConfirm(canvas.toDataURL('image/png'));
    onClose();
  };

  return (
    <div className="modal-overlay">
      <div className="modal-panel">
        <div className="modal-header">
          <span>Draw Signature</span>
          <button className="btn btn-ghost" onClick={onClose}>✕</button>
        </div>
        {/* Canvas dimensions are set programmatically in useEffect to account for
            the device pixel ratio (HiDPI/Retina). The CSS size is 400×200 px and
            the backing store is scaled to CSS_SIZE * dpr physical pixels. */}
        <canvas
          ref={canvasRef}
          className="signature-canvas"
          onMouseDown={startDraw}
          onMouseMove={draw}
          onMouseUp={endDraw}
          onMouseLeave={endDraw}
        />
        <div className="modal-actions">
          <button className="btn btn-ghost" onClick={clear}>Clear</button>
          <button className="btn btn-strong" onClick={confirm}>Add to PDF</button>
        </div>
      </div>
    </div>
  );
};
