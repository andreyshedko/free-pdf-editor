# Free PDF Editor (Desktop)

Cross-platform desktop PDF editor built with **Electron**, **React**, and **TypeScript**.

This is an early foundation focusing on:

- Electron shell (Windows & macOS ready)
- React + Vite renderer
- Zustand state store
- Modular architecture for viewer / editor / annotations / OCR / export / security
- PDF.js integration for rendering (single-page for now)

## Getting started

```bash
npm install
npm run dev
```

This starts:

- Vite dev server for the React renderer
- Electron shell pointing at `http://localhost:5173`

## High-level architecture

- `electron/` – Electron main & preload processes, native file dialogs, IPC entrypoints
- `src/` – React renderer
  - `shell/` – App chrome & layout
  - `modules/viewer/` – PDF viewing UI (thumbnails, main viewer, right inspector)
  - `core/pdf/` – PDF.js / pdf-lib integration (rendering, parsing, export)
  - `store/` – Zustand stores for document, annotations, UI state

Planned modules (stubs to be added):

- `modules/editor/` – Text & image editing (pdf-lib)
- `modules/annotations/` – Fabric.js overlay: highlights, shapes, comments
- `modules/pages/` – Page add/delete/reorder
- `modules/forms/` – Form filling & creation
- `modules/signatures/` – Drawing, uploaded certificates
- `modules/ocr/` – Tesseract.js integration
- `modules/export/` – Optimized export & conversion
- `modules/security/` – Encryption, permissions, redaction

## Next steps

- Wire the `Open PDF` action to the store and viewer
- Add lazy multi-page rendering (virtualized scroll)
- Add Fabric.js overlay for annotation tools
- Introduce module shells for editor, OCR, export, security
- Add tests for core pdf services and viewer components

