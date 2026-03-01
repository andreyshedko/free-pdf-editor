import React from 'react';
import { usePdfDocumentStore } from '@store/pdfDocumentStore';

export const PdfThumbnailsPane: React.FC = () => {
  const { pageCount, currentPageIndex } = usePdfDocumentStore();

  if (!pageCount) {
    return (
      <div className="thumbs-scroll empty-state">
        <div className="empty-orbit" aria-hidden="true">
          <div className="empty-orbit-inner">
            <div className="empty-orbit-glow" />
            <div className="empty-orbit-badge">Drag & drop coming soon</div>
          </div>
        </div>
        <div className="empty-caption">
          <strong>Start by opening a PDF.</strong> Thumbnails, page reordering, and page-level actions will appear
          here.
        </div>
      </div>
    );
  }

  return (
    <div className="thumbs-scroll">
      {Array.from({ length: pageCount }, (_, i) => (
        <button key={i} className={`thumb-card ${i === currentPageIndex ? 'active' : ''}`}>
          <div className="thumb-number">Page {i + 1}</div>
          <div className="thumb-preview">
            <span className="muted">Preview</span>
          </div>
        </button>
      ))}
    </div>
  );
};

