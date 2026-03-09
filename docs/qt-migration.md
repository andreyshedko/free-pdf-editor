# Qt Migration Plan

## Goal
Replace the current Slint desktop UI with a Qt-based frontend while preserving existing document/editing logic.

## Current State
- `app-desktop` now uses a Qt/QML shell as the primary `pdf-editor` binary:
  - Top actions: `Open`, `Save`, `Insert Text`
  - Canvas placeholder
  - Status bar
- Legacy Slint modules still exist in source tree but are no longer wired from `main.rs`.
- `app-desktop-qt/` remains as an isolated staging sandbox.

## Migration Phases
1. UI bridge layer
- Extract an interface from `app-desktop/src/controller.rs` so it does not depend directly on Slint `AppWindow`.
- Add a Qt window adapter that implements this interface.

2. Event/callback wiring
- Port callbacks currently registered in `wire_callbacks()`:
  - document open/save/close
  - page navigation/zoom
  - canvas click/drag/double-click
  - text/image edit actions

3. Canvas and overlays
- Replace Slint canvas with Qt Quick area:
  - page bitmap display
  - text selection rectangle
  - image selection/overlay controls
  - coordinate transforms (`canvas <-> pdf`)

4. Panels and menus
- Port text edit panel, image edit panel, and menu actions.

5. Remove Slint path (cleanup)
- Delete unused Slint UI files/modules after Qt parity is reached.

## How to run the Qt shell
From repo root:

```powershell
$env:QMAKE='C:\Qt\6.10.2\mingw_64\bin\qmake.exe'
$env:PATH='C:\Qt\Tools\mingw1310_64\bin;C:\Qt\6.10.2\mingw_64\bin;C:\msys64\usr\bin;'+$env:PATH
cargo run -p app-desktop --target x86_64-pc-windows-gnu --features mupdf
```

Notes:
- Current machine has Qt MinGW kit (`mingw_64`), so Rust target must be
  `x86_64-pc-windows-gnu` (MSVC target is incompatible with MinGW Qt).

- For real PDF preview, build/run with --features mupdf (requires make in PATH on MinGW).
