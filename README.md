# Free PDF Editor (Desktop)

Cross-platform offline-first desktop PDF editor built with **Rust**, **Qt/QML** UI, **lopdf** (document model), and **MuPDF** (rendering).

## C++/Qt6 migration scaffold

A new C++/Qt6 project scaffold is now available in `src/` and is built with CMake.
It now includes layered modules (`app/ui/editor/document/overlay/pdf_engine/cache/ocr`) and compiles to `pdf-editor.exe`.

Build (from repo root):

```bash
cmake -S . -B build-cpp
cmake --build build-cpp --config Release
```

Windows + Qt MinGW (explicit toolchain paths):

```powershell
$env:PATH = "C:\Qt\Tools\mingw1310_64\bin;C:\Qt\6.10.2\mingw_64\bin;$env:PATH"
& "C:\Program Files\CMake\bin\cmake.exe" -S . -B build-cpp-mingw `
  -G "MinGW Makefiles" `
  -DCMAKE_PREFIX_PATH="C:/Qt/6.10.2/mingw_64" `
  -DCMAKE_C_COMPILER="C:/Qt/Tools/mingw1310_64/bin/gcc.exe" `
  -DCMAKE_CXX_COMPILER="C:/Qt/Tools/mingw1310_64/bin/g++.exe" `
  -DCMAKE_MAKE_PROGRAM="C:/Qt/Tools/mingw1310_64/bin/mingw32-make.exe"
& "C:\Program Files\CMake\bin\cmake.exe" --build build-cpp-mingw -j 8
```

Optional runtime dependencies:

```powershell
# PDFium runtime (if pdfium.dll is not in PATH)
$env:PDFIUM_DLL="C:\\path\\to\\pdfium.dll"

# OCR runtime (if tesseract.exe is not in PATH)
$env:PATH="C:\\Program Files\\Tesseract-OCR;$env:PATH"
```

Run:

```bash
# Linux/macOS
./build-cpp/src/pdf-editor

# Windows (Visual Studio generator)
build-cpp\\src\\Release\\pdf-editor.exe

# Windows (MinGW generator)
build-cpp-mingw\\src\\pdf-editor.exe
```

## Implemented functionality

### Document management (`pdf-core`)
- Open existing PDF files via `Document::open`
- Create new blank PDF documents via `Document::create_new`
- Save documents in-place (`save`) or to a new path (`save_to`)
- **Incremental saves** вЂ” `save_incremental` / `save_incremental_to` append a new revision to the original file bytes using `lopdf::IncrementalDocument`; falls back to a full rewrite for freshly-created documents
- Page enumeration with accurate `MediaBox` dimensions
- Text extraction per page via `Document::extract_text`
- **Undo / Redo** вЂ” `CommandHistory` with configurable depth; every mutating
  operation implements the `DocumentCommand` trait and can be undone/redone
- **Event bus** вЂ” `EventBus` / `DocumentEvent` for loosely-coupled UI updates
- OCR provider trait (`OcrProvider`) for pluggable text recognition; `NoOpOcrProvider` available as a zero-dependency stub
- Plugin trait (`Plugin`) for future extension points

### Page rendering (`pdf-render`)
- `MuPdfRenderer` вЂ” **MuPDF-backed rasterizer** that renders real page bitmaps (RGBA8) via `libmupdf`; extracts per-block text bounding boxes; falls back to `SoftwareRenderer` for unsaved in-memory documents
- `SoftwareRenderer` вЂ” pure-Rust fallback that produces a white rectangle with a border (used in tests and for unsaved documents)
  with a visible border; zoom range 0.1 Г— вЂ“ 10 Г—
- `PageCache` вЂ” LRU cache keyed by `(document_id, page_index, zoom)` with
  per-document eviction
- `get_text_boxes` вЂ” returns text with bounding-box coordinates

### Page editing (`pdf-editor`)
| Command | Description | Undo support |
|---------|-------------|:---:|
| `DeletePageCommand` | Remove a page by index | вњ“ (snapshot) |
| `RotatePageCommand` | Set page rotation (multiples of 90В°) | вњ“ |
| `ReorderPagesCommand` | Reorder all pages by a new index mapping | вњ“ |
| `MergeDocumentCommand` | Append all pages from another document | вњ“ |
| `InsertTextCommand` | Add text at a specified position on a page (Helvetica, configurable size) | вњ“ (snapshot) |
| `ModifyTextCommand` | Replace every literal-string occurrence of `old_text` with `new_text` across all content streams on a page | вњ“ (snapshot) |
| `InsertImageCommand` | Embed a raw RGB bitmap as an uncompressed PDF Image XObject at a given position and display size | вњ“ (snapshot) |
| `ReplaceImageCommand` | Replace an existing Image XObject (by resource name) with new raw RGB data; optionally update display dimensions via the `cm` transform | вњ“ (snapshot) |
| `FontSubstitutionCommand` | Replace all `Tf` references to one font with another across a page's content streams; auto-registers standard Type1 fonts in `/Resources/Font` | вњ“ (snapshot) |
| `SetPasswordCommand` | Apply RC4-128 owner-password encryption to the document using `lopdf`'s `EncryptionVersion::V2`; injects a `/ID` trailer entry when absent | вњ“ (snapshot) |
| `RedactRegionCommand` | Permanently remove text content within a region from the content streams and paint a filled black rectangle over it | вњ“ (snapshot) |
| `ApplyOcrCommand` | Apply pre-computed `OcrResult` regions as an invisible text layer (render mode 3) on a page, enabling text selection in conforming PDF viewers | вњ“ (snapshot) |

### Annotations (`pdf-annotations`)
Annotation types supported: **Highlight**, **Underline**, **Strikeout**,
**Note** (sticky note with author/content), **Drawing** (freehand ink),
**Stamp**.

| Command | Description | Undo support |
|---------|-------------|:---:|
| `AddAnnotationCommand` | Write a new annotation into the PDF `Annots` array | вњ“ |
| `RemoveAnnotationCommand` | Detach an annotation by UUID; re-attaches on undo without data loss | вњ“ |

Annotations are persisted as proper PDF annotation dictionaries (not
just pixel overlays).  Serialization / deserialization helpers are
provided in `pdf_annotations::io`.

### Forms (`pdf-forms`)
- `detect_form_fields` вЂ” walks the AcroForm tree and returns all fields
  (text fields, checkboxes, radio buttons, dropdowns, signature fields)
  with their names, types, current values, page locations, and option lists
- `CreateFieldCommand` вЂ” create a new AcroForm field (any `FormFieldKind`) on a page,
  automatically creating the `/AcroForm` catalog entry if absent; undo supported
- `SetFieldValueCommand` вЂ” update a field's `/V` entry with undo support
- `export_form_data` вЂ” serialize all field values to a JSON object

### Desktop application (`app-desktop`)
Built with **Qt Quick (QML)** via `qmetaobject` — currently a migration shell.

**Current shell UI**

| Button | Action |
|--------|--------|
| Open | Placeholder action |
| Save | Placeholder action |
| Insert Text | Placeholder action |

The current Qt window contains a toolbar, canvas placeholder, and status bar.

Full parity with the previous desktop controller is tracked in `docs/qt-migration.md`.

**Thread model**

```
Qt UI thread (QML event loop)
      |
      `-- Rust bridge / controller wiring (in migration)
```

The previous Slint-specific render worker wiring is being ported to Qt.

## Stack

| Layer | Technology |
|-------|------------|
| Language | Rust (edition 2021) |
| UI | [Qt Quick / QML](https://doc.qt.io/qt-6/qtquick-index.html) via [qmetaobject](https://crates.io/crates/qmetaobject) |
| PDF library | [lopdf](https://crates.io/crates/lopdf) 0.39 (document model) В· [MuPDF](https://mupdf.com/) 1.23 via [mupdf](https://crates.io/crates/mupdf) 0.6 (rendering) |
| OCR | [Tesseract](https://github.com/tesseract-ocr/tesseract) 5.x via [tesseract](https://crates.io/crates/tesseract) 0.15 (`pdf-ocr` crate) |
| Build | Cargo workspace |
| Targets | Windows В· macOS В· Linux |

## Workspace structure

```
pdf-core          в†ђ Document model, CommandHistory, EventBus, OCR/Plugin traits
pdf-render        в†ђ MuPdfRenderer, SoftwareRenderer (fallback), PageCache, TextBox
pdf-editor        в†ђ Page / text / security / OCR editing commands
pdf-ocr           в†ђ TesseractOcrProvider вЂ” Tesseract-backed OcrProvider implementation
pdf-annotations   в†ђ Annotation CRUD commands + PDF I/O
pdf-forms         в†ђ AcroForm field detection, value commands, JSON export
app-desktop       в†ђ Qt/QML desktop frontend (`pdf-editor` binary)
```

## Getting started

### Prerequisites

- Rust в‰Ґ 1.75
- Qt installation with `qmake` available (or `QMAKE` env var pointing to it)
- `make` available in `PATH` for MuPDF builds on MinGW (e.g. `C:\msys64\usr\bin\make.exe`)
- **Tesseract 5.x** headers and `libtesseract` (required to build `pdf-ocr`)
  - Ubuntu/Debian: `sudo apt-get install libtesseract-dev tesseract-ocr`
  - macOS: `brew install tesseract`
  - Windows: install via [UB Mannheim tesseract installer](https://github.com/UB-Mannheim/tesseract/wiki)
  - Language data (e.g. English): `sudo apt-get install tesseract-ocr-eng` (or set `TESSDATA_PREFIX` to point at your tessdata directory)

### Build

```bash
cargo build -p pdf-core -p pdf-render -p pdf-editor -p pdf-annotations -p pdf-forms
```

To build the full desktop application (Qt + MinGW on Windows):

```powershell
.\scripts\setup_qt_mingw_env.ps1
cargo build -p app-desktop --bin pdf-editor --target x86_64-pc-windows-gnu --features mupdf
```

#### Building a release executable

**Linux / macOS**

```bash
cargo build --release -p app-desktop --bin pdf-editor
# output: target/release/pdf-editor (or platform-specific target dir)
```

**Windows**

```powershell
.\scripts\setup_qt_mingw_env.ps1
cargo build --release -p app-desktop --bin pdf-editor --target x86_64-pc-windows-gnu --features mupdf
# output: target\x86_64-pc-windows-gnu\release\pdf-editor.exe
```

### Run

```powershell
.\scripts\setup_qt_mingw_env.ps1
cargo run -p app-desktop --bin pdf-editor --target x86_64-pc-windows-gnu --features mupdf
```

### Debug run (Windows)

```powershell
.\scripts\setup_qt_mingw_env.ps1
$env:RUST_LOG='app_desktop=debug'
cargo run -p app-desktop --bin pdf-editor --target x86_64-pc-windows-gnu --features mupdf
```

### Windows troubleshooting (`--features mupdf`)

If you get:

`Failed to call make: program not found`

run:

```powershell
.\scripts\setup_qt_mingw_env.ps1
```

This script:
- copies `mingw32-make.exe` to `%USERPROFILE%\.cargo\bin\make.exe`
- configures `QMAKE`, `CC`, `CXX`, `AR`, `SHELL`
- prepends required Qt/MinGW/Git paths for the current PowerShell session

If you still hit a `mupdf-sys` bindgen error with `ia32intrin.h` / `mmintrin.h`, the current workaround is to use a libclang/LLVM installation compatible with your MinGW toolchain.
Optional fallback (useful in headless environments):

```powershell
# OPEN_PDF fallback is not wired in the Qt shell yet.
# Use regular run command above.
```

### Test

Run library-only tests:

```bash
cargo test -p pdf-core -p pdf-render -p pdf-editor -p pdf-annotations -p pdf-forms
```

Tests cover (52 tests total):

- `pdf-core` вЂ” document open/save/page operations, `CommandHistory` undo/redo semantics
- `pdf-render` вЂ” LRU cache eviction and per-document cache eviction
- `pdf-editor` вЂ” delete/rotate/reorder/insert-text/modify-text/font-substitution/insert-image/replace-image execute-and-undo, redaction removes text in region, out-of-range errors
- `pdf-annotations` вЂ” add/remove annotation execute-and-undo, idempotent undo guard
- `pdf-forms` вЂ” AcroForm field detection, `SetFieldValueCommand` execute-and-undo, `CreateFieldCommand` (all field kinds, multi-field, undo)

### Desktop E2E (Windows)

For UI regression checks on the native desktop app, use Appium + WinAppDriver.

1. Install Appium and Windows driver:

```powershell
npm i -g appium
appium driver install windows
```

2. Start WinAppDriver (Administrator) on `127.0.0.1:4723`.

3. Run the smoke test from repo root:

```powershell
scripts\run_e2e_windows.ps1
```

Run regression scenarios:

```powershell
scripts\run_e2e_windows.ps1 -Suite regression
```

The script builds `pdf-editor.exe` (debug) and runs `tests/e2e/test/smoke.mjs`.
See `tests/e2e/README.md` for environment variables and manual run options.

## License management

The application uses an **ED25519-signed** JSON license file to gate commercial features.
The license system lives in `services/licensing` (runtime verification) and
`tools/license-generator` (offline issuance CLI).

### License types and included features

| Type | `editor` | `forms` | `ocr` | `batch` | Notes |
|------|:--------:|:-------:|:-----:|:-------:|-------|
| `personal` | вњ“ | | | | Free tier, no expiry |
| `trial` | вњ“ | вњ“ | | | 14-day auto-trial on first launch |
| `commercial` | вњ“ | вњ“ | вњ“ | | Paid single-seat or multi-seat |
| `enterprise` | вњ“ | вњ“ | вњ“ | вњ“ | Includes batch processing |

### 1. Generate an ED25519 key pair

The private key is used only by the license generator (never shipped with the app).
The public key is embedded into the application at compile time.

```bash
# Requires Python 3 with the cryptography package:
#   pip install cryptography
python3 - <<'EOF'
from cryptography.hazmat.primitives.asymmetric.ed25519 import Ed25519PrivateKey
key = Ed25519PrivateKey.generate()
priv = key.private_bytes_raw().hex()
pub  = key.public_key().public_bytes_raw().hex()
print(f"Private key (keep secret): {priv}")
print(f"Public key  (embed in app): {pub}")
EOF
```

Store the **private key** securely (e.g. as `LICENSE_PRIVATE_KEY` in your CI
secrets or a password manager).  The **public key** is embedded into production
builds via the `APP_PUBLIC_KEY` environment variable (see step 5).

### 2. Build the license-generator CLI

```bash
cargo build --release -p license-generator
# Output: target/release/license-generator  (or .exe on Windows)
```

### 3. Generate a license file

```bash
export LICENSE_PRIVATE_KEY=<64-hex-char private key seed from step 1>

# Personal license (no expiry)
./target/release/license-generator generate \
    --holder "Jane Doe" \
    --email  jane@example.com \
    --type   personal \
    --seats  1

# Commercial license, 5 seats, expires 2028-12-31
./target/release/license-generator generate \
    --holder "ACME Inc" \
    --email  admin@acme.com \
    --type   commercial \
    --seats  5 \
    --expiry 2028-12-31

# Enterprise license
./target/release/license-generator generate \
    --holder "Big Corp" \
    --email  licensing@bigcorp.com \
    --type   enterprise \
    --seats  50
```

Each run writes a `<holder>-<type>.pdfeditor-license` file to the current
directory and prints the JSON to stdout.  Spaces and special characters in
`--holder` are replaced with `_` and the name is lowercased, e.g.
`"ACME Inc"` в†’ `acme_inc-commercial.pdfeditor-license`.

#### CLI flags

| Flag | Required | Description |
|------|:--------:|-------------|
| `--holder <name>` | вњ“ | License holder name (used in filename and `issued_to`) |
| `--email <address>` | вњ“ | Contact e-mail address |
| `--type <type>` | вњ“ | `personal` В· `trial` В· `commercial` В· `enterprise` |
| `--seats <n>` | | Number of seats (default: 1) |
| `--expiry YYYY-MM-DD` | | Expiry date (default: `9999-12-31` = no expiry) |

### 4. Inspect a license file

```bash
./target/release/license-generator inspect acme_inc-commercial.pdfeditor-license
# License ID : LIC-20260101120000-AI-4321
# Type       : commercial
# Issued to  : ACME Inc <admin@acme.com>
# Product    : PdfEditor
# Seats      : 5
# Expiry     : 2028-12-31
# Features   : editor, ocr, forms
# Signature  : AbCdEfGhIjKlвЂ¦
```

### 5. Embed the public key in production builds

Pass `APP_PUBLIC_KEY` when building the `licensing` crate (or the full app).
The build script (`services/licensing/build.rs`) validates the key and bakes it
in at compile time.

```bash
# Linux / macOS
export APP_PUBLIC_KEY=<64-hex-char public key from step 1>
cargo build --release -p app-desktop --bin pdf-editor --target x86_64-pc-windows-gnu --features mupdf

# Windows (PowerShell)
$Env:APP_PUBLIC_KEY = "<64-hex-char public key>"
$env:QMAKE='C:\Qt\6.10.2\mingw_64\bin\qmake.exe'
$env:PATH='C:\Qt\Tools\mingw1310_64\bin;C:\Qt\6.10.2\mingw_64\bin;'+$env:PATH
cargo build --release -p app-desktop --bin pdf-editor --target x86_64-pc-windows-gnu --features mupdf
```

> **Note:** If `APP_PUBLIC_KEY` is not set, a well-known test key is used
> automatically for `debug` and `cargo test` builds.  Release builds will fail
> at compile time without the variable.

### 6. Activate a license on the end-user machine

The application looks for `license.json` at the following platform-specific paths:

| Platform | Path |
|----------|------|
| Windows | `%APPDATA%\PdfEditor\license.json` |
| macOS | `~/Library/Application Support/PdfEditor/license.json` |
| Linux | `~/.config/pdfeditor/license.json` (or `$XDG_CONFIG_HOME/pdfeditor/license.json`) |

Rename the generated `.pdfeditor-license` file to `license.json` and copy it
to the appropriate path, **or** call `LicenseManager::activate()` from the
application to copy and validate it programmatically:

```rust
use licensing::LicenseManager;
let mut mgr = LicenseManager::new();
mgr.activate(std::path::Path::new("/path/to/acme_inc-commercial.pdfeditor-license"))?;
```

The application re-reads the new license immediately вЂ” no restart required.

---

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

1. **Register in Partner Center** вЂ” create a new app reservation at
   [Partner Center](https://partner.microsoft.com/dashboard) and note your
   *Publisher identity* (used as `PUBLISHER` above).

2. **Update store metadata** вЂ” edit `store/metadata.json` to set
   `windows_package_name` to the package name shown in Partner Center.

3. **Set the version** вЂ” bump `version` and `build_number` in
   `release/release.json`.

4. **Build and package**

   ```powershell
   $Env:WINDOWS_CERT_BASE64   = "<base64 PFX>"
   $Env:WINDOWS_CERT_PASSWORD = "<password>"
   $Env:PUBLISHER             = "CN=..."
   .\scripts\build_windows.ps1
   # Output: dist\windows\FreePDFEditor_<VERSION>.msix
   ```

   Set `SKIP_SIGNING=1` to build without signing (local testing only вЂ”
   Partner Center re-signs the package on ingestion, so you may omit signing
   for Store submissions if your Partner Center account supports it).

5. **Submit to the Store** вЂ” in Partner Center create a new submission, upload
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
| `APPLE_CERT_BASE64` | Base-64-encoded Distribution certificate (p12) вЂ” *"Apple Distribution: вЂ¦"* or *"3rd Party Mac Developer Application: вЂ¦"* |
| `APPLE_CERT_PASSWORD` | Certificate password |
| `APPLE_TEAM_ID` | 10-character Apple Developer Team ID |
| `APPLE_SIGN_IDENTITY` | Full common name of the signing certificate, e.g. `Apple Distribution: Your Name (TEAMID)` |
| `APPLE_INSTALLER_CERT_BASE64` | Base-64-encoded installer certificate (p12) вЂ” *"3rd Party Mac Developer Installer: вЂ¦"* |
| `APPLE_INSTALLER_CERT_PASSWORD` | Installer certificate password |
| `APPLE_INSTALLER_SIGN_IDENTITY` | Full common name of the installer certificate, e.g. `3rd Party Mac Developer Installer: Your Name (TEAMID)` |
| `APPLE_API_KEY_ID` | App Store Connect API key ID |
| `APPLE_API_ISSUER_ID` | App Store Connect API issuer ID |
| `APPLE_API_PRIVATE_KEY` | Contents of the `.p8` private key file |

#### Steps

1. **Create the app in App Store Connect** вЂ” go to
   [appstoreconnect.apple.com](https://appstoreconnect.apple.com), create a
   new macOS app, and note the *Bundle ID* (must match `bundle_id` in
   `store/metadata.json`, currently `com.freepdfeditor.app`).

2. **Create an App Store Connect API key** вЂ” in App Store Connect в†’ Users and
   Access в†’ Integrations в†’ App Store Connect API, generate a key with
   *Developer* or *Admin* role and download the `.p8` file.

3. **Export certificates from Xcode** вЂ” in Xcode в†’ Settings в†’ Accounts в†’
   Manage Certificates, create and export:
   - *Apple Distribution* (application signing)
   - *3rd Party Mac Developer Installer* (package signing)

4. **Update store metadata and version**

   ```bash
   # store/metadata.json  в†’ set bundle_id, display_name, description
   # release/release.json в†’ bump version and build_number
   ```

5. **Build a universal binary** (skip signing вЂ” the next step handles it)

   ```bash
   SKIP_SIGNING=1 bash scripts/build_macos.sh
   # Output: dist/macos/FreePDFEditor.app
   ```

6. **Sign and package**

   ```bash
   # Code-sign the .app bundle for Mac App Store distribution
   export APPLE_CERT_BASE64="<base64 p12>"
   export APPLE_CERT_PASSWORD="<password>"
   export APPLE_TEAM_ID="<TEAMID>"
   export APPLE_SIGN_IDENTITY="Apple Distribution: Your Name (TEAMID)"
   bash scripts/sign_macos.sh

   # Build a signed .pkg for Mac App Store submission (no notarization step here)
   export APPLE_INSTALLER_CERT_BASE64="<base64 installer p12>"
   export APPLE_INSTALLER_CERT_PASSWORD="<password>"
   export APPLE_INSTALLER_SIGN_IDENTITY="3rd Party Mac Developer Installer: Your Name (TEAMID)"

   # Example: create the installer package with productbuild
   # (adjust paths / identifiers as needed)
   security import <(echo "$APPLE_INSTALLER_CERT_BASE64" | base64 --decode) -P "$APPLE_INSTALLER_CERT_PASSWORD" -A
   productbuild \
     --component "dist/macos/FreePDFEditor.app" /Applications \
     --sign "$APPLE_INSTALLER_SIGN_IDENTITY" \
     "dist/macos/FreePDFEditor_<VERSION>.pkg"
   ```

7. **Upload to App Store Connect** вЂ” use Apple's *Transporter* app
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

   Alternatively, open **Xcode в†’ Organizer в†’ Distribute App** and follow the
   guided upload workflow.

8. **Submit for review** вЂ” in App Store Connect, select the uploaded build,
   complete the required metadata (screenshots, description, privacy details),
   and click **Submit for Review**.

---

## Extensibility

Every new feature follows the same pattern:

1. Define a struct that implements `DocumentCommand` (`execute` + `undo` + `description`).
2. Emit the appropriate `DocumentEvent` variant from the controller.
3. Wire a Qt/QML signal-handler (or Rust bridge callback) to trigger the command.

No business logic lives in the UI layer.

## Gap analysis vs. full specification

The table below maps each requirement from the product specification to its
current implementation status.

### Rendering engine

| Requirement | Status | Notes |
|-------------|:------:|-------|
| Page rasterization | вњ… | `MuPdfRenderer::render_from_path` opens a `mupdf::Document`, calls `page.to_pixmap()`, and converts the RGB pixmap to RGBA8. |
| Zoom levels | вњ… | 0.1 Г— вЂ“ 10 Г— |
| LRU page cache | вњ… | Keyed by `(doc_id, page, zoom)`; shared via `Arc<Mutex<PageCache>>` with the render worker |
| Text extraction | вњ… | Via lopdf (command layer) and MuPDF `TextPage` blocks (renderer) |
| Coordinate mapping | вњ… | `MediaBox`-based |
| MuPDF as rendering backend | вњ… | `MuPdfRenderer` uses `mupdf` 0.6 (wraps libmupdf 1.23) for rasterization and text-box extraction. lopdf is retained for document editing commands. |

### Document engine

| Requirement | Status | Notes |
|-------------|:------:|-------|
| Open / save PDF | вњ… | |
| Incremental saves | вњ… | `save_incremental` / `save_incremental_to` use `lopdf::IncrementalDocument` to append a new xref section without rewriting the full file. Falls back to a full save for freshly-created documents. |
| Undo / redo | вњ… | `CommandHistory` with configurable depth |
| Page tree management | вњ… | |

### Editing engine

| Requirement | Status | Notes |
|-------------|:------:|-------|
| Insert text | вњ… | `InsertTextCommand` |
| Modify existing text | вњ… | `ModifyTextCommand` вЂ” decompresses content streams, replaces literal-string occurrences of the target text in `Tj`/`TJ` operators, and re-encodes the result as a merged stream. |
| Font substitution | вњ… | `FontSubstitutionCommand` вЂ” replaces `Tf` font-name operands in all content streams on a page and auto-adds standard Type1 font entries to `/Resources/Font`. |
| Insert image | вњ… | `InsertImageCommand` вЂ” embeds a raw RGB bitmap as an uncompressed `DeviceRGB` PDF Image XObject with undo support |
| Replace / resize image | вњ… | `ReplaceImageCommand` вЂ” replaces the pixel data and intrinsic dimensions of an existing Image XObject identified by resource name; optionally updates the on-page `cm` transform for display resizing. |
| Delete / rotate / reorder pages | вњ… | |
| Merge documents | вњ… | |

### Annotation system

| Requirement | Status | Notes |
|-------------|:------:|-------|
| Highlight, Underline, Strikeout | вњ… | |
| Notes (sticky notes) | вњ… | |
| Drawing paths (ink) | вњ… | |
| Stamps | вњ… | |

### Forms engine

| Requirement | Status | Notes |
|-------------|:------:|-------|
| Detect form fields | вњ… | All AcroForm field types |
| Edit field values | вњ… | `SetFieldValueCommand` with undo |
| Create new form fields | вњ… | `CreateFieldCommand` вЂ” creates TextField, Checkbox, Radio, Dropdown, or SignatureField; creates AcroForm if absent; undo supported |
| Export form data (JSON) | вњ… | `export_form_data` |

### OCR

| Requirement | Status | Notes |
|-------------|:------:|-------|
| `OcrProvider` abstraction | вњ… | Trait + `OcrResult` / `TextRegion` types |
| `NoOpOcrProvider` stub | вњ… | Zero-dependency placeholder; returns empty results |
| `ApplyOcrCommand` | вњ… | Embeds pre-computed OCR regions as an invisible text layer (render mode 3) on a PDF page; font registered in `/Resources/Font` |
| `TesseractOcrProvider` | вњ… | `pdf-ocr` crate вЂ” Tesseract 5.x backed `OcrProvider`; parses TSV word-level bounding boxes; converts pixel coordinates to PDF points via configurable DPI |

### Security

| Requirement | Status | Notes |
|-------------|:------:|-------|
| Password protection | вњ… | `SetPasswordCommand` applies RC4-128 encryption via `lopdf::Document::encrypt` with `EncryptionVersion::V2`; injects `/ID` trailer entry when absent. |
| Permissions | вќЊ Not started | |
| Redaction | вњ… | `RedactRegionCommand` now performs **true redaction**: decompresses all content streams, parses `BTвЂ¦ET` text blocks, removes blocks whose text position falls within the target rectangle, then re-encodes the result into a single filtered stream. A filled black rectangle is added on top as a visual marker. Falls back to visual-only if content stream parsing fails. |

### Performance targets

| Requirement | Status | Notes |
|-------------|:------:|-------|
| Memory-safe LRU cache | вњ… | |
| Background rendering (off UI thread) | вњ… | Dedicated `render-worker` thread; `MuPdfRenderer::render_from_path` runs off the UI thread; Qt event bridge parity is in migration. |
| Lazy page loading | вќЊ Not started | `Document::open` loads the full lopdf object graph at open time. |
| `<100 ms` page navigation latency | вќЊ Not measured | Achievable with real rendering (MuPDF) once integrated. |
| Incremental saves | вњ… | `save_incremental` / `save_incremental_to` implemented. |

### Plugin system

| Requirement | Status | Notes |
|-------------|:------:|-------|
| `Plugin` trait | вњ… | `name()`, `on_load()`, `on_unload()` |
| `PluginContext` | вњ… | Exposes `EventBus` and tool registry |
| `PluginRegistry` | вњ… | Load / unload lifecycle |
| Runtime-loadable plugins (dylib) | вќЊ Not started | Spec notes "design only" for this phase |

### Architecture compliance

| Principle | Status | Notes |
|-----------|:------:|-------|
| Core is UI-agnostic | вњ… | |
| UI communicates via commands / events | вњ… | |
| PDF manipulation independent of UI | вњ… | |
| Features as independent modules | вњ… | One crate per feature area |
| Trait-based abstractions | вњ… | `DocumentCommand`, `RenderEngine`, `OcrProvider`, `Plugin` |
| No global state | вњ… | |
| Workspace layout matches spec | вњ… | `pdf-core / pdf-render / pdf-editor / pdf-annotations / pdf-forms / app-desktop` |
| Async Rust | вќЊ Not started | Spec lists async as part of the stack; currently all synchronous (background rendering uses OS threads, not async/await) |

**Legend:** вњ… Implemented В· вљ пёЏ Partial / placeholder В· вќЊ Not started В· рџ”І Intentionally deferred








