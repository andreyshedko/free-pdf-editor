# Desktop E2E (Windows)

This folder contains desktop end-to-end smoke tests for `pdf-editor.exe`
using Appium + Windows UI Automation (WinAppDriver backend).

## Prerequisites

1. Install Node.js 18+
2. Install Appium 2 globally:
   - `npm i -g appium`
3. Install Appium Windows driver:
   - `appium driver install windows`
4. Install and run WinAppDriver (as Administrator):
   - default URL: `http://127.0.0.1:4723`

## Install dependencies

From repo root:

```powershell
Push-Location tests/e2e
npm install
Pop-Location
```

## Run smoke test

```powershell
Push-Location tests/e2e
npm run test:smoke
Pop-Location
```

## Run regression scenarios

```powershell
Push-Location tests/e2e
npm run test:regression
Pop-Location
```

Current regression scenarios:

- Open Recent submenu visibility (localized labels)
- Open fixture document via Open Recent entry (`e2e_sample.pdf`)
- Click fixture text and verify Text panel switches to Save mode
- In Save mode, save edited text and verify action remains Save
- Open Recent stability across repeated open cycles
- Text panel controls presence contract (Close + Save/Insert)

Regression setup details:

- The test creates `tests/e2e/.tmp/e2e_sample.pdf` automatically.
- The test seeds `%APPDATA%\\free-pdf-editor\\recent_documents.txt` with this fixture.
- `scripts/run_e2e_windows.ps1` sets an isolated APPDATA sandbox under `tests/e2e/.tmp/appdata` so local user recents are not modified.

Environment variables:

- `APP_EXE` - path to app executable (default: `target/debug/pdf-editor.exe`)
- `APPIUM_HOST` - Appium host (default: `127.0.0.1`)
- `APPIUM_PORT` - Appium port (default: `4723`)
- `APPDATA` - optional custom APPDATA root (defaults to user profile; runner overrides with sandbox)
