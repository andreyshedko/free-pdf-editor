import React, { useState } from 'react';
import { usePdfDocumentStore } from '@store/pdfDocumentStore';
import { usePageManagementStore } from '@store/pageManagementStore';
import { PdfThumbnail } from './PdfThumbnail';

export const PdfThumbnailsPane: React.FC = () => {
  const { fileData, pageCount, currentPageIndex, setCurrentPageIndex } = usePdfDocumentStore();
  const { pageOrder, reorderPage } = usePageManagementStore();
  const [dragFromIndex, setDragFromIndex] = useState<number | null>(null);
  const [dragOverIndex, setDragOverIndex] = useState<number | null>(null);

  if (!fileData || !pageCount) {
    return (
      <div className="thumbs-scroll empty-state">
        <div className="empty-orbit" aria-hidden="true">
          <div className="empty-orbit-inner">
            <div className="empty-orbit-glow" />
            <div className="empty-orbit-badge">Drag to reorder</div>
          </div>
        </div>
        <div className="empty-caption">
          <strong>Start by opening a PDF.</strong> Thumbnails, page reordering, and page-level actions will appear
          here.
        </div>
      </div>
    );
  }

  const pages =
    pageOrder.length > 0 ? pageOrder : Array.from({ length: pageCount }, (_, i) => i);

  const handleDragStart = (index: number) => setDragFromIndex(index);
  const handleDragOver = (_e: React.DragEvent, index: number) => setDragOverIndex(index);
  const handleDrop = (toIndex: number) => {
    if (dragFromIndex !== null && dragFromIndex !== toIndex) {
      reorderPage(dragFromIndex, toIndex);
    }
    setDragOverIndex(null);
  };
  const handleDragEnd = () => {
    setDragFromIndex(null);
    setDragOverIndex(null);
  };

  return (
    <div className="thumbs-scroll">
      {pages.map((origIndex, listIndex) => (
        <PdfThumbnail
          key={`${origIndex}-${listIndex}`}
          fileData={fileData}
          pageNumber={origIndex + 1}
          pageIndex={listIndex}
          isActive={listIndex === currentPageIndex}
          isDragOver={dragOverIndex === listIndex}
          onClick={() => setCurrentPageIndex(listIndex)}
          onDragStart={handleDragStart}
          onDragOver={handleDragOver}
          onDrop={handleDrop}
          onDragEnd={handleDragEnd}
        />
      ))}
    </div>
  );
};
