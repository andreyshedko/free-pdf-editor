import React, { useCallback, useState } from 'react';
import { PdfWorkspace } from '@modules/viewer/PdfWorkspace';
import { usePdfDocumentStore } from '@store/pdfDocumentStore';
import { useAnnotationStore } from '@store/annotationStore';
import { usePageManagementStore } from '@store/pageManagementStore';
import { exportPdf, reorderPages } from '@core/pdf/exportService';
import { SignaturePanel } from '@modules/viewer/SignaturePanel';
import { SecurityPanel } from '@modules/viewer/SecurityPanel';
import { OCRPanel } from '@modules/viewer/OCRPanel';

export const App: React.FC = () => {
  const setDocument = usePdfDocumentStore((s) => s.setDocument);
  const zoom = usePdfDocumentStore((s) => s.zoom);
  const setZoom = usePdfDocumentStore((s) => s.setZoom);
  const setViewMode = usePdfDocumentStore((s) => s.setViewMode);
  const { fileData, currentFileName } = usePdfDocumentStore();
  const { activeTool, setTool, currentColor, setColor, strokeWidth, setStrokeWidth } = useAnnotationStore();
  const { pageOrder } = usePageManagementStore();

  const [showSignaturePanel, setShowSignaturePanel] = useState(false);
  const [showSecurityPanel, setShowSecurityPanel] = useState(false);
  const [showOCRPanel, setShowOCRPanel] = useState(false);

  const handleOpenPdf = useCallback(async () => {
    const opened = await window.electronAPI?.openPdf?.();
    if (opened) setDocument({ data: opened.data, name: opened.name });
  }, [setDocument]);

  const handleExport = useCallback(async () => {
    if (!fileData) return;
    try {
      // Only invoke the more expensive reorderPages when the order genuinely differs
      // from the original sequential order (i.e. pages were deleted or reordered).
      const isOrderModified =
        pageOrder.length > 0 && !pageOrder.every((v, i) => v === i);
      const bytes = isOrderModified
        ? await reorderPages(fileData, pageOrder)
        : await exportPdf(fileData);
      const blob = new Blob([bytes], { type: 'application/pdf' });
      const url = URL.createObjectURL(blob);
      const a = document.createElement('a');
      a.href = url;
      a.download = currentFileName ?? 'export.pdf';
      a.click();
      URL.revokeObjectURL(url);
    } catch (e) {
      console.error('Export failed:', e);
    }
  }, [fileData, pageOrder, currentFileName]);

  // Keyboard shortcut handler
  React.useEffect(() => {
    const handler = (e: KeyboardEvent) => {
      if ((e.ctrlKey || e.metaKey) && e.key === 'o') {
        e.preventDefault();
        handleOpenPdf();
      } else if ((e.ctrlKey || e.metaKey) && e.key === 's') {
        e.preventDefault();
        handleExport();
      } else if ((e.ctrlKey || e.metaKey) && (e.key === '=' || e.key === '+')) {
        e.preventDefault();
        setZoom(zoom + 10);
      } else if ((e.ctrlKey || e.metaKey) && e.key === '-') {
        e.preventDefault();
        setZoom(zoom - 10);
      } else if (e.key === 'Escape') {
        setTool(null);
        setShowSignaturePanel(false);
        setShowSecurityPanel(false);
        setShowOCRPanel(false);
      }
    };
    window.addEventListener('keydown', handler);
    return () => window.removeEventListener('keydown', handler);
  }, [zoom, setZoom, setTool, handleOpenPdf, handleExport]);

  const isToolActive = (tool: string) => activeTool === tool;

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
            <span className="btn-key">Ctrl/Cmd+O</span>
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
          <button
            className={`btn ${isToolActive('draw') ? 'btn-strong' : 'btn-ghost'}`}
            onClick={() => setTool(isToolActive('draw') ? null : 'draw')}
            title="Freehand draw (pencil)"
          >
            <span className="btn-icon">✏️</span>
            Draw
          </button>
          <button
            className={`btn ${isToolActive('highlight') ? 'btn-strong' : 'btn-ghost'}`}
            onClick={() => setTool(isToolActive('highlight') ? null : 'highlight')}
            title="Highlight"
          >
            <span className="btn-icon">🖍</span>
            Highlight
          </button>
          <button
            className={`btn ${isToolActive('text') ? 'btn-strong' : 'btn-ghost'}`}
            onClick={() => setTool(isToolActive('text') ? null : 'text')}
            title="Add text note"
          >
            <span className="btn-icon">✉</span>
            Comment
          </button>
          <button
            className={`btn ${isToolActive('rect') ? 'btn-strong' : 'btn-ghost'}`}
            onClick={() => setTool(isToolActive('rect') ? null : 'rect')}
            title="Add rectangle"
          >
            <span className="btn-icon">▭</span>
            Rect
          </button>
          <button
            className={`btn ${isToolActive('ellipse') ? 'btn-strong' : 'btn-ghost'}`}
            onClick={() => setTool(isToolActive('ellipse') ? null : 'ellipse')}
            title="Add ellipse"
          >
            <span className="btn-icon">⬭</span>
            Ellipse
          </button>
          <button
            className="btn btn-ghost"
            onClick={() => setShowSignaturePanel(true)}
            title="Add signature"
          >
            <span className="btn-icon">✍</span>
            Sign
          </button>
        </div>

        <div className="toolbar-group" aria-label="Annotation style">
          <label style={{ display: 'flex', alignItems: 'center', gap: '0.25rem', fontSize: '0.75rem', color: 'var(--text-muted)' }}>
            Color
            <input
              type="color"
              value={currentColor}
              onChange={(e) => setColor(e.target.value)}
              style={{ width: 24, height: 24, border: 'none', borderRadius: 4, cursor: 'pointer', background: 'transparent' }}
            />
          </label>
          <label style={{ display: 'flex', alignItems: 'center', gap: '0.25rem', fontSize: '0.75rem', color: 'var(--text-muted)' }}>
            Width
            <input
              type="range"
              min={1}
              max={20}
              value={strokeWidth}
              onChange={(e) => setStrokeWidth(Number(e.target.value))}
              style={{ width: 60, accentColor: '#4f46e5' }}
            />
          </label>
        </div>

        <div className="toolbar-group" aria-label="Export and security">
          <button className="btn btn-ghost" onClick={handleExport} disabled={!fileData}>
            <span className="btn-icon">⬇</span>
            Export
            <span className="btn-key">Ctrl/Cmd+S</span>
          </button>
          <button className="btn btn-ghost" onClick={() => setShowSecurityPanel(true)} disabled={!fileData}>
            <span className="btn-icon">🔒</span>
            Protect
          </button>
          <button className="btn btn-ghost" onClick={() => setShowOCRPanel(true)} disabled={!fileData}>
            <span className="btn-icon">🔍</span>
            OCR
          </button>
        </div>
      </nav>

      <PdfWorkspace />

      {showSignaturePanel && (
        <SignaturePanel
          onClose={() => setShowSignaturePanel(false)}
          onConfirm={(dataUrl) => {
            // TODO: embed signature image into annotation overlay
            console.log('Signature captured:', dataUrl.substring(0, 50));
            window.alert(
              'Your signature was captured, but adding it to the PDF is not yet supported in this version.'
            );
          }}
        />
      )}
      {showSecurityPanel && <SecurityPanel onClose={() => setShowSecurityPanel(false)} />}
      {showOCRPanel && <OCRPanel onClose={() => setShowOCRPanel(false)} />}
    </div>
  );
};

