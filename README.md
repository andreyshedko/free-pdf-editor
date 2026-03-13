# Free PDF Editor (Desktop)

A cross-platform **C++20 + Qt 6 Widgets** desktop PDF editor with overlay-based editing, undo/redo support, and multi-language UI.


## C++/Qt Application (Next Generation)

This is the next-generation, cross-platform desktop PDF editor built with **C++20** and **Qt 6 Widgets**. The source code is located in the `src/` directory and is built with CMake.

### C++ Architecture

The C++ application is structured into the following modules:

-   `app`: Main application entry point and event loop.
-   `ui`: Qt Widgets-based UI components, including the main window, toolbar, and panels.
-   `editor`: Manages the application state, including the undo/redo stack (`UndoStack`) and user interactions (`EditorController`).
-   `document`: Handles the core PDF document structure (`Document`, `PageModel`).
-   `pdf_engine`: A bridge to the PDFium library for rendering (`PdfRenderer`) and writing (`PdfWriter`).
-   `cache`: Provides a `PageRenderCache` for efficient page rendering.
-   `overlay`: Manages UI objects for annotations, text, and selections.
-   `ocr`: Provides OCR capabilities via `OCRProcessor`.
-   `utils`: Contains shared utilities for logging and file operations.

### Build & Run (C++)

#### Windows Quick Start (Current Setup)

This machine has Qt MinGW installed (`C:\Qt\6.10.2\mingw_64`) and no MSVC Qt kit.
Use this flow:

```powershell
$env:PATH = "C:\Qt\Tools\mingw1310_64\bin;" + $env:PATH
cmake -B build_mingw -S . -G "MinGW Makefiles" -DCMAKE_CXX_COMPILER="C:/Qt/Tools/mingw1310_64/bin/g++.exe"
cmake --build build_mingw -j 8
C:/Qt/6.10.2/mingw_64/bin/windeployqt.exe --release ./build_mingw/src/pdf-editor.exe
# Optional but recommended for real PDF rendering (instead of placeholder pages):
# copy .\pdfium.dll .\build_mingw\src\pdfium.dll
# or set: $env:PDFIUM_DLL = "C:\full\path\to\pdfium.dll"
./build_mingw/src/pdf-editor.exe
```

**Prerequisites:**
- CMake 3.20 or later
- Qt 6.5+ (with Widgets, Gui, Core modules)
- C++20 compatible compiler (MSVC, GCC/MinGW, or Clang)

The CMakeLists.txt auto-detects Qt6 on Windows by scanning common install paths (`C:/Qt/*`, environment variables `$QTDIR`, `$QT_DIR`).

On Windows, your compiler and Qt kit must match:
- MSVC compiler <-> Qt `msvc*` kit (for example, `msvc2022_64`)
- MinGW compiler <-> Qt `mingw*` kit (for example, `mingw_64`)

#### Linux / macOS

```bash
# Debug build
cmake -B build -S . -DCMAKE_BUILD_TYPE=Debug
cmake --build build -j$(nproc)

# Release build
cmake -B build -S . -DCMAKE_BUILD_TYPE=Release
cmake --build build -j$(nproc)

# Run
./build/src/pdf-editor
```

#### Windows with MSVC (Visual Studio)

```powershell
# Debug build
cmake -B build -S . -G "Visual Studio 17 2022"
cmake --build build --config Debug -j 8

# Release build
cmake -B build -S . -G "Visual Studio 17 2022"
cmake --build build --config Release -j 8

# Run
./build/src/Release/pdf-editor.exe
```

#### Windows with Qt MinGW (bundled with Qt)

If you have Qt installed with MinGW support (e.g., `C:\Qt\6.10.2\mingw_64`):

```powershell
# Add MinGW to PATH
$env:PATH = "C:\Qt\Tools\mingw1310_64\bin;" + $env:PATH

# Debug build
cmake -B build_mingw -S . -G "MinGW Makefiles" `
  -DCMAKE_CXX_COMPILER="C:/Qt/Tools/mingw1310_64/bin/g++.exe"
cmake --build build_mingw -j 8

# Release build (add -DCMAKE_BUILD_TYPE=Release)
cmake -B build_mingw -S . -G "MinGW Makefiles" `
  -DCMAKE_CXX_COMPILER="C:/Qt/Tools/mingw1310_64/bin/g++.exe" `
  -DCMAKE_BUILD_TYPE=Release
cmake --build build_mingw -j 8

# Copy Qt DLLs/plugins beside the executable (recommended once after build)
C:/Qt/6.10.2/mingw_64/bin/windeployqt.exe --release ./build_mingw/src/pdf-editor.exe

# Optional but recommended for real PDF rendering (instead of placeholder pages):
# copy .\pdfium.dll .\build_mingw\src\pdfium.dll
# or set: $env:PDFIUM_DLL = "C:\full\path\to\pdfium.dll"

# Run
./build_mingw/src/pdf-editor.exe
```

**Note:** The build system auto-detects Qt6 on Windows. If CMake cannot find it, set `Qt6_DIR` manually to the kit that matches your compiler:
```powershell
-DQt6_DIR="C:/Qt/6.10.2/mingw_64/lib/cmake/Qt6"
# For MSVC builds, use an MSVC Qt path, for example:
# -DQt6_DIR="C:/Qt/6.10.2/msvc2022_64/lib/cmake/Qt6"
```

---

## Features

- **Multi-language UI** � Automatic language detection (English, Spanish, French, German, Czech, Polish, Russian)
- **Overlay-Based Editing** � Text, images, annotations as Qt graphics objects with real-time rendering
- **Undo/Redo Stack** � Full command pattern support for all editing operations
- **Page Management** � Insert, delete, merge, split PDF documents
- **Text Editing** � Edit text, change fonts, adjust font sizes  
- **Image Handling** � Insert, move, and resize images
- **Annotations** � Support for highlights, underlines, strikeouts, notes
- **Export** � Export pages as PNG images or plain text
- **Signatures** � Digital signature support
- **Security** � Password protection and content redaction

## Project Structure

```
src/
+-- app/              # Application entry point
+-- ui/               # Qt Widgets UI (MainWindow, PageView, Panels)
+-- editor/           # EditorController and command pattern
+-- document/         # Document model and page management
+-- pdf_engine/       # PDF rendering and writing (QPdfWriter)
+-- overlay/          # Interactive PDF overlays (text, images, shapes)
+-- cache/            # Page render caching
+-- ocr/              # OCR functionality
+-- utils/            # Utilities (logging, file operations)
L-- translations/     # Translation files (.ts) for i18n
```

## Development

### Code Organization

- **Overlay Model** � UI objects (TextEditObject, ImageObject, etc.) on top of rendered PDF pages
- **EditorController** � Central API coordinating document, PDF engine, overlays, and undo stack
- **Command Pattern** � All mutations go through Command subclasses for undo/redo support
- **Qt Signals/Slots** � Loose coupling between UI and business logic

### Building with Translations

The build system automatically:
1. Compiles `.ts` (translation source) files > `.qm` (compiled translation) files
2. Embeds `.qm` files as resources in the executable
3. App loads translations at runtime based on system locale

### Editable Overlay Metadata

When you save a PDF from the editor, the application may also create a sidecar file next to it:

```text
your-file.fpe.json
```

This file stores editor-specific metadata required to reopen overlays as editable objects, including:
- annotations
- text overlays
- image overlays
- shapes
- object positions and sizes

Without this sidecar file, the PDF can still be opened and viewed, but overlays that were flattened into the saved PDF may no longer be restored as editable objects on reopen.

The UI now distinguishes between two save flows:
- `Save Editable PDF` / `Save Editable PDF As...` writes the PDF and the editable metadata sidecar, preserving overlay editability on reopen.
- `Export As -> Flat PDF...` writes only a plain PDF without editable overlay metadata.

When a document is open, the status bar shows whether editable metadata is currently available for that file.

### Common Tasks

**Run the application:**
```bash
./build_mingw/src/pdf-editor.exe    # Windows
./build/src/pdf-editor              # Linux/macOS
```

**Clean rebuild:**
```bash
rm -rf build_mingw && cmake -B build_mingw -S . -G "MinGW Makefiles" \
  -DCMAKE_CXX_COMPILER="C:/Qt/Tools/mingw1310_64/bin/g++.exe"
cmake --build build_mingw -j4
```

## Dependencies

- **Qt 6.5+** (Core, Gui, Widgets, LinguistTools)
- **C++20 compiler** (MSVC, GCC/MinGW, Clang)
- **CMake 3.20+**

Optional:
- **PDFium runtime DLL** (`pdfium.dll`) for real PDF rendering on Windows.
  Without it, the app shows placeholder page previews with a PDFium unavailable message.
- **Tesseract** (for OCR functionality)

## License

This project is provided as-is. See [LICENSE](LICENSE) file for details.

## Contributing

While this is a reference implementation, contributions and improvements are welcome. Please ensure:
- C++20 compliance
- Cross-platform compatibility (Windows, macOS, Linux)
- All new strings use `tr()` macro for translation support
- Undo/redo support for all document mutations

## Legacy Rust Implementation

The project originally started with a Rust backend and has been migrated to C++ with Qt Widgets. The previous Rust implementation details, build instructions, and feature documentation have been archived. Refer to earlier git commits for historical reference.

