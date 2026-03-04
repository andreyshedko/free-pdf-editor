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
| `InsertImageCommand` | Embed a raw RGB bitmap as an uncompressed PDF Image XObject at a given position and display size | ✓ (snapshot) |
| `SetPasswordCommand` | Placeholder for owner-password protection | ✓ (snapshot) |
| `RedactRegionCommand` | Permanently remove text content within a region from the content streams and paint a filled black rectangle over it | ✓ (snapshot) |

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
- `CreateFieldCommand` — create a new AcroForm field (any `FormFieldKind`) on a page,
  automatically creating the `/AcroForm` catalog entry if absent; undo supported
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
AppController  ──render request──►  render-worker thread
      │                                    │
      │                          SoftwareRenderer::render_from_dims
      │                          Arc<Mutex<PageCache>>
      │                                    │
      │◄──invoke_from_event_loop───────────┘
      │
      │  DocumentEvent
      ▼
event-bridge thread  ──invoke_from_event_loop──►  UI thread
```

The Slint UI thread never blocks on rendering. Page rendering is dispatched to
a dedicated `render-worker` thread via a bounded channel.  The worker calls
`SoftwareRenderer::render_from_dims` (which takes only page dimensions, not the
full `Document`), stores the result in a shared `Arc<Mutex<PageCache>>`, then
delivers the pixel buffer back to the Slint event loop via
`invoke_from_event_loop`.  All PDF operations run on the UI thread (the
`Document` is not shared with the render thread, which is the right design for
future MuPDF integration where the document handle must be kept thread-local).
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

Tests cover (40 tests total):

- `pdf-core` — document open/save/page operations, `CommandHistory` undo/redo semantics
- `pdf-render` — LRU cache eviction and per-document cache eviction
- `pdf-editor` — delete/rotate/reorder/insert-text/insert-image execute-and-undo, redaction removes text in region, out-of-range errors
- `pdf-annotations` — add/remove annotation execute-and-undo, idempotent undo guard
- `pdf-forms` — AcroForm field detection, `SetFieldValueCommand` execute-and-undo, `CreateFieldCommand` (all field kinds, multi-field, undo)

## Extensibility

Every new feature follows the same pattern:

1. Define a struct that implements `DocumentCommand` (`execute` + `undo` + `description`).
2. Emit the appropriate `DocumentEvent` variant from the controller.
3. Wire a Slint callback in `AppController::wire_callbacks`.

No business logic lives in the UI layer.

## Gap analysis vs. full specification

The table below maps each requirement from the product specification to its
current implementation status.

### Rendering engine

| Requirement | Status | Notes |
|-------------|:------:|-------|
| Page rasterization | ⚠️ Stub | `SoftwareRenderer` produces a white rectangle with a border — no real pixel rendering. **MuPDF integration is not yet done.** |
| Zoom levels | ✅ | 0.1 × – 10 × |
| LRU page cache | ✅ | Keyed by `(doc_id, page, zoom)`; shared via `Arc<Mutex<PageCache>>` with the render worker |
| Text extraction | ✅ | Via lopdf |
| Coordinate mapping | ✅ | `MediaBox`-based |
| MuPDF as rendering backend | ❌ Not started | The specification lists MuPDF as the rendering library. Currently lopdf is used for document parsing and the renderer is a software stub. |

### Document engine

| Requirement | Status | Notes |
|-------------|:------:|-------|
| Open / save PDF | ✅ | |
| Incremental saves | ❌ Not started | Every save rewrites the full document. lopdf 0.39 does not expose an incremental-write API. |
| Undo / redo | ✅ | `CommandHistory` with configurable depth |
| Page tree management | ✅ | |

### Editing engine

| Requirement | Status | Notes |
|-------------|:------:|-------|
| Insert text | ✅ | `InsertTextCommand` |
| Modify existing text | ❌ Not started | Only new content streams can be appended; in-place text-object editing is not implemented. |
| Font substitution | ❌ Not started | |
| Insert image | ✅ | `InsertImageCommand` — embeds a raw RGB bitmap as an uncompressed `DeviceRGB` PDF Image XObject with undo support |
| Replace / resize image | ❌ Not started | |
| Delete / rotate / reorder pages | ✅ | |
| Merge documents | ✅ | |

### Annotation system

| Requirement | Status | Notes |
|-------------|:------:|-------|
| Highlight, Underline, Strikeout | ✅ | |
| Notes (sticky notes) | ✅ | |
| Drawing paths (ink) | ✅ | |
| Stamps | ✅ | |

### Forms engine

| Requirement | Status | Notes |
|-------------|:------:|-------|
| Detect form fields | ✅ | All AcroForm field types |
| Edit field values | ✅ | `SetFieldValueCommand` with undo |
| Create new form fields | ✅ | `CreateFieldCommand` — creates TextField, Checkbox, Radio, Dropdown, or SignatureField; creates AcroForm if absent; undo supported |
| Export form data (JSON) | ✅ | `export_form_data` |

### OCR

| Requirement | Status | Notes |
|-------------|:------:|-------|
| `OcrProvider` abstraction | ✅ | Trait + `OcrResult` / `TextRegion` types |
| Concrete OCR implementation | 🔲 By design | Spec says "do NOT implement OCR directly" — future provider slot |

### Security

| Requirement | Status | Notes |
|-------------|:------:|-------|
| Password protection | ⚠️ Placeholder | `SetPasswordCommand` logs a warning; lopdf 0.39 has no encryption API. Requires a different PDF library or MuPDF to implement properly. |
| Permissions | ❌ Not started | |
| Redaction | ✅ | `RedactRegionCommand` now performs **true redaction**: decompresses all content streams, parses `BT…ET` text blocks, removes blocks whose text position falls within the target rectangle, then re-encodes the result into a single filtered stream. A filled black rectangle is added on top as a visual marker. Falls back to visual-only if content stream parsing fails. |

### Performance targets

| Requirement | Status | Notes |
|-------------|:------:|-------|
| Memory-safe LRU cache | ✅ | |
| Background rendering (off UI thread) | ✅ | Dedicated `render-worker` thread; `SoftwareRenderer::render_from_dims` runs off the UI thread; results are handed back via `slint::invoke_from_event_loop`. The cache is shared via `Arc<Mutex<PageCache>>`. |
| Lazy page loading | ❌ Not started | `Document::open` loads the full lopdf object graph at open time. |
| `<100 ms` page navigation latency | ❌ Not measured | Achievable with real rendering (MuPDF) once integrated. |
| Incremental saves | ❌ Not started | See document engine row above. |

### Plugin system

| Requirement | Status | Notes |
|-------------|:------:|-------|
| `Plugin` trait | ✅ | `name()`, `on_load()`, `on_unload()` |
| `PluginContext` | ✅ | Exposes `EventBus` and tool registry |
| `PluginRegistry` | ✅ | Load / unload lifecycle |
| Runtime-loadable plugins (dylib) | ❌ Not started | Spec notes "design only" for this phase |

### Architecture compliance

| Principle | Status | Notes |
|-----------|:------:|-------|
| Core is UI-agnostic | ✅ | |
| UI communicates via commands / events | ✅ | |
| PDF manipulation independent of UI | ✅ | |
| Features as independent modules | ✅ | One crate per feature area |
| Trait-based abstractions | ✅ | `DocumentCommand`, `RenderEngine`, `OcrProvider`, `Plugin` |
| No global state | ✅ | |
| Workspace layout matches spec | ✅ | `pdf-core / pdf-render / pdf-editor / pdf-annotations / pdf-forms / app-desktop` |
| Async Rust | ❌ Not started | Spec lists async as part of the stack; currently all synchronous (background rendering uses OS threads, not async/await) |

**Legend:** ✅ Implemented · ⚠️ Partial / placeholder · ❌ Not started · 🔲 Intentionally deferred
