import React from 'react';
import { usePdfDocumentStore } from '@store/pdfDocumentStore';

export const RightInspectorPane: React.FC = () => {
  const { pageCount, currentPageIndex, currentFileName } = usePdfDocumentStore();

  return (
    <div className="right-panel-scroll">
      <section className="right-panel-section">
        <div className="right-panel-label">Document</div>
        {currentFileName ? (
          <div className="file-pill">
            <span aria-hidden="true" />
            <strong>{currentFileName}</strong>
          </div>
        ) : (
          <p className="muted">No document open.</p>
        )}
      </section>

      <section className="right-panel-section">
        <div className="right-panel-label">Pages</div>
        <div className="right-pill-row">
          <div className="right-pill">
            Total: <strong>{pageCount ?? 0}</strong>
          </div>
          {pageCount && currentPageIndex >= 0 && (
            <div className="right-pill">
              Viewing: <strong>{currentPageIndex + 1}</strong>
            </div>
          )}
        </div>
      </section>

      <section className="right-panel-section">
        <div className="right-panel-label">Shortcuts</div>
        <div className="right-pill-row">
          <span className="kbd">Ctrl/Cmd + O</span>
          <span className="kbd">Ctrl/Cmd + S</span>
          <span className="kbd">Ctrl/Cmd + Z</span>
        </div>
      </section>

      <section className="right-panel-section">
        <div className="right-panel-label">AI tools (optional)</div>
        <p className="muted">
          A future `AI` panel can live here for document summaries, redaction suggestions, and quick Q&amp;A on the
          current PDF.
        </p>
      </section>
    </div>
  );
};

