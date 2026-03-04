# Free PDF Editor (Desktop)

Cross-platform offline-first desktop PDF editor built with **Rust**, **Slint** UI, and **lopdf**.

## Implemented functionality

### Document management (`pdf-core`)
- Open existing PDF files via `Document::open`
- Create new blank PDF documents via `Document::create_new`
- Save documents in-place (`save`) or to a new path (`save_to`)
- Page enumeration with accurate `MediaBox` dimensions
- Text extraction per page via `Document::extract_text`
- **Undo / Redo** — `CommandHistory` with configurable depth; every mutating
  operation implements the `DocumentCommand` trait and can be undone/redone
- **Event bus** — `EventBus` / `DocumentEvent` for loosely-coupled UI updates
- OCR provider trait (`OcrProvider`) for pluggable text recognition
- Plugin trait (`Plugin`) for future extension points

### Page rendering (`pdf-render`)
- `SoftwareRenderer` — pure-Rust rasterizer that renders page bitmaps (RGBA8)
  with a visible border; zoom range 0.1 × – 10 ×
- `PageCache` — LRU cache keyed by `(document_id, page_index, zoom)` with
  per-document eviction
- `get_text_boxes` — returns text with bounding-box coordinates

### Page editing (`pdf-editor`)
| Command | Description | Undo support |
|---------|-------------|:---:|
| `DeletePageCommand` | Remove a page by index | ✓ (snapshot) |
| `RotatePageCommand` | Set page rotation (multiples of 90°) | ✓ |
| `ReorderPagesCommand` | Reorder all pages by a new index mapping | ✓ |
| `MergeDocumentCommand` | Append all pages from another document | ✓ |
| `InsertTextCommand` | Add text at a specified position on a page (Helvetica, configurable size) | ✓ (snapshot) |
| `SetPasswordCommand` | Placeholder for owner-password protection | ✓ (snapshot) |
| `RedactRegionCommand` | Paint a filled black rectangle over a page region | ✓ (snapshot) |

### Annotations (`pdf-annotations`)
Annotation types supported: **Highlight**, **Underline**, **Strikeout**,
**Note** (sticky note with author/content), **Drawing** (freehand ink),
**Stamp**.

| Command | Description | Undo support |
|---------|-------------|:---:|
| `AddAnnotationCommand` | Write a new annotation into the PDF `Annots` array | ✓ |
| `RemoveAnnotationCommand` | Detach an annotation by UUID; re-attaches on undo without data loss | ✓ |

Annotations are persisted as proper PDF annotation dictionaries (not
just pixel overlays).  Serialization / deserialization helpers are
provided in `pdf_annotations::io`.

### Forms (`pdf-forms`)
- `detect_form_fields` — walks the AcroForm tree and returns all fields
  (text fields, checkboxes, radio buttons, dropdowns, signature fields)
  with their names, types, current values, page locations, and option lists
- `SetFieldValueCommand` — update a field's `/V` entry with undo support
- `export_form_data` — serialize all field values to a JSON object

### Desktop application (`app-desktop`)
Built with **Slint** 1.9 — a single-window UI with:

**Toolbar buttons**

| Button | Action |
|--------|--------|
| Open | Opens a file via `OPEN_PDF` env var (dialog stub) |
| Save | Saves to `SAVE_PDF` env var path |
| Save As | Saves to `SAVE_AS_PDF` env var path |
| Close | Closes the current document |
| Undo / Redo | Undo / redo last command (disabled when unavailable) |
| Zoom − / Zoom + / 100% | ×0.8 / ×1.25 / reset to 1.0 |
| Prev / Next | Page navigation |
| Highlight | Adds a yellow highlight annotation at a fixed position |
| Note | Adds a sticky-note annotation at a fixed position |
| Del Page | Deletes the current page |
| Rotate | Rotates the current page 90° clockwise |

**Status bar** shows the document title, current page / total pages, and a
status message (errors, zoom level, save path, etc.).

**Thread model**

```
UI thread (Slint event loop)
      │  callbacks
      ▼
AppController  ──render──►  SoftwareRenderer
      │                          │
      │                          ▼
      │                      PageCache (LRU)
      │
      │  DocumentEvent
      ▼
event-bridge thread  ──invoke_from_event_loop──►  UI thread
```

All PDF operations run on the UI thread (single-threaded for simplicity).
The event bridge thread forwards `DocumentEvent` messages back to the Slint
event loop so state updates happen safely on the UI thread.

## Stack

| Layer | Technology |
|-------|------------|
| Language | Rust (edition 2021) |
| UI | [Slint](https://slint.dev) 1.9 |
| PDF library | [lopdf](https://crates.io/crates/lopdf) 0.39 |
| Build | Cargo workspace |
| Targets | Windows · macOS · Linux |

## Workspace structure

```
pdf-core          ← Document model, CommandHistory, EventBus, OCR/Plugin traits
pdf-render        ← SoftwareRenderer, PageCache, TextBox
pdf-editor        ← Page / text / security editing commands
pdf-annotations   ← Annotation CRUD commands + PDF I/O
pdf-forms         ← AcroForm field detection, value commands, JSON export
app-desktop       ← Slint UI, AppController, main entry point
```

## Getting started

### Prerequisites

- Rust ≥ 1.75
- A system font library (fontconfig on Linux, built-in on macOS/Windows) for Slint

### Build

```bash
cargo build -p pdf-core -p pdf-render -p pdf-editor -p pdf-annotations -p pdf-forms
```

To build the full desktop application (requires a display / fontconfig):

```bash
cargo build --bin pdf-editor
```

#### Building a release executable

**Linux / macOS**

```bash
cargo build --release --bin pdf-editor
# output: target/release/pdf-editor
```

**Windows**

```powershell
cargo build --release --bin pdf-editor
# output: target\release\pdf-editor.exe
```

### Run

```bash
OPEN_PDF=/path/to/file.pdf cargo run --bin pdf-editor
```

### Test

Run library-only tests (the `app-desktop` crate is excluded because Slint
requires a display and fontconfig on Linux):

```bash
cargo test -p pdf-core -p pdf-render -p pdf-editor -p pdf-annotations -p pdf-forms
```

Tests cover (27 tests total):

- `pdf-core` — document open/save/page operations, `CommandHistory` undo/redo semantics
- `pdf-render` — LRU cache eviction and per-document cache eviction
- `pdf-editor` — delete/rotate/reorder/insert-text execute-and-undo, out-of-range errors
- `pdf-annotations` — add/remove annotation execute-and-undo, idempotent undo guard
- `pdf-forms` — AcroForm field detection, `SetFieldValueCommand` execute-and-undo

## Extensibility

Every new feature follows the same pattern:

1. Define a struct that implements `DocumentCommand` (`execute` + `undo` + `description`).
2. Emit the appropriate `DocumentEvent` variant from the controller.
3. Wire a Slint callback in `AppController::wire_callbacks`.

No business logic lives in the UI layer.
