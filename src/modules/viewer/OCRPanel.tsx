import React, { useState } from 'react';
import { recognizeCanvas } from '@core/pdf/ocrService';
import { usePdfDocumentStore } from '@store/pdfDocumentStore';
import { getOrLoadPdfDocument } from '@core/pdf/pdfjsService';

interface OCRPanelProps {
  onClose: () => void;
}

export const OCRPanel: React.FC<OCRPanelProps> = ({ onClose }) => {
  const [text, setText] = useState('');
  const [loading, setLoading] = useState(false);
  const { fileData, currentPageIndex } = usePdfDocumentStore();

  const runOCR = async () => {
    if (!fileData) return;
    setLoading(true);
    setText('');
    try {
      const pdf = await getOrLoadPdfDocument(fileData);
      const page = await pdf.getPage(currentPageIndex + 1);
      const viewport = page.getViewport({ scale: 2 });
      const canvas = document.createElement('canvas');
      canvas.width = viewport.width;
      canvas.height = viewport.height;
      const ctx = canvas.getContext('2d');
      if (!ctx) return;
      await page.render({ canvasContext: ctx, viewport } as any).promise;
      const result = await recognizeCanvas(canvas);
      setText(result);
    } catch (e) {
      setText('Error: ' + (e instanceof Error ? e.message : String(e)));
    } finally {
      setLoading(false);
    }
  };

  return (
    <div className="modal-overlay">
      <div className="modal-panel" style={{ width: 500, maxHeight: '80vh' }}>
        <div className="modal-header">
          <span>🔍 OCR – Extract Text</span>
          <button className="btn btn-ghost" onClick={onClose}>✕</button>
        </div>
        <div style={{ padding: '1rem', display: 'flex', flexDirection: 'column', gap: '0.75rem', overflowY: 'auto', maxHeight: '60vh' }}>
          <p style={{ fontSize: '0.8rem', color: 'var(--text-muted)', margin: 0 }}>
            Extract text from the current page using OCR (runs locally, no internet required).
          </p>
          {loading && <span style={{ color: '#22c55e', fontSize: '0.8rem' }}>Running OCR…</span>}
          {text && (
            <textarea
              readOnly
              value={text}
              style={{ width: '100%', height: 300, resize: 'vertical', background: 'rgba(15,23,42,0.9)', color: 'var(--text)', border: '1px solid rgba(55,65,81,0.9)', borderRadius: '0.5rem', padding: '0.5rem', fontSize: '0.8rem' }}
            />
          )}
        </div>
        <div className="modal-actions">
          <button className="btn btn-ghost" onClick={onClose}>Close</button>
          <button
            className="btn btn-strong"
            onClick={runOCR}
            disabled={loading || !fileData || currentPageIndex < 0}
          >
            {loading ? 'Running…' : 'Run OCR on current page'}
          </button>
        </div>
      </div>
    </div>
  );
};
