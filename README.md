# Free PDF Editor (Desktop)

Cross-platform offline-first desktop PDF editor built with **Rust**, **Slint** UI, **lopdf** (document model), and **MuPDF** (rendering).

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
- `MuPdfRenderer` — **MuPDF-backed rasterizer** that renders real page bitmaps (RGBA8) via `libmupdf`; extracts per-block text bounding boxes; falls back to `SoftwareRenderer` for unsaved in-memory documents
- `SoftwareRenderer` — pure-Rust fallback that produces a white rectangle with a border (used in tests and for unsaved documents)
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
| `ModifyTextCommand` | Replace every literal-string occurrence of `old_text` with `new_text` across all content streams on a page | ✓ (snapshot) |
| `InsertImageCommand` | Embed a raw RGB bitmap as an uncompressed PDF Image XObject at a given position and display size | ✓ (snapshot) |
| `ReplaceImageCommand` | Replace an existing Image XObject (by resource name) with new raw RGB data; optionally update display dimensions via the `cm` transform | ✓ (snapshot) |
| `FontSubstitutionCommand` | Replace all `Tf` references to one font with another across a page's content streams; auto-registers standard Type1 fonts in `/Resources/Font` | ✓ (snapshot) |
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
      │                          MuPdfRenderer::render_from_path
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
`MuPdfRenderer::render_from_path`, which opens its own `mupdf::Document` handle
per render call so no document handle is shared across threads.  Results are
stored in a shared `Arc<Mutex<PageCache>>` then delivered back to the Slint
event loop via `invoke_from_event_loop`.  The event bridge thread forwards
`DocumentEvent` messages back to the Slint event loop so state updates happen
safely on the UI thread.

## Stack

| Layer | Technology |
|-------|------------|
| Language | Rust (edition 2021) |
| UI | [Slint](https://slint.dev) 1.9 |
| PDF library | [lopdf](https://crates.io/crates/lopdf) 0.39 (document model) · [MuPDF](https://mupdf.com/) 1.23 via [mupdf](https://crates.io/crates/mupdf) 0.6 (rendering) |
| Build | Cargo workspace |
| Targets | Windows · macOS · Linux |

## Workspace structure

```
pdf-core          ← Document model, CommandHistory, EventBus, OCR/Plugin traits
pdf-render        ← MuPdfRenderer, SoftwareRenderer (fallback), PageCache, TextBox
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

Tests cover (52 tests total):

- `pdf-core` — document open/save/page operations, `CommandHistory` undo/redo semantics
- `pdf-render` — LRU cache eviction and per-document cache eviction
- `pdf-editor` — delete/rotate/reorder/insert-text/modify-text/font-substitution/insert-image/replace-image execute-and-undo, redaction removes text in region, out-of-range errors
- `pdf-annotations` — add/remove annotation execute-and-undo, idempotent undo guard
- `pdf-forms` — AcroForm field detection, `SetFieldValueCommand` execute-and-undo, `CreateFieldCommand` (all field kinds, multi-field, undo)

## Publishing

### Microsoft Store (Windows)

The build script produces a signed **MSIX** package that can be submitted directly to
[Microsoft Partner Center](https://partner.microsoft.com/dashboard).

#### Prerequisites

| Tool | Notes |
|------|-------|
| Windows SDK (`MakeAppx.exe`, `signtool.exe`) | Installed with Visual Studio or the standalone Windows SDK |
| Rust target `x86_64-pc-windows-msvc` | `rustup target add x86_64-pc-windows-msvc` |
| A code-signing certificate (PFX) | EV or standard certificate issued by a trusted CA |

#### Required environment variables

| Variable | Description |
|----------|-------------|
| `WINDOWS_CERT_BASE64` | Base-64-encoded PFX certificate |
| `WINDOWS_CERT_PASSWORD` | PFX certificate password |
| `PUBLISHER` | Publisher identity string from Partner Center, e.g. `CN=Example, O=Example Inc, L=Redmond, S=Washington, C=US` |

#### Steps

1. **Register in Partner Center** — create a new app reservation at
   [Partner Center](https://partner.microsoft.com/dashboard) and note your
   *Publisher identity* (used as `PUBLISHER` above).

2. **Update store metadata** — edit `store/metadata.json` to set
   `windows_package_name` to the package name shown in Partner Center.

3. **Set the version** — bump `version` and `build_number` in
   `release/release.json`.

4. **Build and package**

   ```powershell
   $Env:WINDOWS_CERT_BASE64   = "<base64 PFX>"
   $Env:WINDOWS_CERT_PASSWORD = "<password>"
   $Env:PUBLISHER             = "CN=..."
   .\scripts\build_windows.ps1
   # Output: dist\windows\FreePDFEditor_<VERSION>.msix
   ```

   Set `SKIP_SIGNING=1` to build without signing (local testing only —
   Partner Center re-signs the package on ingestion, so you may omit signing
   for Store submissions if your Partner Center account supports it).

5. **Submit to the Store** — in Partner Center create a new submission, upload
   `dist\windows\FreePDFEditor_<VERSION>.msix` as the package, fill in the
   listing details, and click **Submit to certification**.

---

### Mac App Store (Apple)

The build scripts produce a notarized **.pkg** installer. For the Mac App
Store you need an *Apple Distribution* certificate instead of a Developer ID
certificate; the notarization step is replaced by uploading directly through
App Store Connect.

#### Prerequisites

| Tool | Notes |
|------|-------|
| Xcode Command Line Tools | `xcode-select --install` |
| Rust targets for Apple Silicon and Intel | `rustup target add aarch64-apple-darwin x86_64-apple-darwin` |
| Active Apple Developer Program membership | [developer.apple.com](https://developer.apple.com) |

#### Required environment variables

| Variable | Description |
|----------|-------------|
| `APPLE_CERT_BASE64` | Base-64-encoded Distribution certificate (p12) — *"Apple Distribution: …"* or *"3rd Party Mac Developer Application: …"* |
| `APPLE_CERT_PASSWORD` | Certificate password |
| `APPLE_TEAM_ID` | 10-character Apple Developer Team ID |
| `APPLE_SIGN_IDENTITY` | Full common name of the signing certificate, e.g. `Apple Distribution: Your Name (TEAMID)` |
| `APPLE_INSTALLER_CERT_BASE64` | Base-64-encoded installer certificate (p12) — *"3rd Party Mac Developer Installer: …"* |
| `APPLE_INSTALLER_CERT_PASSWORD` | Installer certificate password |
| `APPLE_INSTALLER_SIGN_IDENTITY` | Full common name of the installer certificate, e.g. `3rd Party Mac Developer Installer: Your Name (TEAMID)` |
| `APPLE_API_KEY_ID` | App Store Connect API key ID |
| `APPLE_API_ISSUER_ID` | App Store Connect API issuer ID |
| `APPLE_API_PRIVATE_KEY` | Contents of the `.p8` private key file |

#### Steps

1. **Create the app in App Store Connect** — go to
   [appstoreconnect.apple.com](https://appstoreconnect.apple.com), create a
   new macOS app, and note the *Bundle ID* (must match `bundle_id` in
   `store/metadata.json`, currently `com.freepdfeditor.app`).

2. **Create an App Store Connect API key** — in App Store Connect → Users and
   Access → Integrations → App Store Connect API, generate a key with
   *Developer* or *Admin* role and download the `.p8` file.

3. **Export certificates from Xcode** — in Xcode → Settings → Accounts →
   Manage Certificates, create and export:
   - *Apple Distribution* (application signing)
   - *3rd Party Mac Developer Installer* (package signing)

4. **Update store metadata and version**

   ```bash
   # store/metadata.json  → set bundle_id, display_name, description
   # release/release.json → bump version and build_number
   ```

5. **Build a universal binary**

   ```bash
   bash scripts/build_macos.sh
   # Output: dist/macos/FreePDFEditor.app
   ```

   To build without signing (e.g. for testing):

   ```bash
   SKIP_SIGNING=1 bash scripts/build_macos.sh
   ```

6. **Sign and package**

   ```bash
   # Code-sign the .app bundle
   export APPLE_CERT_BASE64="<base64 p12>"
   export APPLE_CERT_PASSWORD="<password>"
   export APPLE_TEAM_ID="<TEAMID>"
   export APPLE_SIGN_IDENTITY="Apple Distribution: Your Name (TEAMID)"
   bash scripts/sign_macos.sh

   # Build the signed .pkg and submit for notarization
   export APPLE_INSTALLER_CERT_BASE64="<base64 installer p12>"
   export APPLE_INSTALLER_CERT_PASSWORD="<password>"
   export APPLE_INSTALLER_SIGN_IDENTITY="3rd Party Mac Developer Installer: Your Name (TEAMID)"
   export APPLE_API_KEY_ID="<key id>"
   export APPLE_API_ISSUER_ID="<issuer id>"
   export APPLE_API_PRIVATE_KEY="$(cat /path/to/AuthKey_KEYID.p8)"
   bash scripts/notarize_macos.sh
   # Output: dist/macos/FreePDFEditor_<VERSION>.pkg
   ```

7. **Upload to App Store Connect** — use Apple's *Transporter* app
   ([download from Mac App Store](https://apps.apple.com/app/transporter/id1450874784))
   or its bundled CLI:

   ```bash
   # Transporter CLI (installed with the Transporter app)
   /Applications/Transporter.app/Contents/itms/bin/iTMSTransporter \
     -m upload \
     -f dist/macos/FreePDFEditor_<VERSION>.pkg \
     -apiKey "$APPLE_API_KEY_ID" \
     -apiIssuer "$APPLE_API_ISSUER_ID"
   ```

   Alternatively, open **Xcode → Organizer → Distribute App** and follow the
   guided upload workflow.

8. **Submit for review** — in App Store Connect, select the uploaded build,
   complete the required metadata (screenshots, description, privacy details),
   and click **Submit for Review**.

---

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
| Page rasterization | ✅ | `MuPdfRenderer::render_from_path` opens a `mupdf::Document`, calls `page.to_pixmap()`, and converts the RGB pixmap to RGBA8. |
| Zoom levels | ✅ | 0.1 × – 10 × |
| LRU page cache | ✅ | Keyed by `(doc_id, page, zoom)`; shared via `Arc<Mutex<PageCache>>` with the render worker |
| Text extraction | ✅ | Via lopdf (command layer) and MuPDF `TextPage` blocks (renderer) |
| Coordinate mapping | ✅ | `MediaBox`-based |
| MuPDF as rendering backend | ✅ | `MuPdfRenderer` uses `mupdf` 0.6 (wraps libmupdf 1.23) for rasterization and text-box extraction. lopdf is retained for document editing commands. |

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
| Modify existing text | ✅ | `ModifyTextCommand` — decompresses content streams, replaces literal-string occurrences of the target text in `Tj`/`TJ` operators, and re-encodes the result as a merged stream. |
| Font substitution | ✅ | `FontSubstitutionCommand` — replaces `Tf` font-name operands in all content streams on a page and auto-adds standard Type1 font entries to `/Resources/Font`. |
| Insert image | ✅ | `InsertImageCommand` — embeds a raw RGB bitmap as an uncompressed `DeviceRGB` PDF Image XObject with undo support |
| Replace / resize image | ✅ | `ReplaceImageCommand` — replaces the pixel data and intrinsic dimensions of an existing Image XObject identified by resource name; optionally updates the on-page `cm` transform for display resizing. |
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
| Background rendering (off UI thread) | ✅ | Dedicated `render-worker` thread; `MuPdfRenderer::render_from_path` runs off the UI thread; results are handed back via `slint::invoke_from_event_loop`. The cache is shared via `Arc<Mutex<PageCache>>`. |
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
