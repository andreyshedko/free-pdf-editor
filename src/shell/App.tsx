import React from 'react';
import { PdfWorkspace } from '@modules/viewer/PdfWorkspace';
import { usePdfDocumentStore } from '@store/pdfDocumentStore';
import { SystemStatus } from '@core/SystemStatus';

export const App: React.FC = () => {
  const setDocument = usePdfDocumentStore((s) => s.setDocument);
  const zoom = usePdfDocumentStore((s) => s.zoom);
  const setZoom = usePdfDocumentStore((s) => s.setZoom);
  const setViewMode = usePdfDocumentStore((s) => s.setViewMode);

  const handleOpenPdf = async () => {
    const opened = await window.electronAPI?.openPdf?.();
    if (opened) setDocument({ data: opened.data, name: opened.name });
  };

  return (
    <div className="app-shell" aria-label="Free PDF Editor workspace">
      <header className="top-bar">
        <div className="top-bar-left">
          <div className="logo-mark" aria-hidden="true" />
          <span className="logo-text">Free PDF Editor</span>
          <span className="pill">Desktop · Offline · Secure</span>
        </div>
        <div className="top-bar-right">
          <SystemStatus />
          <span className="badge-subtle" aria-label="AI tools status">
            <span className="badge-dot" />
            AI tools ready
          </span>
        </div>
      </header>

      <nav className="toolbar" aria-label="Main PDF toolbar">
        <div className="toolbar-group" aria-label="File actions">
          <button className="btn btn-strong" onClick={handleOpenPdf}>
            <span className="btn-icon">＋</span>
            Open PDF
            <span className="btn-key">Ctrl/Cmd + O</span>
          </button>
        </div>

        <div className="toolbar-group" aria-label="View controls">
          <button className="btn btn-ghost" onClick={() => setViewMode('fit-width')} title="Fit width">
            <span className="btn-icon">↔</span>
            Fit width
          </button>
          <button className="btn btn-ghost" onClick={() => setViewMode('fit-page')} title="Fit page">
            <span className="btn-icon">⤢</span>
            Fit page
          </button>
          <div className="scale-slider" aria-label="Zoom">
            <span className="muted">Zoom</span>
            <input
              type="range"
              min={25}
              max={200}
              value={zoom}
              onChange={(e) => setZoom(Number(e.target.value))}
            />
            <span className="muted">{Math.round(zoom)}%</span>
          </div>
        </div>

        <div className="toolbar-group" aria-label="Annotation tools">
          <button className="btn btn-ghost">
            <span className="btn-icon">✏️</span>
            Draw
          </button>
          <button className="btn btn-ghost">
            <span className="btn-icon">🖍</span>
            Highlight
          </button>
          <button className="btn btn-ghost">
            <span className="btn-icon">✉</span>
            Comment
          </button>
        </div>

        <div className="toolbar-group" aria-label="Export and security">
          <button className="btn btn-ghost">
            <span className="btn-icon">⬇</span>
            Export
          </button>
          <button className="btn btn-ghost">
            <span className="btn-icon">🔒</span>
            Protect
          </button>
        </div>
      </nav>

      <PdfWorkspace />
    </div>
  );
};

