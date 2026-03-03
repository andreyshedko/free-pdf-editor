import React from 'react';
import { usePdfDocumentStore } from '@store/pdfDocumentStore';
import { PdfThumbnailsPane } from './PdfThumbnailsPane';
import { PdfPageViewer } from './PdfPageViewer';
import { RightInspectorPane } from './RightInspectorPane';

export const PdfWorkspace = () => {
  const { currentFileName } = usePdfDocumentStore();

  return (
    <main className="content">
      <section className="panel" aria-label="Page thumbnails">
        <div className="panel-header">
          <div className="panel-header-title">
            <span aria-hidden="true" />
            <span>Pages</span>
          </div>
          <span className="muted">Thumbnails</span>
        </div>
        <PdfThumbnailsPane />
      </section>

      <section className="panel viewer-shell" aria-label="PDF viewer">
        <div className="panel-header">
          <div className="panel-header-title">
            <span aria-hidden="true" />
            <span>{currentFileName ?? 'No document open'}</span>
          </div>
          <span className="status-chip">
            <span className="status-dot" />
            Live editing sandbox
          </span>
        </div>
        <PdfPageViewer />
      </section>

      <section className="panel" aria-label="Document inspector and AI tools">
        <div className="panel-header">
          <div className="panel-header-title">
            <span aria-hidden="true" />
            <span>Insights & tools</span>
          </div>
          <span className="muted">Context · AI · Security</span>
        </div>
        <RightInspectorPane />
      </section>
    </main>
  );
};
