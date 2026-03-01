import React from 'react';
import { PdfWorkspace } from '@modules/viewer/PdfWorkspace';
import { usePdfDocumentStore } from '@store/pdfDocumentStore';

export const App: React.FC = () => {
  const setDocument = usePdfDocumentStore((s) => s.setDocument);

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
          <button className="btn btn-ghost">
            <span className="btn-icon">↔</span>
            Fit width
          </button>
          <button className="btn btn-ghost">
            <span className="btn-icon">⤢</span>
            Fit page
          </button>
          <div className="scale-slider" aria-label="Zoom">
            <span className="muted">Zoom</span>
            <input type="range" min={25} max={200} defaultValue={100} />
            <span className="muted">100%</span>
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

