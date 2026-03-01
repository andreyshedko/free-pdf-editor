import React from 'react';
import { usePdfDocumentStore } from '@store/pdfDocumentStore';
import { PdfThumbnail } from './PdfThumbnail';

export const PdfThumbnailsPane: React.FC = () => {
  const { fileData, pageCount, currentPageIndex, setCurrentPageIndex } = usePdfDocumentStore();

  if (!fileData || !pageCount) {
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
        <PdfThumbnail
          key={i}
          fileData={fileData}
          pageNumber={i + 1}
          isActive={i === currentPageIndex}
          onClick={() => setCurrentPageIndex(i)}
        />
      ))}
    </div>
  );
};
