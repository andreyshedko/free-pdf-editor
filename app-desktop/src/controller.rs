use licensing::LicenseManager;
use pdf_annotations::{
    types::{Annotation, AnnotationKind, Color, Rect},
    AddAnnotationCommand,
};
use pdf_core::{
    command::{CommandHistory, DocumentCommand},
    document::Document,
    event::{DocumentEvent, EventBus},
    ocr::{OcrResult, TextRegion},
};
use pdf_editor::{
    ApplyOcrCommand, DeletePageCommand, FontSubstitutionCommand, InsertImageCommand,
    InsertTextCommand, MergeDocumentCommand, ModifyTextCommand, RedactRegionCommand,
    ReorderPagesCommand, ReplaceImageCommand, RotatePageCommand, SetPasswordCommand,
};
use pdf_forms::{
    detect_form_fields, export_form_data, CreateFieldCommand, FormFieldKind, FormFieldValue,
    SetFieldValueCommand,
};
#[cfg(feature = "mupdf")]
use pdf_render::MuPdfRenderer;
use pdf_render::{CacheKey, PageCache, RenderedPage, SoftwareRenderer};
use rfd::FileDialog;
use slint::{Image, Rgba8Pixel, SharedPixelBuffer, Weak};
use std::path::PathBuf;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{mpsc, mpsc::Sender, Arc, Mutex};
use std::thread;

use crate::AppWindow;

const CACHE_CAPACITY: usize = 50;
const HISTORY_DEPTH: usize = 100;

/// A render task submitted to the background render worker.
struct RenderTask {
    doc_id: u64,
    #[cfg_attr(not(feature = "mupdf"), allow(dead_code))]
    doc_path: std::path::PathBuf,
    page_index: u32,
    page_width: f64,
    page_height: f64,
    zoom: f32,
    page_count: u32,
    cache: Arc<Mutex<PageCache>>,
    window: Weak<AppWindow>,
    /// Shared generation counter.  The UI thread increments this whenever the
    /// "desired" render target changes (page nav, zoom, open/close).
    render_generation: Arc<AtomicU64>,
    /// The generation value at the time this task was submitted.  If
    /// `render_generation.load()` no longer equals `expected_generation` when
    /// the render completes, the result is a stale frame and is discarded.
    expected_generation: u64,
}

// `Weak<AppWindow>` is `Send` in Slint and `Arc<Mutex<PageCache>>` is
// `Send + Sync`, so `RenderTask` derives `Send` automatically.

pub struct AppController {
    window: Weak<AppWindow>,
    evt_tx: Sender<DocumentEvent>,
    document: Option<Document>,
    history: CommandHistory,
    cache: Arc<Mutex<PageCache>>,
    zoom: f32,
    current_page: u32,
    #[allow(dead_code)]
    bus: EventBus,
    /// Licensing state – derived from cryptographic validation at startup.
    license: LicenseManager,
    /// Channel to the background render worker thread.
    render_tx: mpsc::SyncSender<RenderTask>,
    /// Monotonically-increasing counter.  Incremented every time the desired
    /// render target changes so background results for stale targets can be
    /// detected and discarded before they reach the UI.
    render_generation: Arc<AtomicU64>,
}

impl AppController {
    fn pick_save_path(default_name: &str) -> Option<PathBuf> {
        FileDialog::new()
            .add_filter("PDF", &["pdf"])
            .set_file_name(default_name)
            .save_file()
    }

    pub fn new(window: Weak<AppWindow>, evt_tx: Sender<DocumentEvent>) -> Self {
        let cache = Arc::new(Mutex::new(PageCache::new(CACHE_CAPACITY)));
        let render_tx = spawn_render_worker();
        let license = LicenseManager::new();
        Self {
            window,
            evt_tx,
            document: None,
            history: CommandHistory::new(HISTORY_DEPTH),
            cache,
            zoom: 1.0,
            current_page: 0,
            bus: EventBus::new(),
            license,
            render_tx,
            render_generation: Arc::new(AtomicU64::new(0)),
        }
    }

    pub fn wire_callbacks(&mut self) {
        // SAFETY: Slint runs all callbacks on the same GUI thread that owns the window and
        // this AppController.  AppController's lifetime is tied to the stack frame of main(),
        // which outlives the window.  No re-entrancy occurs because Slint serialises callbacks.
        let ptr = self as *mut AppController;

        let win = match self.window.upgrade() {
            Some(w) => w,
            None => return,
        };

        win.on_open_document(move || {
            let me = unsafe { &mut *ptr };
            me.open_document_dialog();
        });

        win.on_save_document(move || {
            let me = unsafe { &mut *ptr };
            me.save_document();
        });

        win.on_save_document_as(move || {
            let me = unsafe { &mut *ptr };
            me.save_document_as_dialog();
        });

        win.on_close_document(move || {
            let me = unsafe { &mut *ptr };
            me.close_document();
        });

        win.on_zoom_in(move || {
            let me = unsafe { &mut *ptr };
            me.set_zoom(me.zoom * 1.25);
        });

        win.on_zoom_out(move || {
            let me = unsafe { &mut *ptr };
            me.set_zoom(me.zoom * 0.8);
        });

        win.on_zoom_reset(move || {
            let me = unsafe { &mut *ptr };
            me.set_zoom(1.0);
        });

        win.on_next_page(move || {
            let me = unsafe { &mut *ptr };
            if let Some(doc) = &me.document {
                let count = doc.page_count();
                if me.current_page + 1 < count {
                    me.current_page += 1;
                    me.render_current_page();
                    me.emit(DocumentEvent::PageChanged {
                        index: me.current_page,
                    });
                }
            }
        });

        win.on_prev_page(move || {
            let me = unsafe { &mut *ptr };
            if me.current_page > 0 {
                me.current_page -= 1;
                me.render_current_page();
                me.emit(DocumentEvent::PageChanged {
                    index: me.current_page,
                });
            }
        });

        win.on_undo(move || {
            let me = unsafe { &mut *ptr };
            me.undo();
        });

        win.on_redo(move || {
            let me = unsafe { &mut *ptr };
            me.redo();
        });

        win.on_add_highlight(move || {
            let me = unsafe { &mut *ptr };
            me.add_highlight_annotation();
        });

        win.on_add_note(move || {
            let me = unsafe { &mut *ptr };
            me.add_note_annotation();
        });

        win.on_delete_current_page(move || {
            let me = unsafe { &mut *ptr };
            me.delete_current_page();
        });

        win.on_rotate_current_page(move || {
            let me = unsafe { &mut *ptr };
            me.rotate_current_page();
        });

        win.on_upgrade_license(move || {
            let me = unsafe { &mut *ptr };
            me.emit(DocumentEvent::StatusChanged {
                message: "Visit https://example.com/upgrade to purchase a commercial license."
                    .into(),
            });
        });

        win.on_tools_insert_text(move || {
            let me = unsafe { &mut *ptr };
            me.tool_insert_text();
        });

        win.on_tools_modify_text(move || {
            let me = unsafe { &mut *ptr };
            me.tool_modify_text();
        });

        win.on_tools_font_substitute(move || {
            let me = unsafe { &mut *ptr };
            me.tool_font_substitution();
        });

        win.on_tools_insert_image(move || {
            let me = unsafe { &mut *ptr };
            me.tool_insert_image();
        });

        win.on_tools_replace_image(move || {
            let me = unsafe { &mut *ptr };
            me.tool_replace_image();
        });

        win.on_tools_set_password(move || {
            let me = unsafe { &mut *ptr };
            me.tool_set_password();
        });

        win.on_tools_redact_region(move || {
            let me = unsafe { &mut *ptr };
            me.tool_redact_region();
        });

        win.on_tools_apply_ocr(move || {
            let me = unsafe { &mut *ptr };
            me.tool_apply_ocr();
        });

        win.on_tools_reorder_pages(move || {
            let me = unsafe { &mut *ptr };
            me.tool_reorder_pages();
        });

        win.on_tools_merge_document(move || {
            let me = unsafe { &mut *ptr };
            me.tool_merge_document();
        });

        win.on_tools_create_field(move || {
            let me = unsafe { &mut *ptr };
            me.tool_create_field();
        });

        win.on_tools_set_field_value(move || {
            let me = unsafe { &mut *ptr };
            me.tool_set_field_value();
        });

        win.on_tools_detect_fields(move || {
            let me = unsafe { &mut *ptr };
            me.tool_detect_fields();
        });

        win.on_tools_export_form_data(move || {
            let me = unsafe { &mut *ptr };
            me.tool_export_form_data();
        });

        win.on_activate_license(move || {
            let me = unsafe { &mut *ptr };
            me.activate_license_dialog();
        });

        // Display the initial license state in the UI.
        self.update_license_display();
    }

    fn open_document_dialog(&mut self) {
        let path = FileDialog::new()
            .add_filter("PDF", &["pdf"])
            .pick_file()
            .or_else(|| std::env::var("OPEN_PDF").ok().map(std::path::PathBuf::from));

        let Some(path) = path else {
            self.emit(DocumentEvent::StatusChanged {
                message: "Open canceled (optionally set OPEN_PDF as fallback)".into(),
            });
            return;
        };

        match Document::open(&path) {
            Ok(doc) => {
                let title = doc.title.clone();
                let page_count = doc.page_count();
                self.document = Some(doc);
                self.current_page = 0;
                self.history.clear();
                self.cache
                    .lock()
                    .expect("PageCache mutex was poisoned")
                    .evict_document(self.document.as_ref().unwrap().id);
                self.render_current_page();
                self.emit(DocumentEvent::DocumentOpened { title, page_count });
                #[cfg(not(feature = "mupdf"))]
                self.emit(DocumentEvent::StatusChanged {
                    message: "This build uses a placeholder renderer. Rebuild with --features mupdf for full PDF preview.".into(),
                });
                self.update_undo_redo_state();
            }
            Err(e) => {
                self.emit(DocumentEvent::Error {
                    message: e.to_string(),
                });
            }
        }
    }

    fn save_document(&mut self) {
        // Check license before borrowing document mutably.
        let needs_watermark_notice = !self.license.is_commercial_allowed();
        if needs_watermark_notice {
            self.emit(DocumentEvent::StatusChanged {
                message: "Saving with personal license – exported PDF may contain watermark for commercial use.".into(),
            });
        }
        if let Some(doc) = &mut self.document {
            // If the document has no concrete path yet, ask the user where to save it.
            if doc.path.as_os_str().is_empty() {
                let default_name = format!("{}.pdf", doc.title);
                let Some(path) = Self::pick_save_path(&default_name) else {
                    self.emit(DocumentEvent::StatusChanged {
                        message: "Save canceled".into(),
                    });
                    return;
                };
                match doc.save_to(&path) {
                    Ok(()) => self.emit(DocumentEvent::DocumentSaved {
                        path: path.display().to_string(),
                    }),
                    Err(e) => self.emit(DocumentEvent::Error {
                        message: e.to_string(),
                    }),
                }
            } else {
                match doc.save() {
                    Ok(()) => {
                        let path = doc.path.display().to_string();
                        self.emit(DocumentEvent::DocumentSaved { path });
                    }
                    Err(e) => self.emit(DocumentEvent::Error {
                        message: e.to_string(),
                    }),
                }
            }
        }
    }

    fn save_document_as_dialog(&mut self) {
        let Some(default_name) = self
            .document
            .as_ref()
            .map(|d| format!("{}.pdf", d.title))
        else {
            self.emit(DocumentEvent::StatusChanged {
                message: "No document is open".into(),
            });
            return;
        };

        let Some(path) = Self::pick_save_path(&default_name) else {
            self.emit(DocumentEvent::StatusChanged {
                message: "Save As canceled".into(),
            });
            return;
        };

        // Check license before borrowing document mutably.
        let needs_watermark_notice = !self.license.is_commercial_allowed();
        if needs_watermark_notice {
            self.emit(DocumentEvent::StatusChanged {
                message: "Saving with personal license – exported PDF may contain watermark for commercial use.".into(),
            });
        }
        if let Some(doc) = &mut self.document {
            match doc.save_to(&path) {
                Ok(()) => self.emit(DocumentEvent::DocumentSaved {
                    path: path.display().to_string(),
                }),
                Err(e) => self.emit(DocumentEvent::Error {
                    message: e.to_string(),
                }),
            }
        }
    }

    fn close_document(&mut self) {
        if let Some(doc) = self.document.take() {
            self.cache
                .lock()
                .expect("PageCache mutex was poisoned")
                .evict_document(doc.id);
        }
        self.current_page = 0;
        self.history.clear();
        self.emit(DocumentEvent::DocumentClosed);
    }

    fn set_zoom(&mut self, zoom: f32) {
        let zoom = zoom.clamp(0.1, 10.0);
        if let Some(doc) = &self.document {
            self.cache
                .lock()
                .expect("PageCache mutex was poisoned")
                .evict_document(doc.id);
        }
        self.zoom = zoom;
        self.render_current_page();
        self.emit(DocumentEvent::ZoomChanged { factor: zoom });
    }

    fn undo(&mut self) {
        if let Some(doc) = &mut self.document {
            if let Err(e) = self.history.undo(doc) {
                self.emit(DocumentEvent::Error {
                    message: e.to_string(),
                });
            } else {
                self.render_current_page();
                self.update_undo_redo_state();
            }
        }
    }

    fn redo(&mut self) {
        if let Some(doc) = &mut self.document {
            if let Err(e) = self.history.redo(doc) {
                self.emit(DocumentEvent::Error {
                    message: e.to_string(),
                });
            } else {
                self.render_current_page();
                self.update_undo_redo_state();
            }
        }
    }

    fn add_highlight_annotation(&mut self) {
        if self.document.is_none() {
            return;
        }
        let annotation = Annotation::new(
            self.current_page,
            Rect {
                x: 72.0,
                y: 700.0,
                width: 200.0,
                height: 20.0,
            },
            AnnotationKind::Highlight {
                color: Color::yellow(),
            },
        );
        let id = annotation.id.0.clone();
        let cmd = Box::new(AddAnnotationCommand::new(annotation));
        if let Some(doc) = &mut self.document {
            match self.history.execute(cmd, doc) {
                Ok(()) => {
                    self.emit(DocumentEvent::AnnotationAdded {
                        page: self.current_page,
                        annotation_id: id,
                    });
                    self.update_undo_redo_state();
                }
                Err(e) => self.emit(DocumentEvent::Error {
                    message: e.to_string(),
                }),
            }
        }
    }

    fn add_note_annotation(&mut self) {
        if self.document.is_none() {
            return;
        }
        let annotation = Annotation::new(
            self.current_page,
            Rect {
                x: 500.0,
                y: 750.0,
                width: 20.0,
                height: 20.0,
            },
            AnnotationKind::Note {
                author: "User".into(),
                content: "Note added by editor".into(),
            },
        );
        let id = annotation.id.0.clone();
        let cmd = Box::new(AddAnnotationCommand::new(annotation));
        if let Some(doc) = &mut self.document {
            match self.history.execute(cmd, doc) {
                Ok(()) => {
                    self.emit(DocumentEvent::AnnotationAdded {
                        page: self.current_page,
                        annotation_id: id,
                    });
                    self.update_undo_redo_state();
                }
                Err(e) => self.emit(DocumentEvent::Error {
                    message: e.to_string(),
                }),
            }
        }
    }

    fn delete_current_page(&mut self) {
        if self.document.is_none() {
            return;
        }
        let page = self.current_page;
        let cmd = Box::new(DeletePageCommand::new(page));
        if let Some(doc) = &mut self.document {
            match self.history.execute(cmd, doc) {
                Ok(()) => {
                    let count = doc.page_count();
                    if self.current_page >= count && count > 0 {
                        self.current_page = count - 1;
                    }
                    self.render_current_page();
                    self.emit(DocumentEvent::PageDeleted { index: page });
                    self.update_undo_redo_state();
                }
                Err(e) => self.emit(DocumentEvent::Error {
                    message: e.to_string(),
                }),
            }
        }
    }

    fn rotate_current_page(&mut self) {
        if self.document.is_none() {
            return;
        }
        let page = self.current_page;
        let cmd = Box::new(RotatePageCommand::new(page, 90));
        if let Some(doc) = &mut self.document {
            match self.history.execute(cmd, doc) {
                Ok(()) => {
                    if let Some(ref doc) = self.document {
                        self.cache
                            .lock()
                            .expect("PageCache mutex was poisoned")
                            .evict_document(doc.id);
                    }
                    self.render_current_page();
                    self.emit(DocumentEvent::PageRotated {
                        index: page,
                        angle: 90,
                    });
                    self.update_undo_redo_state();
                }
                Err(e) => self.emit(DocumentEvent::Error {
                    message: e.to_string(),
                }),
            }
        }
    }

    fn ensure_document_open(&self) -> bool {
        if self.document.is_none() {
            self.emit(DocumentEvent::StatusChanged {
                message: "Open a document first".into(),
            });
            return false;
        }
        true
    }

    fn run_tool_command(&mut self, cmd: Box<dyn DocumentCommand>, success_message: &str) {
        if !self.ensure_document_open() {
            return;
        }
        if let Some(doc) = &mut self.document {
            match self.history.execute(cmd, doc) {
                Ok(()) => {
                    self.render_current_page();
                    self.update_undo_redo_state();
                    self.emit(DocumentEvent::StatusChanged {
                        message: success_message.into(),
                    });
                }
                Err(e) => self.emit(DocumentEvent::Error {
                    message: e.to_string(),
                }),
            }
        }
    }

    fn tool_insert_text(&mut self) {
        let cmd = Box::new(InsertTextCommand::new(
            self.current_page,
            "Tool text",
            72.0,
            720.0,
            14.0,
        ));
        self.run_tool_command(cmd, "Inserted sample text");
    }

    fn tool_modify_text(&mut self) {
        let cmd = Box::new(ModifyTextCommand::new(self.current_page, "Tool", "Edited"));
        self.run_tool_command(cmd, "Modified matching text on page");
    }

    fn tool_font_substitution(&mut self) {
        let cmd = Box::new(FontSubstitutionCommand::new(
            self.current_page,
            "Helvetica",
            "Times-Roman",
        ));
        self.run_tool_command(cmd, "Applied font substitution");
    }

    fn tool_insert_image(&mut self) {
        let w = 64u32;
        let h = 64u32;
        let mut data = vec![0u8; (w * h * 3) as usize];
        for y in 0..h {
            for x in 0..w {
                let i = ((y * w + x) * 3) as usize;
                let checker = ((x / 8) + (y / 8)) % 2 == 0;
                data[i] = if checker { 30 } else { 220 };
                data[i + 1] = if checker { 120 } else { 40 };
                data[i + 2] = if checker { 230 } else { 40 };
            }
        }
        let cmd = Box::new(InsertImageCommand::new(
            self.current_page,
            data,
            w,
            h,
            72.0,
            500.0,
            96.0,
            96.0,
        ));
        self.run_tool_command(cmd, "Inserted sample image");
    }

    fn tool_replace_image(&mut self) {
        let w = 32u32;
        let h = 32u32;
        let mut data = vec![0u8; (w * h * 3) as usize];
        for y in 0..h {
            for x in 0..w {
                let i = ((y * w + x) * 3) as usize;
                data[i] = 240;
                data[i + 1] = 180;
                data[i + 2] = 30;
            }
        }
        let cmd = Box::new(ReplaceImageCommand::new(
            self.current_page,
            "ImAuto1",
            data,
            w,
            h,
            Some(96.0),
            Some(96.0),
        ));
        self.run_tool_command(cmd, "Replaced image resource ImAuto1");
    }

    fn tool_set_password(&mut self) {
        let cmd = Box::new(SetPasswordCommand::new("password123"));
        self.run_tool_command(cmd, "Set document password");
    }

    fn tool_redact_region(&mut self) {
        let cmd = Box::new(RedactRegionCommand::new(
            self.current_page,
            72.0,
            680.0,
            240.0,
            40.0,
        ));
        self.run_tool_command(cmd, "Applied redaction region");
    }

    fn tool_apply_ocr(&mut self) {
        let result = OcrResult {
            page_index: self.current_page,
            regions: vec![TextRegion {
                text: "OCR sample".into(),
                x: 72.0,
                y: 640.0,
                width: 160.0,
                height: 20.0,
                confidence: 0.9,
            }],
            full_text: "OCR sample".into(),
        };
        let cmd = Box::new(ApplyOcrCommand::new(result));
        self.run_tool_command(cmd, "Applied OCR text layer");
    }

    fn tool_reorder_pages(&mut self) {
        if !self.ensure_document_open() {
            return;
        }
        let Some(doc) = &self.document else {
            return;
        };
        let count = doc.page_count();
        if count < 2 {
            self.emit(DocumentEvent::StatusChanged {
                message: "Need at least 2 pages to reorder".into(),
            });
            return;
        }
        let mut order: Vec<u32> = (0..count).collect();
        order.reverse();
        let cmd = Box::new(ReorderPagesCommand::new(order));
        self.run_tool_command(cmd, "Reordered pages (reversed)");
    }

    fn tool_merge_document(&mut self) {
        if !self.ensure_document_open() {
            return;
        }
        let path = FileDialog::new().add_filter("PDF", &["pdf"]).pick_file();
        let Some(path) = path else {
            self.emit(DocumentEvent::StatusChanged {
                message: "Merge canceled".into(),
            });
            return;
        };
        let other = match Document::open(&path) {
            Ok(doc) => doc,
            Err(e) => {
                self.emit(DocumentEvent::Error {
                    message: e.to_string(),
                });
                return;
            }
        };
        let cmd = Box::new(MergeDocumentCommand::new(other));
        self.run_tool_command(cmd, "Merged another document");
    }

    fn tool_create_field(&mut self) {
        let cmd = Box::new(CreateFieldCommand::new(
            format!("Field{}", self.current_page + 1),
            FormFieldKind::TextField,
            self.current_page,
            [72.0, 620.0, 260.0, 645.0],
        ));
        self.run_tool_command(cmd, "Created text form field");
    }

    fn tool_set_field_value(&mut self) {
        if !self.ensure_document_open() {
            return;
        }
        if let Some(doc) = &mut self.document {
            let fields = detect_form_fields(doc);
            let Some(field) = fields.first() else {
                self.emit(DocumentEvent::StatusChanged {
                    message: "No form fields found".into(),
                });
                return;
            };
            let new_value = match field.kind {
                FormFieldKind::Checkbox | FormFieldKind::Radio => FormFieldValue::Boolean(true),
                FormFieldKind::Dropdown => FormFieldValue::Selected("Option".into()),
                _ => FormFieldValue::Text("Sample value".into()),
            };
            let field_name = field.full_name.clone();
            let cmd = Box::new(SetFieldValueCommand::new(field_name, new_value));
            match self.history.execute(cmd, doc) {
                Ok(()) => {
                    self.render_current_page();
                    self.update_undo_redo_state();
                    self.emit(DocumentEvent::StatusChanged {
                        message: "Set value for first form field".into(),
                    });
                }
                Err(e) => self.emit(DocumentEvent::Error {
                    message: e.to_string(),
                }),
            }
        }
    }

    fn tool_detect_fields(&mut self) {
        if !self.ensure_document_open() {
            return;
        }
        if let Some(doc) = &self.document {
            let fields = detect_form_fields(doc);
            self.emit(DocumentEvent::StatusChanged {
                message: format!("Detected {} form field(s)", fields.len()),
            });
        }
    }

    fn tool_export_form_data(&mut self) {
        if !self.ensure_document_open() {
            return;
        }
        if let Some(doc) = &self.document {
            let json = export_form_data(doc).to_string();
            let preview: String = json.chars().take(140).collect();
            self.emit(DocumentEvent::StatusChanged {
                message: format!("Form JSON: {}", preview),
            });
        }
    }

    fn render_current_page(&mut self) {
        let (doc_id, page_count) = match &self.document {
            Some(d) => (d.id, d.page_count()),
            None => return,
        };
        if page_count == 0 {
            return;
        }
        let page = self.current_page.min(page_count - 1);
        let key = CacheKey::new(doc_id, page, self.zoom);

        // Increment the generation so any in-flight renders for the old target
        // can detect they are now stale and discard their results.
        let gen = self.render_generation.fetch_add(1, Ordering::Relaxed) + 1;

        // Fast path: serve from cache without touching the render thread.
        let cached_page = {
            let mut cache = self.cache.lock().expect("PageCache mutex was poisoned");
            cache.get(&key).cloned()
        };
        if let Some(rendered) = cached_page {
            apply_rendered_page(&rendered, &self.window, page, page_count);
            return;
        }

        // Slow path: send to the background render worker.
        if let Some(doc) = &self.document {
            let page_obj = match doc.get_page(page) {
                Ok(p) => p,
                Err(e) => {
                    self.emit(DocumentEvent::Error {
                        message: e.to_string(),
                    });
                    return;
                }
            };
            let task = RenderTask {
                doc_id,
                doc_path: doc.path.clone(),
                page_index: page,
                page_width: page_obj.media_box.width,
                page_height: page_obj.media_box.height,
                zoom: self.zoom,
                page_count,
                cache: Arc::clone(&self.cache),
                window: self.window.clone(),
                render_generation: Arc::clone(&self.render_generation),
                expected_generation: gen,
            };
            // Ensure the render task is eventually enqueued, even if the
            // bounded channel is currently full. We perform the potentially
            // blocking `send` on a short-lived background thread so the UI
            // thread remains responsive.
            let tx = self.render_tx.clone();
            thread::spawn(move || {
                let _ = tx.send(task);
            });
        }
    }

    /// Activates a commercial license file.
    ///
    /// In debug builds the path is read from the `ACTIVATE_LICENSE` environment
    /// variable so that activation can be exercised non-interactively (e.g. in
    /// automated tests).  In release builds this path is not available because
    /// a native file-picker has not yet been integrated; the button is therefore
    /// disabled in release to avoid shipping a non-functional code path.
    fn activate_license_dialog(&mut self) {
        #[cfg(debug_assertions)]
        {
            let path = match std::env::var("ACTIVATE_LICENSE") {
                Ok(p) => std::path::PathBuf::from(p),
                Err(_) => {
                    self.emit(DocumentEvent::StatusChanged {
                        message: "Set ACTIVATE_LICENSE env var to the path of your .pdfeditor-license file".into(),
                    });
                    return;
                }
            };

            match self.license.activate(&path) {
                Ok(()) => {
                    self.update_license_display();
                    self.emit(DocumentEvent::StatusChanged {
                        message: "Commercial license activated successfully.".into(),
                    });
                }
                Err(e) => {
                    tracing::warn!("license activation failed: {e}");
                    self.emit(DocumentEvent::StatusChanged {
                        message: format!("License activation failed: {e}"),
                    });
                }
            }
        }
        #[cfg(not(debug_assertions))]
        {
            self.emit(DocumentEvent::StatusChanged {
                message: "License activation is not available in this build.".into(),
            });
        }
    }

    fn update_undo_redo_state(&self) {
        if let Some(win) = self.window.upgrade() {
            win.set_can_undo(self.history.can_undo());
            win.set_can_redo(self.history.can_redo());
        }
    }

    /// Push the current license type and expiry to the UI.
    ///
    /// The UI only *reads* this state; it never validates the license itself.
    fn update_license_display(&self) {
        if let Some(win) = self.window.upgrade() {
            let state = self.license.current_license();
            let license_type_str = match state.license_type {
                licensing::LicenseType::Personal => "Personal",
                licensing::LicenseType::Commercial => "Commercial",
                licensing::LicenseType::Trial => "Trial",
                licensing::LicenseType::Enterprise => "Enterprise",
            };
            win.set_license_type(license_type_str.into());
            let expiry = if matches!(state.license_type, licensing::LicenseType::Personal) {
                // Personal has no meaningful expiry.
                String::new()
            } else {
                state.expiry.format("%Y-%m-%d").to_string()
            };
            win.set_license_expiry(expiry.into());
        }
    }

    fn emit(&self, event: DocumentEvent) {
        let _ = self.evt_tx.send(event);
    }
}

// ---------------------------------------------------------------------------
// Background render worker
// ---------------------------------------------------------------------------

/// Spawn a dedicated render worker thread and return a channel sender.
///
/// The worker receives [`RenderTask`]s, renders each page with
/// [`MuPdfRenderer::render_from_path`] (which opens the PDF via MuPDF without
/// requiring the lopdf `Document` object), then queues the result back onto
/// the Slint event loop via [`slint::invoke_from_event_loop`].  This ensures
/// the Slint UI thread is never blocked by rendering.
///
/// Falls back to [`SoftwareRenderer::render_from_dims`] when the document
/// path does not yet exist on disk (e.g. an unsaved new document).
fn spawn_render_worker() -> mpsc::SyncSender<RenderTask> {
    // Capacity 4: old requests are dropped when full so the worker always
    // processes the most recent navigation rather than queuing stale frames.
    let (tx, rx) = mpsc::sync_channel::<RenderTask>(4);

    thread::Builder::new()
        .name("render-worker".into())
        .spawn(move || {
            for task in rx {
                #[cfg(feature = "mupdf")]
                let result =
                    MuPdfRenderer::render_from_path(&task.doc_path, task.page_index, task.zoom)
                        .or_else(|mupdf_err| {
                            tracing::debug!(
                        "MuPDF render failed ({mupdf_err}), falling back to software renderer"
                    );
                            SoftwareRenderer::render_from_dims(
                                task.page_index,
                                task.page_width,
                                task.page_height,
                                task.zoom,
                            )
                        });
                #[cfg(not(feature = "mupdf"))]
                let result = SoftwareRenderer::render_from_dims(
                    task.page_index,
                    task.page_width,
                    task.page_height,
                    task.zoom,
                );

                let rendered = match result {
                    Ok(r) => r,
                    Err(e) => {
                        tracing::warn!("render error: {e}");
                        continue;
                    }
                };

                // Store in the shared cache before invoking the UI callback.
                let key = CacheKey::new(task.doc_id, task.page_index, task.zoom);
                task.cache
                    .lock()
                    .expect("PageCache mutex was poisoned")
                    .insert(key, rendered.clone());

                // Hand the pixel buffer to the Slint event loop.
                let win = task.window.clone();
                let page_index = task.page_index;
                let page_count = task.page_count;
                let render_generation = Arc::clone(&task.render_generation);
                let expected_generation = task.expected_generation;
                slint::invoke_from_event_loop(move || {
                    // If the UI has since requested a different page/zoom/doc,
                    // the generation counter will have advanced beyond ours.
                    // Discard stale frames to prevent them from overwriting the
                    // current view.
                    if render_generation.load(Ordering::Relaxed) == expected_generation {
                        apply_rendered_page(&rendered, &win, page_index, page_count);
                    }
                })
                .ok();
            }
        })
        .expect("spawn render worker");

    tx
}

/// Apply a finished [`RenderedPage`] to the Slint window.
/// Must be called on the Slint UI thread.
fn apply_rendered_page(
    rendered: &RenderedPage,
    window: &Weak<AppWindow>,
    page_index: u32,
    page_count: u32,
) {
    if let Some(win) = window.upgrade() {
        let mut buf = SharedPixelBuffer::<Rgba8Pixel>::new(rendered.width, rendered.height);
        buf.make_mut_bytes().copy_from_slice(&rendered.data);
        let image = Image::from_rgba8(buf);
        win.set_page_image(image.clone());
        win.set_thumbnail_image(image);
        win.set_current_page(page_index as i32 + 1);
        win.set_page_count(page_count as i32);
    }
}
