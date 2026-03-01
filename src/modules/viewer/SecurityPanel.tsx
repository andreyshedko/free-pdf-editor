import React, { useState } from 'react';
import { exportPdf } from '@core/pdf/exportService';
import { usePdfDocumentStore } from '@store/pdfDocumentStore';

interface SecurityPanelProps {
  onClose: () => void;
}

export const SecurityPanel: React.FC<SecurityPanelProps> = ({ onClose }) => {
  const [password, setPassword] = useState('');
  const [status, setStatus] = useState('');
  const { fileData, currentFileName } = usePdfDocumentStore();

  const handleProtect = async () => {
    if (!fileData) return;
    setStatus('Saving...');
    try {
      const bytes = await exportPdf(fileData, password || undefined);
      const blob = new Blob([bytes], { type: 'application/pdf' });
      const url = URL.createObjectURL(blob);
      const a = document.createElement('a');
      a.href = url;
      a.download = currentFileName ? `protected_${currentFileName}` : 'protected.pdf';
      a.click();
      URL.revokeObjectURL(url);
      setStatus('Downloaded!');
    } catch (e) {
      setStatus('Error: ' + (e instanceof Error ? e.message : String(e)));
    }
  };

  return (
    <div className="signature-panel-overlay">
      <div className="signature-panel">
        <div className="signature-panel-header">
          <span>🔒 Protect PDF</span>
          <button className="btn btn-ghost" onClick={onClose}>✕</button>
        </div>
        <div style={{ padding: '1rem', display: 'flex', flexDirection: 'column', gap: '0.75rem' }}>
          <label style={{ fontSize: '0.8rem', color: 'var(--text-muted)' }}>Password (leave empty to save without encryption)</label>
          <input
            type="password"
            className="text-input"
            placeholder="Enter password..."
            value={password}
            onChange={(e) => setPassword(e.target.value)}
          />
          {status && <span style={{ fontSize: '0.75rem', color: status.startsWith('Error') ? 'var(--danger)' : '#22c55e' }}>{status}</span>}
        </div>
        <div className="signature-panel-actions">
          <button className="btn btn-ghost" onClick={onClose}>Cancel</button>
          <button className="btn btn-strong" onClick={handleProtect}>Save PDF</button>
        </div>
      </div>
    </div>
  );
};
