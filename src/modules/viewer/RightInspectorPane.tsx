import React from 'react';
import { usePdfDocumentStore } from '@store/pdfDocumentStore';
import { usePageManagementStore } from '@store/pageManagementStore';
import { useAnnotationStore } from '@store/annotationStore';

export const RightInspectorPane: React.FC = () => {
  const { pageCount, currentPageIndex, currentFileName, setCurrentPageIndex } = usePdfDocumentStore();
  const { pageOrder, deletePage } = usePageManagementStore();
  const { activeTool } = useAnnotationStore();

  const handleDeleteCurrentPage = () => {
    const newLength = pageOrder.length - 1;
    deletePage(currentPageIndex);
    // Clamp currentPageIndex so it stays within the new page list bounds
    if (newLength > 0 && currentPageIndex >= newLength) {
      setCurrentPageIndex(newLength - 1);
    }
  };

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
            Total: <strong>{pageOrder.length || pageCount || 0}</strong>
          </div>
          {(pageCount || pageOrder.length > 0) && currentPageIndex >= 0 && (
            <div className="right-pill">
              Viewing: <strong>{currentPageIndex + 1}</strong>
            </div>
          )}
        </div>
        {pageOrder.length > 0 && currentPageIndex >= 0 && (
          <div style={{ marginTop: '0.4rem' }}>
            <button
              className="btn btn-ghost"
              style={{ fontSize: '0.7rem', color: 'var(--danger)' }}
              onClick={handleDeleteCurrentPage}
            >
              🗑 Delete current page
            </button>
          </div>
        )}
      </section>

      <section className="right-panel-section">
        <div className="right-panel-label">Active Tool</div>
        <div className="right-pill-row">
          <div className="right-pill">
            {activeTool ?? 'None (select mode)'}
          </div>
        </div>
      </section>

      <section className="right-panel-section">
        <div className="right-panel-label">Shortcuts</div>
        <div className="right-pill-row">
          <span className="kbd">Ctrl/Cmd+O</span>
          <span className="kbd">Ctrl/Cmd+S</span>
          <span className="kbd">Ctrl/Cmd+=</span>
          <span className="kbd">Ctrl/Cmd+-</span>
          <span className="kbd">Esc</span>
        </div>
      </section>
    </div>
  );
};

