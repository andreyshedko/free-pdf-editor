import React, { useState } from 'react';
import { usePdfDocumentStore } from '@store/pdfDocumentStore';
import { usePageManagementStore } from '@store/pageManagementStore';
import { insertText } from './editorService';

interface EditorPanelProps {
  onClose: () => void;
}

export const EditorPanel: React.FC<EditorPanelProps> = ({ onClose }) => {
  const { fileData, currentPageIndex, updateFileData } = usePdfDocumentStore();
  const { pageOrder } = usePageManagementStore();
  const [text, setText] = useState('');
  const [x, setX] = useState(50);
  const [y, setY] = useState(50);
  const [fontSize, setFontSize] = useState(14);
  const [color, setColor] = useState('#000000');
  const [status, setStatus] = useState('');

  const actualPageIndex =
    pageOrder.length > 0 ? (pageOrder[currentPageIndex] ?? currentPageIndex) : currentPageIndex;

  const handleInsert = async () => {
    if (!fileData || !text.trim()) return;
    setStatus('Inserting…');
    try {
      const newBytes = await insertText(fileData, {
        text: text.trim(),
        pageIndex: actualPageIndex,
        x,
        y,
        fontSize,
        color,
      });
      updateFileData(
        newBytes.buffer.slice(
          newBytes.byteOffset,
          newBytes.byteOffset + newBytes.byteLength,
        ) as ArrayBuffer,
      );
      setStatus('Text inserted!');
      setText('');
    } catch (e) {
      setStatus('Error: ' + (e instanceof Error ? e.message : String(e)));
    }
  };

  return (
    <div className="modal-overlay">
      <div className="modal-panel">
        <div className="modal-header">
          <span>✏️ Insert Text</span>
          <button className="btn btn-ghost" onClick={onClose}>✕</button>
        </div>
        <div style={{ padding: '1rem', display: 'flex', flexDirection: 'column', gap: '0.75rem' }}>
          <textarea
            className="text-input"
            placeholder="Enter text to insert…"
            value={text}
            onChange={(e) => setText(e.target.value)}
            rows={3}
            style={{ resize: 'vertical', fontFamily: 'inherit' }}
          />
          <div style={{ display: 'grid', gridTemplateColumns: '1fr 1fr', gap: '0.5rem' }}>
            <label style={{ fontSize: '0.8rem', color: 'var(--text-muted)', display: 'flex', flexDirection: 'column', gap: '0.25rem' }}>
              X (px from left)
              <input type="number" className="text-input" value={x} min={0} onChange={(e) => setX(Number(e.target.value))} />
            </label>
            <label style={{ fontSize: '0.8rem', color: 'var(--text-muted)', display: 'flex', flexDirection: 'column', gap: '0.25rem' }}>
              Y (px from top)
              <input type="number" className="text-input" value={y} min={0} onChange={(e) => setY(Number(e.target.value))} />
            </label>
          </div>
          <div style={{ display: 'grid', gridTemplateColumns: '1fr 1fr', gap: '0.5rem' }}>
            <label style={{ fontSize: '0.8rem', color: 'var(--text-muted)', display: 'flex', flexDirection: 'column', gap: '0.25rem' }}>
              Font size (pt)
              <input type="number" className="text-input" value={fontSize} min={6} max={72} onChange={(e) => setFontSize(Math.max(6, Number(e.target.value)))} />
            </label>
            <label style={{ fontSize: '0.8rem', color: 'var(--text-muted)', display: 'flex', alignItems: 'center', gap: '0.5rem' }}>
              Color
              <input
                type="color"
                value={color}
                onChange={(e) => setColor(e.target.value)}
                style={{ width: 32, height: 32, border: 'none', background: 'transparent', cursor: 'pointer', borderRadius: 4 }}
              />
            </label>
          </div>
          <p style={{ fontSize: '0.75rem', color: 'var(--text-muted)', margin: 0 }}>
            Inserting on page {actualPageIndex + 1}. Coordinates measured from the top-left corner.
          </p>
          {status && (
            <span style={{ fontSize: '0.75rem', color: status.startsWith('Error') ? 'var(--danger)' : '#22c55e' }}>
              {status}
            </span>
          )}
        </div>
        <div className="modal-actions">
          <button className="btn btn-ghost" onClick={onClose}>Cancel</button>
          <button className="btn btn-strong" onClick={handleInsert} disabled={!text.trim() || !fileData}>
            Insert Text
          </button>
        </div>
      </div>
    </div>
  );
};
