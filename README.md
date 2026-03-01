# Free PDF Editor (Desktop)

Cross-platform desktop PDF editor built with **Electron**, **React**, and **TypeScript**.

Key features:

- Electron shell (Windows, macOS, and Linux)
- React + Vite renderer
- Zustand state store
- PDF.js multi-page rendering with virtualized scroll
- Fabric.js annotation overlay: freehand draw, highlight, shapes, text comments
- Signature drawing panel
- OCR via Tesseract.js (runs fully offline)
- Page management: delete and reorder pages
- Export / Save PDF via pdf-lib
- Security panel (save with optional password – encryption coming)

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
  - `modules/viewer/` – PDF viewing UI: thumbnails pane, multi-page viewer, right inspector, annotation overlay, OCR panel, signature panel, security panel
  - `modules/editor/` – Text & image editing stub (pdf-lib, in progress)
  - `modules/forms/` – Form filling & creation stub (in progress)
  - `core/pdf/` – PDF.js rendering service, pdf-lib export service, Tesseract.js OCR service
  - `store/` – Zustand stores for document state, annotations, page management

## Planned / in-progress

- `modules/editor/` – Text & image editing via pdf-lib
- `modules/forms/` – Form filling & creation
- Password encryption in the security panel
- Embedding captured signatures into the PDF
- Drag-and-drop page reordering in the thumbnails pane

## Next steps

- Implement text / image editing layer (pdf-lib)
- Add form field detection and filling
- Wire password encryption into the export pipeline
- Embed signature images as PDF annotations
- Add drag-and-drop reordering to the thumbnails pane
- Add tests for viewer components (thumbnails, annotation overlay)

