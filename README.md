# Free PDF Editor (Desktop)

Cross-platform desktop PDF editor built with **Electron**, **React**, and **TypeScript**.

Key features:

- Electron shell (Windows, macOS, and Linux)
- React + Vite renderer
- Zustand state store
- PDF.js multi-page rendering with continuous scroll (no virtualization yet)
- Fabric.js annotation overlay: freehand draw, highlight, shapes, text comments
- Signature drawing and embedding into PDF (bottom-right of current page)
- OCR via Tesseract.js (runs fully offline)
- Page management: delete pages and reorder via drag-and-drop in the thumbnails pane
- Text insertion via pdf-lib (Helvetica, configurable position/size/color)
- PDF form field detection and filling (AcroForm text fields, checkboxes, dropdowns)
- Export / Save PDF via pdf-lib
- Security panel (save PDF; password encryption requires a library beyond pdf-lib v1)

## Getting started

```bash
npm install
npm run dev
```

This starts:

- Vite dev server for the React renderer
- Electron shell pointing at `http://localhost:5173`

## Keyboard shortcuts

| Shortcut | Action |
|----------|--------|
| `Ctrl/Cmd+O` | Open PDF |
| `Ctrl/Cmd+S` | Export / Save PDF |
| `Ctrl/Cmd+=` | Zoom in |
| `Ctrl/Cmd+-` | Zoom out |
| `Esc` | Deselect tool / close panels |

## High-level architecture

- `electron/` – Electron main & preload processes, native file dialogs, IPC entrypoints
- `src/` – React renderer
  - `shell/` – App chrome & layout (`App.tsx`)
  - `modules/viewer/` – PDF viewing UI: thumbnails pane with drag-to-reorder, multi-page viewer, right inspector, annotation overlay, OCR panel, signature panel, security panel
  - `modules/editor/` – Text insertion panel and service (`EditorPanel`, `editorService`)
  - `modules/forms/` – Form field filling panel and service (`FormsPanel`, `formsService`)
  - `core/pdf/` – PDF.js rendering service, pdf-lib export/edit/signature service, Tesseract.js OCR service
  - `store/` – Zustand stores for document state, annotations, page management

## Planned / in-progress

- PDF password encryption (pdf-lib v1 does not support writing encrypted PDFs; a different library is needed)
- Image insertion into pages (groundwork exists via `pdfDoc.embedPng`)
- Virtualized scroll for very large documents

## Next steps

- Add tests for viewer components (thumbnails, annotation overlay)
- Add image insertion panel alongside the text editor
- Implement PDF password encryption with a suitable library

