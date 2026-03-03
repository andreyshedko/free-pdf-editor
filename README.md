# Free PDF Editor (Desktop)

Cross-platform offline-first desktop PDF editor built with **Rust**, **Slint** UI, and **MuPDF**.

## Stack

| Layer | Technology |
|-------|------------|
| Language | Rust (edition 2021) |
| UI | [Slint](https://slint.dev) |
| PDF engine | MuPDF (via Rust bindings, feature-gated) |
| Build | Cargo workspace |
| Targets | Windows · macOS · Linux |

## Architecture

The project is a Cargo workspace with six crates:

```
app ──► ui ──► shared
 │             ▲
 ▼             │
core ──────────┤
 │             │
 ▼             │
pdf-engine ────┘
      │
      ▼
   (MuPDF)
```

| Crate | Purpose | Thread |
|-------|---------|--------|
| `shared` | `Command`, `Event`, error types | any (`Send+Sync`) |
| `pdf-engine` | Safe MuPDF wrapper / stub renderer | worker threads only |
| `platform` | File dialogs, clipboard, OS services | UI thread |
| `core` | `AppState`, `CoreLoop`, LRU page cache | core-loop thread |
| `ui` | Slint window, `AppController` | UI thread |
| `app` | Entry point, thread spawning | main thread |

### Thread model

```
UI thread            cmd_tx ──► core-loop thread ──► render worker threads
(Slint event loop)              (AppState, cache)     (MuPDF rendering)
       ▲                                │
       └────── event-bridge ────────────┘  (invoke_from_event_loop)
```

- The **UI thread** handles input, layout, and display updates only.
- The **core-loop thread** owns `AppState` and processes all `Command`s.
- **Worker threads** run expensive MuPDF rendering and text extraction.
- MuPDF **never** runs on the UI thread.
- Communication uses `std::sync::mpsc` channels (no shared mutable state).

### Render pipeline

1. UI sends `Command::ViewportChanged` + `Command::RenderVisiblePages`.
2. Core computes visible page indices via viewport math.
3. Core checks the LRU cache (`PageCacheKey = (doc_id, page, zoom)`).
4. Cache miss → render via `pdf-engine::PdfDocument::render_page`.
5. Bitmap returned as `Event::PageRendered { data, width, height }`.
6. Event bridge posts the update to Slint via `invoke_from_event_loop`.

## Getting started

### Prerequisites

- Rust ≥ 1.75
- (Optional) MuPDF native library for real PDF rendering

### Build

```bash
cargo build
```

The default build uses a **stub PDF engine** (no native dependency).
To enable real MuPDF rendering (requires the `mupdf` system library):

```bash
cargo build --features pdf-engine/mupdf
```

### Run

```bash
cargo run --bin pdf-editor
```

### Test

```bash
cargo test
```

Tests cover:

- LRU cache eviction (`core::cache`)
- Command processing — open, zoom, render, page navigation (`core::command_loop`)
- Document manager — open/render/extract error handling (`pdf-engine`)
- `PageCacheKey` zoom encoding (`shared`)

## Extensibility

The `Command` and `Event` enums in `shared` are the single extension point.
New tools (annotation, redaction, forms, AI assistant) register by:

1. Adding variants to `Command` / `Event`.
2. Handling them in `CoreLoop`.
3. Wiring Slint callbacks in `AppController`.

No business logic lives in the UI layer.
