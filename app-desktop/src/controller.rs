use licensing::LicenseManager;
use lopdf::content::Content;
use lopdf::Object;
use pdf_annotations::{
    io::{read_annotations, update_note_content_by_object_id},
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
use slint::{Image, Rgba8Pixel, SharedPixelBuffer, SharedString, Weak};
use std::collections::{BTreeSet, HashMap};
use std::fs;
use std::path::PathBuf;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{mpsc, mpsc::Sender, Arc, Mutex};
use std::thread;

use crate::AppWindow;
use crate::constants::{
    CACHE_CAPACITY, HISTORY_DEPTH, IMAGE_EDIT_PANEL_HEIGHT, IMAGE_EDIT_PANEL_WIDTH,
    MAX_IMAGE_RESOURCE_MENU_ITEMS, MAX_RECENT_DISPLAY_CHARS, MAX_RECENT_DOCUMENTS,
};
use crate::models::{
    DeleteImagePlacementCommand, DeleteTextAtOpCommand, ImageHit, InsertedTextAnchor,
    PendingAnnotationTool, RenderTask, TextHit, UpdateImageTransformCommand,
    UpdateNoteContentCommand, UpdateTextAtOpCommand,
};

impl UpdateNoteContentCommand {
    fn new(
        page_index: u32,
        object_id: (u32, u16),
        old_content: String,
        new_content: String,
    ) -> Self {
        Self {
            page_index,
            object_id,
            old_content,
            new_content,
        }
    }
}

impl DocumentCommand for UpdateNoteContentCommand {
    fn description(&self) -> &str {
        "Edit note annotation"
    }

    fn execute(&mut self, doc: &mut Document) -> Result<(), pdf_core::PdfCoreError> {
        update_note_content_by_object_id(doc, self.page_index, self.object_id, &self.new_content)
            .map(|_| ())
    }

    fn undo(&mut self, doc: &mut Document) -> Result<(), pdf_core::PdfCoreError> {
        update_note_content_by_object_id(doc, self.page_index, self.object_id, &self.old_content)
            .map(|_| ())
    }
}

impl DeleteImagePlacementCommand {
    fn new(page_index: u32, stream_id: lopdf::ObjectId, do_op_index: usize) -> Self {
        Self {
            page_index,
            stream_id,
            do_op_index,
            snapshot: None,
        }
    }
}

impl DocumentCommand for DeleteImagePlacementCommand {
    fn description(&self) -> &str {
        "Delete image"
    }

    fn execute(&mut self, doc: &mut Document) -> Result<(), pdf_core::PdfCoreError> {
        let _ = doc.get_page(self.page_index)?;

        let mut buf = std::io::Cursor::new(Vec::new());
        doc.inner_mut()
            .save_to(&mut buf)
            .map_err(|e| pdf_core::PdfCoreError::LopdfError(e.to_string()))?;
        self.snapshot = Some(buf.into_inner());

        doc.inner_mut().decompress();
        let stream = doc
            .inner_mut()
            .get_object_mut(self.stream_id)
            .map_err(|e| pdf_core::PdfCoreError::LopdfError(e.to_string()))?
            .as_stream_mut()
            .map_err(|e| pdf_core::PdfCoreError::LopdfError(e.to_string()))?;

        let mut content = Content::decode(&stream.content)
            .map_err(|e| pdf_core::PdfCoreError::LopdfError(e.to_string()))?;

        if self.do_op_index >= content.operations.len() {
            return Err(pdf_core::PdfCoreError::InvalidArgument(
                "target image no longer exists".to_owned(),
            ));
        }

        let op = &content.operations[self.do_op_index];
        if op.operator != "Do" {
            return Err(pdf_core::PdfCoreError::InvalidArgument(
                "target image placement changed".to_owned(),
            ));
        }

        content.operations.remove(self.do_op_index);
        let encoded = content
            .encode()
            .map_err(|e| pdf_core::PdfCoreError::LopdfError(e.to_string()))?;
        stream.dict.remove(b"Filter");
        stream.dict.remove(b"DecodeParms");
        stream.dict.set("Length", Object::Integer(encoded.len() as i64));
        stream.content = encoded;
        Ok(())
    }

    fn undo(&mut self, doc: &mut Document) -> Result<(), pdf_core::PdfCoreError> {
        let snap = self
            .snapshot
            .as_ref()
            .ok_or(pdf_core::PdfCoreError::NotUndoable)?;
        let restored = lopdf::Document::load_mem(snap)
            .map_err(|e| pdf_core::PdfCoreError::LopdfError(e.to_string()))?;
        *doc.inner_mut() = restored;
        Ok(())
    }
}

impl UpdateImageTransformCommand {
    fn new(
        page_index: u32,
        stream_id: lopdf::ObjectId,
        cm_op_index: usize,
        old_matrix: [f32; 6],
        new_matrix: [f32; 6],
    ) -> Self {
        Self {
            page_index,
            stream_id,
            cm_op_index,
            old_matrix,
            new_matrix,
            snapshot: None,
        }
    }
}

impl DocumentCommand for UpdateImageTransformCommand {
    fn description(&self) -> &str {
        "Edit image transform"
    }

    fn execute(&mut self, doc: &mut Document) -> Result<(), pdf_core::PdfCoreError> {
        let _ = doc.get_page(self.page_index)?;

        let mut buf = std::io::Cursor::new(Vec::new());
        doc.inner_mut()
            .save_to(&mut buf)
            .map_err(|e| pdf_core::PdfCoreError::LopdfError(e.to_string()))?;
        self.snapshot = Some(buf.into_inner());

        doc.inner_mut().decompress();

        let stream = doc
            .inner_mut()
            .get_object_mut(self.stream_id)
            .map_err(|e| pdf_core::PdfCoreError::LopdfError(e.to_string()))?
            .as_stream_mut()
            .map_err(|e| pdf_core::PdfCoreError::LopdfError(e.to_string()))?;

        let mut content = Content::decode(&stream.content)
            .map_err(|e| pdf_core::PdfCoreError::LopdfError(e.to_string()))?;

        if self.cm_op_index >= content.operations.len() {
            return Err(pdf_core::PdfCoreError::InvalidArgument(
                "target image transform no longer exists".to_owned(),
            ));
        }

        let op = &mut content.operations[self.cm_op_index];
        if op.operator != "cm" || op.operands.len() < 6 {
            return Err(pdf_core::PdfCoreError::InvalidArgument(
                "target image transform changed".to_owned(),
            ));
        }

        let current = [
            AppController::operand_to_f32(&op.operands[0]).unwrap_or(0.0),
            AppController::operand_to_f32(&op.operands[1]).unwrap_or(0.0),
            AppController::operand_to_f32(&op.operands[2]).unwrap_or(0.0),
            AppController::operand_to_f32(&op.operands[3]).unwrap_or(0.0),
            AppController::operand_to_f32(&op.operands[4]).unwrap_or(0.0),
            AppController::operand_to_f32(&op.operands[5]).unwrap_or(0.0),
        ];

        if !AppController::matrix_approx_eq(current, self.old_matrix, 0.5) {
            return Err(pdf_core::PdfCoreError::InvalidArgument(
                "image transform changed since selection".to_owned(),
            ));
        }

        op.operands = vec![
            Object::Real(self.new_matrix[0]),
            Object::Real(self.new_matrix[1]),
            Object::Real(self.new_matrix[2]),
            Object::Real(self.new_matrix[3]),
            Object::Real(self.new_matrix[4]),
            Object::Real(self.new_matrix[5]),
        ];

        let encoded = content
            .encode()
            .map_err(|e| pdf_core::PdfCoreError::LopdfError(e.to_string()))?;
        stream.dict.remove(b"Filter");
        stream.dict.remove(b"DecodeParms");
        stream.dict.set("Length", Object::Integer(encoded.len() as i64));
        stream.content = encoded;

        Ok(())
    }

    fn undo(&mut self, doc: &mut Document) -> Result<(), pdf_core::PdfCoreError> {
        let snap = self
            .snapshot
            .as_ref()
            .ok_or(pdf_core::PdfCoreError::NotUndoable)?;
        let restored = lopdf::Document::load_mem(snap)
            .map_err(|e| pdf_core::PdfCoreError::LopdfError(e.to_string()))?;
        *doc.inner_mut() = restored;
        Ok(())
    }
}

impl UpdateTextAtOpCommand {
    fn new(
        page_index: u32,
        stream_id: lopdf::ObjectId,
        op_index: usize,
        old_text: String,
        new_text: String,
    ) -> Self {
        Self {
            page_index,
            stream_id,
            op_index,
            old_text,
            new_text,
            snapshot: None,
        }
    }
}

impl DocumentCommand for UpdateTextAtOpCommand {
    fn description(&self) -> &str {
        "Edit selected text"
    }

    fn execute(&mut self, doc: &mut Document) -> Result<(), pdf_core::PdfCoreError> {
        let _ = doc.get_page(self.page_index)?;

        let mut buf = std::io::Cursor::new(Vec::new());
        doc.inner_mut()
            .save_to(&mut buf)
            .map_err(|e| pdf_core::PdfCoreError::LopdfError(e.to_string()))?;
        self.snapshot = Some(buf.into_inner());

        doc.inner_mut().decompress();
        let stream = doc
            .inner_mut()
            .get_object_mut(self.stream_id)
            .map_err(|e| pdf_core::PdfCoreError::LopdfError(e.to_string()))?
            .as_stream_mut()
            .map_err(|e| pdf_core::PdfCoreError::LopdfError(e.to_string()))?;

        let mut content = Content::decode(&stream.content)
            .map_err(|e| pdf_core::PdfCoreError::LopdfError(e.to_string()))?;
        if self.op_index >= content.operations.len() {
            return Err(pdf_core::PdfCoreError::InvalidArgument(
                "selected text operation no longer exists".to_owned(),
            ));
        }

        let op = &mut content.operations[self.op_index];
        if op.operator != "Tj" {
            return Err(pdf_core::PdfCoreError::InvalidArgument(
                "selected text is not directly editable".to_owned(),
            ));
        }

        let current_text = match op.operands.first() {
            Some(Object::String(bytes, _)) => AppController::decode_pdf_text_bytes(bytes),
            _ => {
                return Err(pdf_core::PdfCoreError::InvalidArgument(
                    "selected text payload changed".to_owned(),
                ))
            }
        };
        if current_text != self.old_text {
            return Err(pdf_core::PdfCoreError::InvalidArgument(
                "selected text no longer matches".to_owned(),
            ));
        }

        op.operands[0] = lopdf::text_string(&self.new_text);
        let encoded = content
            .encode()
            .map_err(|e| pdf_core::PdfCoreError::LopdfError(e.to_string()))?;
        stream.dict.remove(b"Filter");
        stream.dict.remove(b"DecodeParms");
        stream.dict.set("Length", Object::Integer(encoded.len() as i64));
        stream.content = encoded;
        Ok(())
    }

    fn undo(&mut self, doc: &mut Document) -> Result<(), pdf_core::PdfCoreError> {
        let snap = self
            .snapshot
            .as_ref()
            .ok_or(pdf_core::PdfCoreError::NotUndoable)?;
        let restored = lopdf::Document::load_mem(snap)
            .map_err(|e| pdf_core::PdfCoreError::LopdfError(e.to_string()))?;
        *doc.inner_mut() = restored;
        Ok(())
    }
}

impl DeleteTextAtOpCommand {
    fn new(page_index: u32, stream_id: lopdf::ObjectId, op_index: usize, old_text: String) -> Self {
        Self {
            page_index,
            stream_id,
            op_index,
            old_text,
            snapshot: None,
        }
    }
}

impl DocumentCommand for DeleteTextAtOpCommand {
    fn description(&self) -> &str {
        "Delete selected text"
    }

    fn execute(&mut self, doc: &mut Document) -> Result<(), pdf_core::PdfCoreError> {
        let _ = doc.get_page(self.page_index)?;

        let mut buf = std::io::Cursor::new(Vec::new());
        doc.inner_mut()
            .save_to(&mut buf)
            .map_err(|e| pdf_core::PdfCoreError::LopdfError(e.to_string()))?;
        self.snapshot = Some(buf.into_inner());

        doc.inner_mut().decompress();
        let stream = doc
            .inner_mut()
            .get_object_mut(self.stream_id)
            .map_err(|e| pdf_core::PdfCoreError::LopdfError(e.to_string()))?
            .as_stream_mut()
            .map_err(|e| pdf_core::PdfCoreError::LopdfError(e.to_string()))?;

        let mut content = Content::decode(&stream.content)
            .map_err(|e| pdf_core::PdfCoreError::LopdfError(e.to_string()))?;
        if self.op_index >= content.operations.len() {
            return Err(pdf_core::PdfCoreError::InvalidArgument(
                "selected text operation no longer exists".to_owned(),
            ));
        }

        let op = &content.operations[self.op_index];
        if op.operator != "Tj" {
            return Err(pdf_core::PdfCoreError::InvalidArgument(
                "selected text is not directly deletable".to_owned(),
            ));
        }
        let current_text = match op.operands.first() {
            Some(Object::String(bytes, _)) => AppController::decode_pdf_text_bytes(bytes),
            _ => {
                return Err(pdf_core::PdfCoreError::InvalidArgument(
                    "selected text payload changed".to_owned(),
                ))
            }
        };
        if current_text != self.old_text {
            return Err(pdf_core::PdfCoreError::InvalidArgument(
                "selected text no longer matches".to_owned(),
            ));
        }

        content.operations.remove(self.op_index);
        let encoded = content
            .encode()
            .map_err(|e| pdf_core::PdfCoreError::LopdfError(e.to_string()))?;
        stream.dict.remove(b"Filter");
        stream.dict.remove(b"DecodeParms");
        stream.dict.set("Length", Object::Integer(encoded.len() as i64));
        stream.content = encoded;
        Ok(())
    }

    fn undo(&mut self, doc: &mut Document) -> Result<(), pdf_core::PdfCoreError> {
        let snap = self
            .snapshot
            .as_ref()
            .ok_or(pdf_core::PdfCoreError::NotUndoable)?;
        let restored = lopdf::Document::load_mem(snap)
            .map_err(|e| pdf_core::PdfCoreError::LopdfError(e.to_string()))?;
        *doc.inner_mut() = restored;
        Ok(())
    }
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
    pending_annotation_tool: Option<PendingAnnotationTool>,
    recent_documents: Vec<PathBuf>,
    image_resource_menu_offset: usize,
    selected_image_resource_name: Option<String>,
    selected_image_target: Option<(lopdf::ObjectId, usize)>,
    selected_image_hit: Option<ImageHit>,
    image_overlay_visible: bool,
    image_overlay_custom_canvas_pos: Option<(f32, f32)>,
    image_drag_active: bool,
    image_drag_start_canvas: Option<(f32, f32)>,
    image_drag_current_canvas: Option<(f32, f32)>,
    image_drag_start_hit: Option<ImageHit>,
    suppress_next_canvas_click: bool,
    text_insert_panel_visible: bool,
    selected_text_hit: Option<TextHit>,
    insert_text_next_y_by_page: HashMap<u32, f32>,
    inserted_text_anchors_by_page: HashMap<u32, Vec<InsertedTextAnchor>>,
}

impl AppController {
    fn middle_ellipsize_text(input: &str, max_chars: usize) -> String {
        let total = input.chars().count();
        if total <= max_chars {
            return input.to_string();
        }
        if max_chars <= 3 {
            return "...".to_string();
        }

        let keep = max_chars - 3;
        let left_keep = keep / 2;
        let right_keep = keep - left_keep;

        let left = input.chars().take(left_keep).collect::<String>();
        let right = input
            .chars()
            .rev()
            .take(right_keep)
            .collect::<Vec<char>>()
            .into_iter()
            .rev()
            .collect::<String>();
        format!("{left}...{right}")
    }

    fn middle_ellipsize_path(path: &std::path::Path, max_chars: usize) -> String {
        let full = path.display().to_string();
        if full.chars().count() <= max_chars {
            return full;
        }

        if let Some(name) = path.file_name().and_then(|s| s.to_str()) {
            let name_chars = name.chars().count();
            if name_chars + 4 < max_chars {
                let prefix_budget = max_chars.saturating_sub(name_chars + 3);
                let prefix = full.chars().take(prefix_budget).collect::<String>();
                return format!("{prefix}...{name}");
            }
        }

        Self::middle_ellipsize_text(&full, max_chars)
    }

    fn prompt_input(title: &str, prompt: &str, default_text: &str) -> Option<String> {
        tinyfiledialogs::input_box(title, prompt, default_text)
    }

    fn prompt_note_text(default_text: &str) -> Option<String> {
        tinyfiledialogs::input_box("Insert Note", "Note text:", default_text)
    }

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
        let mut controller = Self {
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
            pending_annotation_tool: None,
            recent_documents: Vec::new(),
            image_resource_menu_offset: 0,
            selected_image_resource_name: None,
            selected_image_target: None,
            selected_image_hit: None,
            image_overlay_visible: false,
            image_overlay_custom_canvas_pos: None,
            image_drag_active: false,
            image_drag_start_canvas: None,
            image_drag_current_canvas: None,
            image_drag_start_hit: None,
            suppress_next_canvas_click: false,
            text_insert_panel_visible: false,
            selected_text_hit: None,
            insert_text_next_y_by_page: HashMap::new(),
            inserted_text_anchors_by_page: HashMap::new(),
        };
        controller.load_recent_documents();
        controller
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

        win.on_open_recent(move |index| {
            let me = unsafe { &mut *ptr };
            me.open_recent_document(index);
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

        win.on_zoom_set(move |factor| {
            let me = unsafe { &mut *ptr };
            me.set_zoom(factor);
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

        win.on_canvas_clicked(move |x, y| {
            let me = unsafe { &mut *ptr };
            if let Some(win) = me.window.upgrade() {
                win.set_status_text(format!("Canvas click: x={:.1}, y={:.1}", x, y).into());
            }
            if me.suppress_next_canvas_click {
                me.suppress_next_canvas_click = false;
                return;
            }
            if me.pending_annotation_tool.is_some() {
                me.place_pending_annotation(x as f32, y as f32);
            } else {
                me.handle_canvas_edit_click(x as f32, y as f32, true);
            }
        });

        win.on_canvas_double_clicked(move |x, y| {
            let me = unsafe { &mut *ptr };
            if let Some(win) = me.window.upgrade() {
                win.set_status_text(format!("Canvas double-click: x={:.1}, y={:.1}", x, y).into());
            }
            // Keep double-click behavior aligned with single-click so we don't
            // open the legacy modal text input when users rapidly click text.
            me.handle_canvas_edit_click(x as f32, y as f32, true);
        });

        win.on_canvas_press_start(move |x, y| {
            let me = unsafe { &mut *ptr };
            me.start_canvas_image_drag(x as f32, y as f32);
        });

        win.on_canvas_press_move(move |x, y| {
            let me = unsafe { &mut *ptr };
            me.move_canvas_image_drag(x as f32, y as f32);
        });

        win.on_canvas_press_end(move |x, y| {
            let me = unsafe { &mut *ptr };
            let drag_start = me.image_drag_start_canvas;
            let moved = drag_start
                .map(|(sx, sy)| ((x as f32 - sx).powi(2) + (y as f32 - sy).powi(2)).sqrt() >= 4.0)
                .unwrap_or(false);
            me.end_canvas_image_drag(x as f32, y as f32);

            // Treat release without meaningful movement as a click. This path
            // is robust even when TouchArea `clicked` is swallowed by ScrollView.
            if !moved {
                if me.suppress_next_canvas_click {
                    me.suppress_next_canvas_click = false;
                    return;
                }
                if me.pending_annotation_tool.is_some() {
                    me.place_pending_annotation(x as f32, y as f32);
                } else {
                    me.handle_canvas_edit_click(x as f32, y as f32, true);
                }
            }
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

        win.on_tools_edit_image_resource(move |index| {
            let me = unsafe { &mut *ptr };
            me.tool_edit_image_resource(index);
        });

        win.on_image_edit_rotate90(move || {
            let me = unsafe { &mut *ptr };
            me.apply_selected_image_action("r90");
        });

        win.on_image_edit_flip(move || {
            let me = unsafe { &mut *ptr };
            me.apply_selected_image_action("flip");
        });

        win.on_image_edit_delete(move || {
            let me = unsafe { &mut *ptr };
            me.tool_delete_selected_image_placement();
        });

        win.on_image_edit_apply_size(move |w_text, h_text, keep_aspect| {
            let me = unsafe { &mut *ptr };
            me.apply_image_size_from_panel(w_text.to_string(), h_text.to_string(), keep_aspect);
        });

        win.on_text_edit_insert(move |content_text, font_text, size_text| {
            let me = unsafe { &mut *ptr };
            me.apply_text_insert_from_panel(
                content_text.to_string(),
                font_text.to_string(),
                size_text.to_string(),
            );
        });

        win.on_text_edit_delete(move || {
            let me = unsafe { &mut *ptr };
            me.apply_text_delete_from_panel();
        });

        win.on_text_edit_close(move || {
            let me = unsafe { &mut *ptr };
            me.text_insert_panel_visible = false;
            me.clear_selected_text_selection();
            me.update_text_insert_panel_display();
        });

        win.on_text_edit_font_changed(move |font_text| {
            let me = unsafe { &mut *ptr };
            me.sync_text_font_name_from_input(font_text.to_string());
        });

        win.on_tools_clear_image_selection(move || {
            let me = unsafe { &mut *ptr };
            me.clear_selected_image_overlay();
        });

        win.on_tools_image_resource_prev_page(move || {
            let me = unsafe { &mut *ptr };
            me.shift_image_resource_menu_page(-1);
        });

        win.on_tools_image_resource_next_page(move || {
            let me = unsafe { &mut *ptr };
            me.shift_image_resource_menu_page(1);
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

        win.on_menu_action_selected(move |menu_index, action_index| {
            let me = unsafe { &mut *ptr };
            me.dispatch_menu_action(menu_index, action_index);
        });

        // Display the initial license state in the UI.
        self.update_license_display();
        self.update_recent_documents_display();
        self.update_image_resource_menu_display();
        self.update_text_insert_panel_display();
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

        self.open_document_path(path);
    }

    fn open_recent_document(&mut self, index: i32) {
        if index < 0 {
            return;
        }
        let Some(path) = self.recent_documents.get(index as usize).cloned() else {
            self.emit(DocumentEvent::StatusChanged {
                message: "Recent document entry not available".into(),
            });
            return;
        };
        if !path.exists() {
            self.recent_documents.retain(|p| p != &path);
            self.save_recent_documents();
            self.update_recent_documents_display();
            self.emit(DocumentEvent::StatusChanged {
                message: format!("Recent file no longer exists: {}", path.display()),
            });
            return;
        }
        self.open_document_path(path);
    }

    fn open_document_path(&mut self, path: PathBuf) {
        match Document::open(&path) {
            Ok(doc) => {
                let title = doc.title.clone();
                let page_count = doc.page_count();
                self.document = Some(doc);
                self.current_page = 0;
                self.insert_text_next_y_by_page.clear();
                self.inserted_text_anchors_by_page.clear();
                self.selected_image_resource_name = None;
                self.selected_image_target = None;
                self.selected_image_hit = None;
                self.image_overlay_visible = false;
                self.image_overlay_custom_canvas_pos = None;
                self.image_drag_active = false;
                self.image_drag_start_canvas = None;
                self.image_drag_current_canvas = None;
                self.image_drag_start_hit = None;
                self.suppress_next_canvas_click = false;
                self.text_insert_panel_visible = false;
                self.clear_selected_text_selection();
                if let Some(win) = self.window.upgrade() {
                    win.set_image_drag_active(false);
                }
                self.update_text_insert_panel_display();
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
                self.add_recent_document(path);
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
        let mut recent_to_add: Option<PathBuf> = None;
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
                    Ok(()) => {
                        recent_to_add = Some(path.clone());
                        self.emit(DocumentEvent::DocumentSaved {
                            path: path.display().to_string(),
                        })
                    }
                    Err(e) => self.emit(DocumentEvent::Error {
                        message: e.to_string(),
                    }),
                }
            } else {
                match doc.save() {
                    Ok(()) => {
                        let saved_path = doc.path.clone();
                        let path = saved_path.display().to_string();
                        if !saved_path.as_os_str().is_empty() {
                            recent_to_add = Some(saved_path);
                        }
                        self.emit(DocumentEvent::DocumentSaved { path });
                    }
                    Err(e) => self.emit(DocumentEvent::Error {
                        message: e.to_string(),
                    }),
                }
            }
        }
        if let Some(path) = recent_to_add {
            self.add_recent_document(path);
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
        let mut recent_to_add: Option<PathBuf> = None;
        if let Some(doc) = &mut self.document {
            match doc.save_to(&path) {
                Ok(()) => {
                    recent_to_add = Some(path.clone());
                    self.emit(DocumentEvent::DocumentSaved {
                        path: path.display().to_string(),
                    })
                }
                Err(e) => self.emit(DocumentEvent::Error {
                    message: e.to_string(),
                }),
            }
        }
        if let Some(path) = recent_to_add {
            self.add_recent_document(path);
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
        self.insert_text_next_y_by_page.clear();
        self.inserted_text_anchors_by_page.clear();
        self.selected_image_resource_name = None;
        self.selected_image_target = None;
        self.selected_image_hit = None;
        self.image_overlay_visible = false;
        self.image_overlay_custom_canvas_pos = None;
        self.image_drag_active = false;
        self.image_drag_start_canvas = None;
        self.image_drag_current_canvas = None;
        self.image_drag_start_hit = None;
        self.suppress_next_canvas_click = false;
        self.text_insert_panel_visible = false;
        self.clear_selected_text_selection();
        if let Some(win) = self.window.upgrade() {
            win.set_image_drag_active(false);
        }
        self.history.clear();
        self.update_image_resource_menu_display();
        self.update_image_overlay_display(None);
        self.update_text_insert_panel_display();
        self.update_image_selection_display(None);
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
                self.cache
                    .lock()
                    .expect("PageCache mutex was poisoned")
                    .evict_document(doc.id);
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
                self.cache
                    .lock()
                    .expect("PageCache mutex was poisoned")
                    .evict_document(doc.id);
                self.render_current_page();
                self.update_undo_redo_state();
            }
        }
    }

    fn add_highlight_annotation(&mut self) {
        if self.document.is_none() {
            return;
        }
        if matches!(self.pending_annotation_tool, Some(PendingAnnotationTool::Highlight)) {
            self.clear_pending_annotation_mode();
            self.emit(DocumentEvent::StatusChanged {
                message: "Highlight placement mode disabled".into(),
            });
            return;
        }
        self.pending_annotation_tool = Some(PendingAnnotationTool::Highlight);
        self.emit(DocumentEvent::StatusChanged {
            message: "Click on the page to place a highlight".into(),
        });
    }

    fn add_note_annotation(&mut self) {
        if self.document.is_none() {
            return;
        }
        if matches!(self.pending_annotation_tool, Some(PendingAnnotationTool::Note)) {
            self.clear_pending_annotation_mode();
            self.emit(DocumentEvent::StatusChanged {
                message: "Note placement mode disabled".into(),
            });
            return;
        }
        self.pending_annotation_tool = Some(PendingAnnotationTool::Note);
        self.emit(DocumentEvent::StatusChanged {
            message: "Click on the page to place a note".into(),
        });
    }

    fn clear_pending_annotation_mode(&mut self) {
        self.pending_annotation_tool = None;
    }

    fn canvas_to_pdf_point(&self, canvas_x: f32, canvas_y: f32) -> Option<(f32, f32)> {
        let doc = self.document.as_ref()?;
        let page = doc.get_page(self.current_page).ok()?;
        let w = page.media_box.width as f32;
        let h = page.media_box.height as f32;
        let rotation = self.page_rotation_degrees();

        let (display_w, display_h) = if rotation == 90 || rotation == 270 {
            (h, w)
        } else {
            (w, h)
        };

        let xd = (canvas_x / self.zoom).clamp(0.0, display_w);
        let yd_top = (canvas_y / self.zoom).clamp(0.0, display_h);
        let yd = (display_h - yd_top).clamp(0.0, display_h);

        // /Rotate is clockwise. Map displayed page coordinates back into the
        // unrotated page user space used by content streams.
        let (px, py) = match rotation {
            90 => ((w - yd).clamp(0.0, w), xd.clamp(0.0, h)),
            180 => ((w - xd).clamp(0.0, w), (h - yd).clamp(0.0, h)),
            270 => (yd.clamp(0.0, w), (h - xd).clamp(0.0, h)),
            _ => (xd.clamp(0.0, w), yd.clamp(0.0, h)),
        };

        Some((px, py))
    }

    fn page_rotation_degrees(&self) -> i32 {
        let Some(doc) = &self.document else {
            return 0;
        };
        let Ok(page) = doc.get_page(self.current_page) else {
            return 0;
        };

        let inner = doc.inner();
        let mut current_id = page.object_id;
        loop {
            let Ok(node_obj) = inner.get_object(current_id) else {
                break;
            };
            let Ok(node_dict) = node_obj.as_dict() else {
                break;
            };

            if let Ok(rot) = node_dict.get(b"Rotate") {
                if let Ok(deg) = rot.as_i64() {
                    let normalized = (((deg % 360) + 360) % 360) as i32;
                    return match normalized {
                        90 | 180 | 270 => normalized,
                        _ => 0,
                    };
                }
            }

            let Some(parent_id) = node_dict
                .get(b"Parent")
                .ok()
                .and_then(|p| p.as_reference().ok())
            else {
                break;
            };
            current_id = parent_id;
        }

        0
    }

    fn execute_add_annotation(&mut self, annotation: Annotation, message: &str) {
        let Some(doc) = &mut self.document else {
            return;
        };
        let id = annotation.id.0.clone();
        let cmd = Box::new(AddAnnotationCommand::new(annotation));
        match self.history.execute(cmd, doc) {
            Ok(()) => {
                self.cache
                    .lock()
                    .expect("PageCache mutex was poisoned")
                    .evict_document(doc.id);
                self.render_current_page();
                self.emit(DocumentEvent::AnnotationAdded {
                    page: self.current_page,
                    annotation_id: id,
                });
                self.emit(DocumentEvent::StatusChanged {
                    message: message.into(),
                });
                self.update_undo_redo_state();
            }
            Err(e) => self.emit(DocumentEvent::Error {
                message: e.to_string(),
            }),
        }
    }

    fn place_pending_annotation(&mut self, canvas_x: f32, canvas_y: f32) {
        let Some(tool) = self.pending_annotation_tool else {
            return;
        };
        let Some((px, py)) = self.canvas_to_pdf_point(canvas_x, canvas_y) else {
            return;
        };

        match tool {
            PendingAnnotationTool::Highlight => {
                let annotation = Annotation::new(
                    self.current_page,
                    Rect {
                        x: (px - 90.0).max(0.0),
                        y: (py - 8.0).max(0.0),
                        width: 180.0,
                        height: 16.0,
                    },
                    AnnotationKind::Highlight {
                        color: Color::yellow(),
                    },
                );
                self.execute_add_annotation(
                    annotation,
                    "Highlight annotation added. Click to place another, or select Highlight again to stop.",
                );
            }
            PendingAnnotationTool::Note => {
                if let Some((object_id, existing_content)) = self.find_note_at_point(px, py) {
                    let Some(raw_note_text) = Self::prompt_note_text(&existing_content) else {
                        self.emit(DocumentEvent::StatusChanged {
                            message: "Note edit canceled".into(),
                        });
                        return;
                    };
                    let note_text = raw_note_text.trim().to_owned();
                    if note_text == existing_content {
                        self.emit(DocumentEvent::StatusChanged {
                            message: "Note unchanged".into(),
                        });
                        return;
                    }
                    let cmd = Box::new(UpdateNoteContentCommand::new(
                        self.current_page,
                        object_id,
                        existing_content,
                        note_text,
                    ));
                    if let Some(doc) = &mut self.document {
                        match self.history.execute(cmd, doc) {
                            Ok(()) => {
                                self.cache
                                    .lock()
                                    .expect("PageCache mutex was poisoned")
                                    .evict_document(doc.id);
                                self.render_current_page();
                                self.update_undo_redo_state();
                                self.emit(DocumentEvent::StatusChanged {
                                    message: "Note updated. Click to edit/add another note, or select Note again to stop.".into(),
                                });
                            }
                            Err(e) => self.emit(DocumentEvent::Error {
                                message: e.to_string(),
                            }),
                        }
                    }
                    return;
                }

                let existing_count = self
                    .document
                    .as_ref()
                    .map(|d| read_annotations(d, self.current_page).len() as u32)
                    .unwrap_or(0);
                let default_text = format!("Note #{}", existing_count + 1);
                let Some(raw_note_text) = Self::prompt_note_text(&default_text) else {
                    self.emit(DocumentEvent::StatusChanged {
                        message: "Note insertion canceled".into(),
                    });
                    return;
                };
                let note_text = if raw_note_text.trim().is_empty() {
                    default_text
                } else {
                    raw_note_text.trim().to_owned()
                };
                let annotation = Annotation::new(
                    self.current_page,
                    Rect {
                        x: (px - 10.0).max(0.0),
                        y: (py - 10.0).max(0.0),
                        width: 22.0,
                        height: 22.0,
                    },
                    AnnotationKind::Note {
                        author: "User".into(),
                        content: note_text,
                    },
                );
                self.execute_add_annotation(
                    annotation,
                    "Note annotation added. Click to place another, or select Note again to stop.",
                );
            }
        }
    }

    fn handle_canvas_edit_click(&mut self, canvas_x: f32, canvas_y: f32, notify_miss: bool) {
        if !self.ensure_document_open() {
            return;
        }
        let Some((px, py)) = self.canvas_to_pdf_point(canvas_x, canvas_y) else {
            return;
        };

        let image_hit = self
            .find_image_hit_at_point(px, py)
            .or_else(|| self.find_nearest_image_hit_at_point(px, py));
        if let Some(hit) = image_hit {
            self.text_insert_panel_visible = false;
            self.clear_selected_text_selection();
            self.update_text_insert_panel_display();
            self.selected_image_resource_name = Some(hit.resource_name.clone());
            self.selected_image_target = Some((hit.stream_id, hit.do_op_index));
            self.selected_image_hit = Some(hit.clone());
            self.image_overlay_visible = true;
            self.image_overlay_custom_canvas_pos = None;
            self.update_image_overlay_display(Some(&hit));
            self.update_image_selection_display(Some(&hit));
            return;
        }

        if let Some(anchor) = self.find_inserted_text_anchor_at_point(px, py) {
            let acx = anchor.x + anchor.width * 0.5;
            let acy = anchor.y + anchor.height * 0.5;
            let mapped_hit = anchor
                .target_stream_id
                .zip(anchor.target_op_index)
                .and_then(|(sid, op)| self.find_text_hit_by_target(sid, op));
            let fallback_hit = self
                .find_nearest_editable_text_hit(acx, acy, Some(&anchor.text))
                .or_else(|| self.find_nearest_editable_text_hit(px, py, Some(&anchor.text)))
                .or_else(|| self.find_text_hit_at_point(acx, acy).filter(|h| h.editable))
                .or_else(|| self.find_nearest_text_hit_at_point(acx, acy).filter(|h| h.editable));

            if let Some(hit) = mapped_hit.or(fallback_hit) {
                self.clear_selected_image_overlay();
                self.set_selected_text_selection(Some(hit.clone()));
                self.open_text_insert_panel_with_text(&hit.text);
                self.sync_inserted_anchor_from_hit(self.current_page, &hit);
                self.emit(DocumentEvent::StatusChanged {
                    message: "Text panel opened for inserted text".into(),
                });
                return;
            }
            self.clear_selected_image_overlay();
            // If we cannot map anchor to an editable op, keep a visual selection
            // marker on the clicked anchor bounds.
            self.set_selected_text_selection(Some(TextHit {
                text: anchor.text.clone(),
                stream_id: (0, 0),
                op_index: 0,
                editable: false,
                x: anchor.x,
                y: anchor.y,
                width: anchor.width,
                height: anchor.height,
            }));
            self.open_text_insert_panel_with_text(&anchor.text);
            self.emit(DocumentEvent::StatusChanged {
                message: "Inserted text opened (save target not resolved)".into(),
            });
            return;
        }

        let text_hit = self
            .find_text_hit_at_point(px, py)
            .filter(|h| h.editable)
            .or_else(|| self.find_nearest_text_hit_at_point(px, py).filter(|h| h.editable))
            .or_else(|| self.find_text_hit_at_point(px, py))
            .or_else(|| self.find_nearest_text_hit_at_point(px, py));
        if let Some(hit) = text_hit {
            self.clear_selected_image_overlay();
            self.set_selected_text_selection(Some(hit.clone()));
            let selected_text = if hit.editable {
                hit.text.clone()
            } else {
                self.compose_text_run_around_hit(&hit)
            };
            self.open_text_insert_panel_with_text(&selected_text);
            self.emit(DocumentEvent::StatusChanged {
                message: "Text panel opened for selected text".into(),
            });
            return;
        }

        // Click outside image/text: clear active image selection/overlay and
        // keep a visible status message so misses are diagnosable.
        self.clear_selected_image_overlay();
        self.clear_selected_text_selection();
        if notify_miss {
            let has_text = !self.collect_text_hits_on_page().is_empty();
            let has_image = !self.page_image_resource_names().is_empty();
            self.emit(DocumentEvent::StatusChanged {
                message: format!(
                    "No editable text or image found at click location (text_detected={}, image_resources={})",
                    has_text,
                    has_image
                )
                .into(),
            });
        }
    }

    fn start_canvas_image_drag(&mut self, canvas_x: f32, canvas_y: f32) {
        // A previous drag can set click suppression; clear it on the next
        // gesture start so selection/editing does not get stuck.
        self.suppress_next_canvas_click = false;

        if !self.ensure_document_open() {
            return;
        }
        if self.pending_annotation_tool.is_some() {
            return;
        }
        let Some((px, py)) = self.canvas_to_pdf_point(canvas_x, canvas_y) else {
            return;
        };
        let Some(hit) = self
            .find_image_hit_at_point(px, py)
            .or_else(|| self.find_nearest_image_hit_at_point(px, py))
        else {
            return;
        };

        self.selected_image_resource_name = Some(hit.resource_name.clone());
        self.selected_image_target = Some((hit.stream_id, hit.do_op_index));
        self.selected_image_hit = Some(hit.clone());
        self.image_overlay_visible = true;
        self.image_overlay_custom_canvas_pos = None;
        self.update_image_overlay_display(Some(&hit));
        self.update_image_selection_display(Some(&hit));

        self.image_drag_active = true;
        self.image_drag_start_canvas = Some((canvas_x, canvas_y));
        self.image_drag_current_canvas = Some((canvas_x, canvas_y));
        self.image_drag_start_hit = Some(hit);
        if let Some(win) = self.window.upgrade() {
            win.set_image_drag_active(true);
        }
    }

    fn move_canvas_image_drag(&mut self, canvas_x: f32, canvas_y: f32) {
        if self.image_drag_active {
            self.image_drag_current_canvas = Some((canvas_x, canvas_y));
        }
    }

    fn end_canvas_image_drag(&mut self, canvas_x: f32, canvas_y: f32) {
        if !self.image_drag_active {
            return;
        }

        self.image_drag_active = false;
        if let Some(win) = self.window.upgrade() {
            win.set_image_drag_active(false);
        }

        self.image_drag_current_canvas = Some((canvas_x, canvas_y));
        let Some((sx, sy)) = self.image_drag_start_canvas else {
            return;
        };
        let Some((cx, cy)) = self.image_drag_current_canvas else {
            return;
        };
        let Some(hit) = self.image_drag_start_hit.clone() else {
            return;
        };
        self.image_drag_start_canvas = None;
        self.image_drag_current_canvas = None;
        self.image_drag_start_hit = None;

        let moved_canvas = ((cx - sx).powi(2) + (cy - sy).powi(2)).sqrt();
        // Treat tiny pointer jitter as a click to keep repeated image selection
        // reliable and avoid accidental micro-drags.
        if moved_canvas < 4.0 {
            return;
        }

        let Some((spx, spy)) = self.canvas_to_pdf_point(sx, sy) else {
            return;
        };
        let Some((cpx, cpy)) = self.canvas_to_pdf_point(cx, cy) else {
            return;
        };
        let dx = cpx - spx;
        let dy = cpy - spy;
        if dx.abs() < 0.1 && dy.abs() < 0.1 {
            return;
        }

        self.suppress_next_canvas_click = true;

        self.pin_image_overlay_position();
        let mut next = hit.matrix;
        next[4] += dx;
        next[5] += dy;
        let cmd = Box::new(UpdateImageTransformCommand::new(
            self.current_page,
            hit.stream_id,
            hit.cm_op_index,
            hit.matrix,
            next,
        ));
        self.run_tool_command(cmd, "Moved image");

        let refreshed = self
            .selected_image_target
            .and_then(|(sid, do_idx)| self.find_image_hit_by_target(sid, do_idx))
            .or_else(|| {
                self.selected_image_resource_name
                    .as_ref()
                    .and_then(|name| self.find_last_image_hit_by_resource_name(name))
            });
        if let Some(ref h) = refreshed {
            self.selected_image_target = Some((h.stream_id, h.do_op_index));
        }
        self.selected_image_hit = refreshed.clone();
        self.update_image_overlay_display(refreshed.as_ref());
        self.update_image_selection_display(refreshed.as_ref());
    }

    fn pin_image_overlay_position(&mut self) {
        if !self.image_overlay_visible {
            return;
        }
        if let Some(win) = self.window.upgrade() {
            self.image_overlay_custom_canvas_pos = Some((
                win.get_image_edit_controls_x(),
                win.get_image_edit_controls_y(),
            ));
        }
    }

    fn find_note_at_point(&self, px: f32, py: f32) -> Option<((u32, u16), String)> {
        let notes = self
            .document
            .as_ref()
            .map(|d| read_annotations(d, self.current_page))
            .unwrap_or_default();

        notes.into_iter().find_map(|ann| {
            let object_id = ann.object_id?;
            let AnnotationKind::Note { content, .. } = ann.kind else {
                return None;
            };

            // Tolerance makes small note icons easier to click at different zoom levels.
            let tol = 8.0;
            let x0 = ann.rect.x - tol;
            let y0 = ann.rect.y - tol;
            let x1 = ann.rect.x + ann.rect.width + tol;
            let y1 = ann.rect.y + ann.rect.height + tol;
            if px >= x0 && px <= x1 && py >= y0 && py <= y1 {
                Some((object_id, content))
            } else {
                None
            }
        })
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
                    self.cache
                        .lock()
                        .expect("PageCache mutex was poisoned")
                        .evict_document(doc.id);
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
        if !self.ensure_document_open() {
            return;
        }
        self.clear_selected_text_selection();
        self.open_text_insert_panel_with_text("New text");
        self.emit(DocumentEvent::StatusChanged {
            message: "Text insert panel opened".into(),
        });
    }

    fn open_text_insert_panel_with_text(&mut self, text: &str) {
        self.clear_selected_image_overlay();
        self.text_insert_panel_visible = true;
        if let Some(win) = self.window.upgrade() {
            win.set_text_edit_content_text(SharedString::from(text));
        }
        self.update_text_insert_panel_display();
    }

    fn apply_text_insert_from_panel(&mut self, content_text: String, font_text: String, size_text: String) {
        if !self.ensure_document_open() {
            return;
        }

        if self.selected_text_hit.is_some() {
            self.apply_text_save_from_panel(content_text);
            return;
        }

        let text = content_text.trim().to_owned();
        if text.is_empty() {
            self.emit(DocumentEvent::StatusChanged {
                message: "Insert text canceled (empty text)".into(),
            });
            return;
        }

        let font_size = size_text
            .trim()
            .parse::<f32>()
            .ok()
            .filter(|v| *v > 1.0)
            .unwrap_or(14.0);
        let preferred_font = {
            let f = font_text.trim();
            if f.is_empty() {
                None
            } else {
                Some(f.to_owned())
            }
        };

        let Some((x, y)) = self.next_insert_text_position(&text, font_size) else {
            self.emit(DocumentEvent::StatusChanged {
                message: "Insert text failed (page is not available)".into(),
            });
            return;
        };

        let cmd = Box::new(InsertTextCommand::new_with_font(
            self.current_page,
            text.clone(),
            x,
            y,
            font_size,
            preferred_font,
        ));
        self.run_tool_command(cmd, "Inserted text");
        self.remember_inserted_text_anchor(self.current_page, &text, x, y, font_size);
        self.text_insert_panel_visible = true;
        self.update_text_insert_panel_display();
    }

    fn apply_text_save_from_panel(&mut self, content_text: String) {
        let Some(hit) = self.selected_text_hit.clone() else {
            return;
        };

        let new_text = content_text.trim().to_owned();
        if new_text.is_empty() {
            self.emit(DocumentEvent::StatusChanged {
                message: "Save canceled (empty text)".into(),
            });
            return;
        }

        if !hit.editable {
            self.emit(DocumentEvent::StatusChanged {
                message: "Selected text is not directly editable".into(),
            });
            return;
        }

        if new_text == hit.text {
            self.emit(DocumentEvent::StatusChanged {
                message: "Text unchanged".into(),
            });
            return;
        }

        let cmd = Box::new(UpdateTextAtOpCommand::new(
            self.current_page,
            hit.stream_id,
            hit.op_index,
            hit.text.clone(),
            new_text.clone(),
        ));
        self.run_tool_command(cmd, "Saved text");

        // Keep panel open in Save mode so repeated edits are straightforward.
        if let Some(updated_hit) = self
            .find_text_hit_by_target(hit.stream_id, hit.op_index)
            .or_else(|| {
                self.find_nearest_editable_text_hit(
                    hit.x + hit.width * 0.5,
                    hit.y + hit.height * 0.5,
                    Some(&new_text),
                )
            })
        {
            self.set_selected_text_selection(Some(updated_hit));
            if let Some(sel) = self.selected_text_hit.clone() {
                self.sync_inserted_anchor_from_hit(self.current_page, &sel);
            }
        }
        if let Some(win) = self.window.upgrade() {
            win.set_text_edit_content_text(SharedString::from(new_text));
        }
        self.text_insert_panel_visible = true;
        self.update_text_insert_panel_display();
    }

    fn apply_text_delete_from_panel(&mut self) {
        if !self.ensure_document_open() {
            return;
        }
        let Some(hit) = self.selected_text_hit.clone() else {
            self.emit(DocumentEvent::StatusChanged {
                message: "Select text first".into(),
            });
            return;
        };
        if !hit.editable {
            self.emit(DocumentEvent::StatusChanged {
                message: "Selected text is not directly deletable".into(),
            });
            return;
        }

        let cmd = Box::new(DeleteTextAtOpCommand::new(
            self.current_page,
            hit.stream_id,
            hit.op_index,
            hit.text.clone(),
        ));
        self.run_tool_command(cmd, "Deleted text");

        self.clear_selected_text_selection();
        self.text_insert_panel_visible = false;
        self.update_text_insert_panel_display();
    }

    fn remember_inserted_text_anchor(
        &mut self,
        page_index: u32,
        text: &str,
        x: f32,
        y: f32,
        font_size: f32,
    ) {
        let width = (text.chars().count() as f32 * font_size * 0.55).max(10.0);
        let y_bottom = -font_size * 0.35;
        let y_top = font_size * 0.95;
        let mut resolved_target: Option<TextHit> = None;
        if page_index == self.current_page {
            let acx = x + width * 0.5;
            let acy = y + y_bottom + ((y_top - y_bottom) * 0.5);
            resolved_target = self
                .find_nearest_editable_text_hit(acx, acy, Some(text))
                .or_else(|| self.find_nearest_editable_text_hit(x, y, Some(text)));
        }

        let anchor = InsertedTextAnchor {
            text: text.to_owned(),
            x,
            y: y + y_bottom,
            width,
            height: (y_top - y_bottom).max(8.0),
            target_stream_id: None,
            target_op_index: None,
        };

        let entry = self.inserted_text_anchors_by_page.entry(page_index).or_default();
        entry.push(anchor);
        if let Some(hit) = resolved_target {
            if let Some(last) = entry.last_mut() {
                last.target_stream_id = Some(hit.stream_id);
                last.target_op_index = Some(hit.op_index);
                last.text = hit.text.clone();
                last.x = hit.x;
                last.y = hit.y;
                last.width = hit.width;
                last.height = hit.height;
            }
        }
        if entry.len() > 64 {
            let excess = entry.len() - 64;
            entry.drain(0..excess);
        }
    }

    fn sync_inserted_anchor_from_hit(&mut self, page_index: u32, hit: &TextHit) {
        let Some(anchors) = self.inserted_text_anchors_by_page.get_mut(&page_index) else {
            return;
        };

        if let Some(anchor) = anchors.iter_mut().find(|a| {
            a.target_stream_id == Some(hit.stream_id) && a.target_op_index == Some(hit.op_index)
        }) {
            anchor.text = hit.text.clone();
            anchor.x = hit.x;
            anchor.y = hit.y;
            anchor.width = hit.width;
            anchor.height = hit.height;
            return;
        }

        if let Some(anchor) = anchors.iter_mut().find(|a| {
            let acx = a.x + a.width * 0.5;
            let acy = a.y + a.height * 0.5;
            let hcx = hit.x + hit.width * 0.5;
            let hcy = hit.y + hit.height * 0.5;
            let dist = ((acx - hcx).powi(2) + (acy - hcy).powi(2)).sqrt();
            dist <= 28.0
        }) {
            anchor.target_stream_id = Some(hit.stream_id);
            anchor.target_op_index = Some(hit.op_index);
            anchor.text = hit.text.clone();
            anchor.x = hit.x;
            anchor.y = hit.y;
            anchor.width = hit.width;
            anchor.height = hit.height;
        }
    }

    fn find_inserted_text_anchor_at_point(&self, px: f32, py: f32) -> Option<InsertedTextAnchor> {
        let anchors = self.inserted_text_anchors_by_page.get(&self.current_page)?;
        anchors
            .iter()
            .filter_map(|a| {
                let tol = 24.0f32;
                let dx = if px < a.x - tol {
                    (a.x - tol) - px
                } else if px > a.x + a.width + tol {
                    px - (a.x + a.width + tol)
                } else {
                    0.0
                };
                let dy = if py < a.y - tol {
                    (a.y - tol) - py
                } else if py > a.y + a.height + tol {
                    py - (a.y + a.height + tol)
                } else {
                    0.0
                };
                let dist = (dx * dx + dy * dy).sqrt();
                if dist <= 64.0 {
                    Some((a.clone(), dist))
                } else {
                    None
                }
            })
            .min_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal))
            .map(|(anchor, _)| anchor)
    }

    fn sync_text_font_name_from_input(&mut self, font_text: String) {
        let trimmed = font_text.trim();
        let canonical = Self::canonical_standard_font_name(trimmed).unwrap_or(trimmed);
        if let Some(win) = self.window.upgrade() {
            if win.get_text_edit_font_text().as_str() != canonical {
                win.set_text_edit_font_text(SharedString::from(canonical));
            }
        }
    }

    fn canonical_standard_font_name(input: &str) -> Option<&'static str> {
        let known = [
            "Helvetica",
            "Helvetica-Bold",
            "Helvetica-Oblique",
            "Helvetica-BoldOblique",
            "Times-Roman",
            "Times-Bold",
            "Times-Italic",
            "Times-BoldItalic",
            "Courier",
            "Courier-Bold",
            "Courier-Oblique",
            "Courier-BoldOblique",
        ];
        known.into_iter().find(|name| name.eq_ignore_ascii_case(input))
    }

    fn clear_selected_text_selection(&mut self) {
        self.selected_text_hit = None;
        self.update_text_selection_display();
    }

    fn set_selected_text_selection(&mut self, hit: Option<TextHit>) {
        self.selected_text_hit = hit;
        self.update_text_selection_display();
    }

    fn update_text_selection_display(&self) {
        if let Some(win) = self.window.upgrade() {
            if let Some(hit) = &self.selected_text_hit {
                let p0 = self.pdf_to_canvas_point(hit.x, hit.y);
                let p1 = self.pdf_to_canvas_point(hit.x + hit.width, hit.y + hit.height);
                if let (Some((x0, y0)), Some((x1, y1))) = (p0, p1) {
                    let x = x0.min(x1);
                    let y = y0.min(y1);
                    let w = (x1 - x0).abs().max(14.0);
                    let h = (y1 - y0).abs().max(14.0);
                    win.set_text_selection_x(x);
                    win.set_text_selection_y(y);
                    win.set_text_selection_width(w);
                    win.set_text_selection_height(h);
                    win.set_text_selection_visible(true);
                } else {
                    win.set_text_selection_visible(false);
                }
                win.set_text_edit_save_mode(true);
            } else {
                win.set_text_selection_visible(false);
                win.set_text_edit_save_mode(false);
            }
        }
    }

    fn update_text_insert_panel_display(&self) {
        if let Some(win) = self.window.upgrade() {
            win.set_text_edit_controls_visible(self.text_insert_panel_visible);
            win.set_text_edit_save_mode(self.selected_text_hit.is_some());
            if self.text_insert_panel_visible {
                if win.get_text_edit_content_text().is_empty() {
                    win.set_text_edit_content_text(SharedString::from("New text"));
                }
                if win.get_text_edit_font_text().is_empty() {
                    win.set_text_edit_font_text(SharedString::from("Helvetica"));
                }
                if win.get_text_edit_size_text().is_empty() {
                    win.set_text_edit_size_text(SharedString::from("14"));
                }
                win.set_text_edit_controls_x(12.0);
                win.set_text_edit_controls_y(12.0);
            }
        }
    }

    fn next_insert_text_position(&mut self, text: &str, font_size: f32) -> Option<(f32, f32)> {
        let page = self
            .document
            .as_ref()
            .and_then(|d| d.get_page(self.current_page).ok())?;

        let page_w = page.media_box.width as f32;
        let page_h = page.media_box.height as f32;
        let margin_right = 36.0f32;
        let margin_top = 36.0f32;
        let min_y = 24.0f32;
        let line_step = (font_size * 1.5).max(18.0);

        let estimated_w = (text.chars().count() as f32 * font_size * 0.55).max(24.0);
        let x = (page_w - margin_right - estimated_w).max(12.0);

        let start_y = (page_h - margin_top).max(min_y);
        if !self
            .insert_text_next_y_by_page
            .contains_key(&self.current_page)
        {
            // On first insert after open/reopen, derive cursor from existing
            // right-side text so stacking continues where the user left off.
            let next_from_existing = {
                let right_threshold = page_w * 0.55;
                let existing = self.collect_text_hits_on_page();
                let lowest_right_y = existing
                    .into_iter()
                    .filter(|h| h.x >= right_threshold)
                    .map(|h| h.y)
                    .min_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));

                lowest_right_y.map(|lowest| lowest - line_step)
            };

            let initial_y = next_from_existing
                .map(|v| v.clamp(min_y, page_h - 8.0))
                .unwrap_or(start_y);
            self.insert_text_next_y_by_page
                .insert(self.current_page, initial_y);
        }

        let next_y_ref = self
            .insert_text_next_y_by_page
            .entry(self.current_page)
            .or_insert(start_y);

        let y = (*next_y_ref).clamp(min_y, page_h - 8.0);
        let candidate_next = y - line_step;
        *next_y_ref = if candidate_next < min_y {
            start_y
        } else {
            candidate_next
        };

        Some((x, y))
    }

    fn tool_modify_text(&mut self) {
        let Some(raw_old) = Self::prompt_input("Modify Text", "Find text:", "") else {
            self.emit(DocumentEvent::StatusChanged {
                message: "Modify text canceled".into(),
            });
            return;
        };
        let old_text = raw_old.trim().to_owned();
        if old_text.is_empty() {
            self.emit(DocumentEvent::StatusChanged {
                message: "Modify text canceled (empty find text)".into(),
            });
            return;
        }
        let Some(raw_new) = Self::prompt_input("Modify Text", "Replace with:", "") else {
            self.emit(DocumentEvent::StatusChanged {
                message: "Modify text canceled".into(),
            });
            return;
        };
        let cmd = Box::new(ModifyTextCommand::new(self.current_page, old_text, raw_new));
        self.run_tool_command(cmd, "Modified text on page");
    }

    fn tool_font_substitution(&mut self) {
        let Some(raw_old) = Self::prompt_input(
            "Font Substitution",
            "Replace font resource name (e.g. Helvetica):",
            "Helvetica",
        ) else {
            self.emit(DocumentEvent::StatusChanged {
                message: "Font substitution canceled".into(),
            });
            return;
        };
        let old_font = raw_old.trim().to_owned();
        if old_font.is_empty() {
            self.emit(DocumentEvent::StatusChanged {
                message: "Font substitution canceled (empty source font)".into(),
            });
            return;
        }
        let Some(raw_new) = Self::prompt_input(
            "Font Substitution",
            "New font resource name (standard PDF font):",
            "Times-Roman",
        ) else {
            self.emit(DocumentEvent::StatusChanged {
                message: "Font substitution canceled".into(),
            });
            return;
        };
        let new_font = raw_new.trim().to_owned();
        if new_font.is_empty() {
            self.emit(DocumentEvent::StatusChanged {
                message: "Font substitution canceled (empty target font)".into(),
            });
            return;
        }
        let cmd = Box::new(FontSubstitutionCommand::new(
            self.current_page,
            old_font,
            new_font,
        ));
        self.run_tool_command(cmd, "Applied font substitution");
    }

    fn tool_insert_image(&mut self) {
        let before = self.page_image_resource_names();
        let Some((data, w, h, source_path)) = self.pick_image_rgb_data() else {
            self.emit(DocumentEvent::StatusChanged {
                message: "Insert image canceled".into(),
            });
            return;
        };

        let display_w = 160.0f32;
        let display_h = (display_w * h as f32 / w as f32).max(24.0);

        let cmd = Box::new(InsertImageCommand::new(
            self.current_page,
            data,
            w,
            h,
            72.0,
            500.0,
            display_w,
            display_h,
        ));
        self.run_tool_command(
            cmd,
            &format!("Inserted image from {}", source_path.display()),
        );

        let after = self.page_image_resource_names();
        if !after.is_empty() {
            let chosen = after
                .iter()
                .find(|name| !before.iter().any(|old| old == *name))
                .cloned()
                .or_else(|| after.last().cloned());
            self.selected_image_resource_name = chosen;
            self.selected_image_target = None;
            self.image_overlay_visible = true;
            self.image_overlay_custom_canvas_pos = None;
            self.update_image_resource_menu_display();
            let hit = self
                .selected_image_resource_name
                .as_ref()
                .and_then(|name| self.find_last_image_hit_by_resource_name(name));
            if let Some(ref h) = hit {
                self.selected_image_target = Some((h.stream_id, h.do_op_index));
            }
            self.selected_image_hit = hit.clone();
            self.update_image_overlay_display(hit.as_ref());
            self.update_image_selection_display(hit.as_ref());
        }
    }

    fn tool_replace_image(&mut self) {
        self.tool_replace_selected_image_resource();
    }

    fn tool_edit_image_resource(&mut self, index: i32) {
        if !self.ensure_document_open() {
            return;
        }
        if index < 0 {
            self.emit(DocumentEvent::StatusChanged {
                message: "Image edit canceled".into(),
            });
            return;
        }

        let names = self.page_image_resource_names();
        if names.is_empty() {
            self.emit(DocumentEvent::StatusChanged {
                message: "No image resources found on current page".into(),
            });
            return;
        }

        let selected = self.image_resource_menu_offset + index as usize;
        let Some(resource_name) = names.get(selected) else {
            self.emit(DocumentEvent::StatusChanged {
                message: "Selected image resource is not available".into(),
            });
            return;
        };
        self.selected_image_resource_name = Some(resource_name.clone());
        self.image_overlay_visible = true;
        self.image_overlay_custom_canvas_pos = None;
        self.update_image_resource_menu_display();
        let hit = self.find_last_image_hit_by_resource_name(&resource_name);
        if let Some(ref h) = hit {
            self.selected_image_target = Some((h.stream_id, h.do_op_index));
        }
        self.selected_image_hit = hit.clone();
        self.update_image_overlay_display(hit.as_ref());
        self.update_image_selection_display(hit.as_ref());
        self.emit(DocumentEvent::StatusChanged {
            message: format!("Selected image resource '{}'", resource_name),
        });
    }

    fn selected_image_resource_name(&self) -> Option<String> {
        let names = self.page_image_resource_names();
        if names.is_empty() {
            return None;
        }
        if let Some(selected) = &self.selected_image_resource_name {
            if names.iter().any(|n| n == selected) {
                return Some(selected.clone());
            }
        }
        names.last().cloned()
    }

    fn apply_selected_image_action(&mut self, action: &str) {
        if !self.ensure_document_open() {
            return;
        }
        let Some(resource_name) = self.selected_image_resource_name() else {
            self.emit(DocumentEvent::StatusChanged {
                message: "No image resources found on current page".into(),
            });
            return;
        };

        self.selected_image_resource_name = Some(resource_name.clone());
        self.update_image_resource_menu_display();

        if action == "replace" {
            self.replace_image_resource_from_dialog(&resource_name);
            let hit = self
                .selected_image_target
                .and_then(|(sid, do_idx)| self.find_image_hit_by_target(sid, do_idx))
                .or_else(|| self.find_last_image_hit_by_resource_name(&resource_name));
            if let Some(ref h) = hit {
                self.selected_image_target = Some((h.stream_id, h.do_op_index));
            }
            self.selected_image_hit = hit.clone();
            self.update_image_overlay_display(hit.as_ref());
            self.update_image_selection_display(hit.as_ref());
            return;
        }

        let Some(hit) = self
            .selected_image_target
            .and_then(|(sid, do_idx)| self.find_image_hit_by_target(sid, do_idx))
            .or_else(|| self.find_last_image_hit_by_resource_name(&resource_name))
        else {
            self.emit(DocumentEvent::StatusChanged {
                message: format!(
                    "Could not locate '{}' placement in page content for transform",
                    resource_name
                ),
            });
            return;
        };

        self.apply_image_transform_action(hit, action);
        let refreshed = self
            .selected_image_target
            .and_then(|(sid, do_idx)| self.find_image_hit_by_target(sid, do_idx))
            .or_else(|| self.find_last_image_hit_by_resource_name(&resource_name));
        if let Some(ref h) = refreshed {
            self.selected_image_target = Some((h.stream_id, h.do_op_index));
        }
        self.selected_image_hit = refreshed.clone();
        self.update_image_overlay_display(refreshed.as_ref());
        self.update_image_selection_display(refreshed.as_ref());
    }

    fn tool_replace_selected_image_resource(&mut self) {
        self.apply_selected_image_action("replace");
    }

    fn tool_delete_selected_image_placement(&mut self) {
        if !self.ensure_document_open() {
            return;
        }

        let Some(hit) = self
            .selected_image_target
            .and_then(|(sid, do_idx)| self.find_image_hit_by_target(sid, do_idx))
            .or_else(|| {
                self.selected_image_resource_name
                    .as_ref()
                    .and_then(|name| self.find_last_image_hit_by_resource_name(name))
            })
        else {
            self.emit(DocumentEvent::StatusChanged {
                message: "No selected image to delete".into(),
            });
            return;
        };

        let cmd = Box::new(DeleteImagePlacementCommand::new(
            self.current_page,
            hit.stream_id,
            hit.do_op_index,
        ));
        self.run_tool_command(cmd, "Deleted image");

        self.selected_image_target = None;
        self.selected_image_hit = None;
        self.image_overlay_visible = false;
        self.update_image_overlay_display(None);
        self.update_image_selection_display(None);
    }

    fn clear_selected_image_overlay(&mut self) {
        self.image_overlay_visible = false;
        self.selected_image_target = None;
        self.selected_image_hit = None;
        self.image_overlay_custom_canvas_pos = None;
        self.image_drag_active = false;
        self.image_drag_start_canvas = None;
        self.image_drag_current_canvas = None;
        self.image_drag_start_hit = None;
        self.suppress_next_canvas_click = false;
        if let Some(win) = self.window.upgrade() {
            win.set_image_drag_active(false);
        }
        self.update_image_overlay_display(None);
        self.update_image_selection_display(None);
    }

    fn update_image_selection_display(&self, _hit: Option<&ImageHit>) {}

    fn clamp_image_overlay_canvas_pos(&self, x: f32, y: f32) -> (f32, f32) {
        let Some(doc) = &self.document else {
            return (x, y);
        };
        let Ok(page) = doc.get_page(self.current_page) else {
            return (x, y);
        };
        let w = page.media_box.width as f32;
        let h = page.media_box.height as f32;
        let rotation = self.page_rotation_degrees();
        let (display_w, display_h) = if rotation == 90 || rotation == 270 {
            (h, w)
        } else {
            (w, h)
        };
        let canvas_w = display_w * self.zoom;
        let canvas_h = display_h * self.zoom;
        let overlay_w = IMAGE_EDIT_PANEL_WIDTH;
        let overlay_h = IMAGE_EDIT_PANEL_HEIGHT;
        (
            x.clamp(4.0, (canvas_w - overlay_w - 4.0).max(4.0)),
            y.clamp(4.0, (canvas_h - overlay_h - 4.0).max(4.0)),
        )
    }

    fn update_image_overlay_display(&self, hit: Option<&ImageHit>) {
        if let Some(win) = self.window.upgrade() {
            if !self.image_overlay_visible {
                win.set_image_edit_controls_visible(false);
                return;
            }

            let mut overlay_hit = hit.cloned();
            if overlay_hit.is_none() {
                overlay_hit = self.selected_image_hit.clone();
            }
            if overlay_hit.is_none() {
                if let Some((sid, do_idx)) = self.selected_image_target {
                    overlay_hit = self.find_image_hit_by_target(sid, do_idx);
                }
            }
            if overlay_hit.is_none() {
                if let Some(name) = &self.selected_image_resource_name {
                    overlay_hit = self.find_last_image_hit_by_resource_name(name);
                }
            }

            let Some(hit) = overlay_hit else {
                win.set_image_edit_controls_visible(false);
                return;
            };

            let Some(doc) = &self.document else {
                win.set_image_edit_controls_visible(false);
                return;
            };
            let Ok(page) = doc.get_page(self.current_page) else {
                win.set_image_edit_controls_visible(false);
                return;
            };

            let w = page.media_box.width as f32;
            let h = page.media_box.height as f32;
            let rotation = self.page_rotation_degrees();
            let (display_w, display_h) = if rotation == 90 || rotation == 270 {
                (h, w)
            } else {
                (w, h)
            };
            let canvas_w = display_w * self.zoom;
            let canvas_h = display_h * self.zoom;

            let overlay_w = IMAGE_EDIT_PANEL_WIDTH;
            let overlay_h = IMAGE_EDIT_PANEL_HEIGHT;

            let p00_pdf = Self::transform_point(hit.matrix, 0.0, 0.0);
            let p10_pdf = Self::transform_point(hit.matrix, 1.0, 0.0);
            let p01_pdf = Self::transform_point(hit.matrix, 0.0, 1.0);
            let p11_pdf = Self::transform_point(hit.matrix, 1.0, 1.0);
            let p00 = self.pdf_to_canvas_point(p00_pdf.0, p00_pdf.1);
            let p10 = self.pdf_to_canvas_point(p10_pdf.0, p10_pdf.1);
            let p01 = self.pdf_to_canvas_point(p01_pdf.0, p01_pdf.1);
            let p11 = self.pdf_to_canvas_point(p11_pdf.0, p11_pdf.1);

            let (img_x_min, img_x_max, img_y_min, img_y_max) = if let (
                Some(p00),
                Some(p10),
                Some(p01),
                Some(p11),
            ) = (p00, p10, p01, p11)
            {
                let xs = [p00.0, p10.0, p01.0, p11.0];
                let ys = [p00.1, p10.1, p01.1, p11.1];
                let mut x_min = xs[0];
                let mut x_max = xs[0];
                let mut y_min = ys[0];
                let mut y_max = ys[0];
                for x in xs {
                    x_min = x_min.min(x);
                    x_max = x_max.max(x);
                }
                for y in ys {
                    y_min = y_min.min(y);
                    y_max = y_max.max(y);
                }
                (x_min, x_max, y_min, y_max)
            } else {
                let Some((ax, ay)) = self.pdf_to_canvas_point(hit.x_max, hit.y_max) else {
                    win.set_image_edit_controls_visible(false);
                    return;
                };
                (ax, ax, ay, ay)
            };

            let margin = 4.0f32;
            let gap = 8.0f32;
            let x_max_allowed = (canvas_w - overlay_w - margin).max(margin);
            let y_max_allowed = (canvas_h - overlay_h - margin).max(margin);

            let candidates = [
                (img_x_max + gap, img_y_min - overlay_h - gap), // right-top
                (img_x_min - overlay_w - gap, img_y_min - overlay_h - gap), // left-top
                (img_x_max + gap, img_y_max + gap),             // right-bottom
                (img_x_min - overlay_w - gap, img_y_max + gap), // left-bottom
            ];

            let mut placed = None;
            for (cx, cy) in candidates {
                if cx >= margin
                    && cy >= margin
                    && cx <= x_max_allowed
                    && cy <= y_max_allowed
                {
                    placed = Some((cx, cy));
                    break;
                }
            }

            let (fallback_x, fallback_y) = candidates[0];
            let (anchored_x, anchored_y) = placed.unwrap_or((
                fallback_x.clamp(margin, x_max_allowed),
                fallback_y.clamp(margin, y_max_allowed),
            ));
            let (x, y) = self
                .image_overlay_custom_canvas_pos
                .map(|(cx, cy)| self.clamp_image_overlay_canvas_pos(cx, cy))
                .unwrap_or((anchored_x, anchored_y));

            let width = (hit.matrix[0].powi(2) + hit.matrix[1].powi(2)).sqrt().max(1.0);
            let height = (hit.matrix[2].powi(2) + hit.matrix[3].powi(2)).sqrt().max(1.0);
            win.set_image_edit_width_text(SharedString::from(format!("{:.0}", width)));
            win.set_image_edit_height_text(SharedString::from(format!("{:.0}", height)));
            if win.get_image_edit_keep_aspect() == false {
                win.set_image_edit_keep_aspect(true);
            }

            win.set_image_edit_controls_x(x);
            win.set_image_edit_controls_y(y);
            win.set_image_edit_controls_visible(true);
        }
    }

    fn pdf_to_canvas_point(&self, px: f32, py: f32) -> Option<(f32, f32)> {
        let doc = self.document.as_ref()?;
        let page = doc.get_page(self.current_page).ok()?;
        let w = page.media_box.width as f32;
        let h = page.media_box.height as f32;
        let rotation = self.page_rotation_degrees();

        let (display_w, display_h) = if rotation == 90 || rotation == 270 {
            (h, w)
        } else {
            (w, h)
        };

        let (xd, yd) = match rotation {
            90 => (py, w - px),
            180 => (w - px, h - py),
            270 => (h - py, px),
            _ => (px, py),
        };

        let xd = xd.clamp(0.0, display_w);
        let yd = yd.clamp(0.0, display_h);
        let canvas_x = xd * self.zoom;
        let canvas_y = (display_h - yd) * self.zoom;
        Some((canvas_x, canvas_y))
    }

    fn shift_image_resource_menu_page(&mut self, delta_pages: isize) {
        if !self.ensure_document_open() {
            return;
        }

        let names = self.page_image_resource_names();
        if names.is_empty() {
            self.image_resource_menu_offset = 0;
            self.update_image_resource_menu_display();
            return;
        }

        let page_size = MAX_IMAGE_RESOURCE_MENU_ITEMS as isize;
        let mut next = self.image_resource_menu_offset as isize + delta_pages * page_size;
        if next < 0 {
            next = 0;
        }

        let max_start = ((names.len() - 1) / MAX_IMAGE_RESOURCE_MENU_ITEMS) * MAX_IMAGE_RESOURCE_MENU_ITEMS;
        let max_start = max_start as isize;
        if next > max_start {
            next = max_start;
        }

        self.image_resource_menu_offset = next as usize;
        self.update_image_resource_menu_display();
    }

    fn page_image_resource_names(&self) -> Vec<String> {
        let Some(doc) = &self.document else {
            return Vec::new();
        };
        let Ok(page) = doc.get_page(self.current_page) else {
            return Vec::new();
        };
        let inner = doc.inner();
        let mut current_id = page.object_id;
        let mut resources_obj: Option<Object> = None;

        // Walk up page tree to resolve inherited /Resources.
        loop {
            let Ok(node_obj) = inner.get_object(current_id) else {
                break;
            };
            let Ok(node_dict) = node_obj.as_dict() else {
                break;
            };

            if let Ok(res) = node_dict.get(b"Resources") {
                resources_obj = Some(res.clone());
                break;
            }

            let Some(parent_id) = node_dict.get(b"Parent").ok().and_then(|p| p.as_reference().ok())
            else {
                break;
            };
            current_id = parent_id;
        }

        let Some(resources_obj) = resources_obj else {
            return Vec::new();
        };

        let resources_dict = match resources_obj {
            Object::Reference(res_id) => inner.get_object(res_id).ok().and_then(|o| o.as_dict().ok()),
            Object::Dictionary(ref d) => Some(d),
            _ => None,
        };
        let Some(resources_dict) = resources_dict else {
            return Vec::new();
        };

        let xobject_obj = resources_dict.get(b"XObject").ok().cloned();
        let Some(xobject_obj) = xobject_obj else {
            return Vec::new();
        };

        let xobject_dict = match xobject_obj {
            Object::Reference(xo_id) => inner.get_object(xo_id).ok().and_then(|o| o.as_dict().ok()),
            Object::Dictionary(ref d) => Some(d),
            _ => None,
        };
        let Some(xobject_dict) = xobject_dict else {
            return Vec::new();
        };

        let mut names = BTreeSet::new();
        for (name_bytes, obj) in xobject_dict.iter() {
            let subtype_is_image = match obj {
                Object::Reference(id) => inner
                    .get_object(*id)
                    .ok()
                    .and_then(|o| o.as_stream().ok())
                    .and_then(|s| s.dict.get(b"Subtype").ok())
                    .and_then(|s| s.as_name().ok())
                    .map(|n| n == b"Image")
                    .unwrap_or(false),
                Object::Stream(stream) => stream
                    .dict
                    .get(b"Subtype")
                    .ok()
                    .and_then(|s| s.as_name().ok())
                    .map(|n| n == b"Image")
                    .unwrap_or(false),
                _ => false,
            };

            if subtype_is_image {
                names.insert(String::from_utf8_lossy(name_bytes).into_owned());
            }
        }

        names.into_iter().collect()
    }

    fn pick_image_rgb_data(&self) -> Option<(Vec<u8>, u32, u32, PathBuf)> {
        let path = FileDialog::new()
            .add_filter("Image", &["png", "jpg", "jpeg", "bmp", "gif", "webp", "tif", "tiff"])
            .pick_file()?;

        let decoded = image::open(&path).ok()?;
        let rgb = decoded.to_rgb8();
        let (w, h) = rgb.dimensions();
        Some((rgb.into_raw(), w, h, path))
    }

    fn replace_image_resource_from_dialog(&mut self, resource_name: &str) {
        let Some((data, w, h, source_path)) = self.pick_image_rgb_data() else {
            self.emit(DocumentEvent::StatusChanged {
                message: "Replace image canceled".into(),
            });
            return;
        };

        let cmd = Box::new(ReplaceImageCommand::new(
            self.current_page,
            resource_name.to_owned(),
            data,
            w,
            h,
            None,
            None,
        ));
        self.run_tool_command(
            cmd,
            &format!(
                "Replaced image '{}' with {}",
                resource_name,
                source_path.display()
            ),
        );
    }

    fn apply_image_transform_action(&mut self, hit: ImageHit, action: &str) {

        self.pin_image_overlay_position();

        if action == "size" || action == "resize" {
            let width = (hit.matrix[0].powi(2) + hit.matrix[1].powi(2)).sqrt().max(1.0);
            let height = (hit.matrix[2].powi(2) + hit.matrix[3].powi(2)).sqrt().max(1.0);
            let default_size = format!("{:.0},{:.0}", width, height);
            let Some(size_raw) = Self::prompt_input(
                "Resize Image",
                "New width,height in points (e.g. 220,140)",
                &default_size,
            ) else {
                self.emit(DocumentEvent::StatusChanged {
                    message: "Resize image canceled".into(),
                });
                return;
            };
            let parts: Vec<&str> = size_raw.split(',').collect();
            if parts.len() != 2 {
                self.emit(DocumentEvent::StatusChanged {
                    message: "Invalid size format".into(),
                });
                return;
            }
            let Ok(new_w) = parts[0].trim().parse::<f32>() else {
                self.emit(DocumentEvent::StatusChanged {
                    message: "Invalid width value".into(),
                });
                return;
            };
            let Ok(new_h) = parts[1].trim().parse::<f32>() else {
                self.emit(DocumentEvent::StatusChanged {
                    message: "Invalid height value".into(),
                });
                return;
            };
            if new_w <= 1.0 || new_h <= 1.0 {
                self.emit(DocumentEvent::StatusChanged {
                    message: "Width/height must be > 1".into(),
                });
                return;
            }
            if let Some(next) = Self::resize_matrix_keep_orientation(hit.matrix, new_w, new_h) {
                let cmd = Box::new(UpdateImageTransformCommand::new(
                    self.current_page,
                    hit.stream_id,
                    hit.cm_op_index,
                    hit.matrix,
                    next,
                ));
                self.run_tool_command(cmd, "Resized image");
            } else {
                self.emit(DocumentEvent::StatusChanged {
                    message: "Image transform is not axis-aligned; resize unsupported".into(),
                });
            }
            return;
        }

        let rotated = match action {
            "r90" | "rotate90" => Self::rotate_matrix_keep_center(hit.matrix, 90),
            "r180" | "rotate180" => Self::rotate_matrix_keep_center(hit.matrix, 180),
            "r270" | "rotate270" => Self::rotate_matrix_keep_center(hit.matrix, 270),
            "flip" | "fliph" => Some(Self::flip_matrix_horizontal_keep_center(hit.matrix)),
            _ => None,
        };

        if let Some(next) = rotated {
            let cmd = Box::new(UpdateImageTransformCommand::new(
                self.current_page,
                hit.stream_id,
                hit.cm_op_index,
                hit.matrix,
                next,
            ));
            self.run_tool_command(cmd, "Rotated image");
        } else {
            self.emit(DocumentEvent::StatusChanged {
                message: "Unknown image action".into(),
            });
        }
    }

    fn find_last_image_hit_by_resource_name(&self, resource_name: &str) -> Option<ImageHit> {
        let doc = self.document.as_ref()?;
        let page = doc.get_page(self.current_page).ok()?;
        let inner = doc.inner();
        let stream_ids = Self::page_content_stream_ids(inner, page.object_id);
        if stream_ids.is_empty() {
            return None;
        }

        let mut hits = Vec::new();
        for stream_id in stream_ids {
            let Ok(stream_obj) = inner.get_object(stream_id) else {
                continue;
            };
            let Ok(stream) = stream_obj.as_stream() else {
                continue;
            };
            let Some(content) = Self::decode_stream_content(stream) else {
                continue;
            };

            let mut ctm = [1.0f32, 0.0, 0.0, 1.0, 0.0, 0.0];
            let mut stack: Vec<[f32; 6]> = Vec::new();
            let mut last_cm_index: Option<usize> = None;
            let mut cm_index_stack: Vec<Option<usize>> = Vec::new();

            for (op_index, op) in content.operations.iter().enumerate() {
                match op.operator.as_str() {
                    "q" => {
                        stack.push(ctm);
                        cm_index_stack.push(last_cm_index);
                    }
                    "Q" => {
                        if let Some(prev) = stack.pop() {
                            ctm = prev;
                        }
                        if let Some(prev_idx) = cm_index_stack.pop() {
                            last_cm_index = prev_idx;
                        }
                    }
                    "cm" => {
                        if let Some(m) = Self::matrix_from_cm_operands(&op.operands) {
                            ctm = Self::concat_matrix(ctm, m);
                            last_cm_index = Some(op_index);
                        }
                    }
                    "Do" => {
                        let Some(Object::Name(name_bytes)) = op.operands.first() else {
                            continue;
                        };
                        let name = String::from_utf8_lossy(name_bytes).into_owned();
                        if name != resource_name {
                            continue;
                        }

                        let cm_op_index = last_cm_index.unwrap_or(op_index);
                        let p0 = Self::transform_point(ctm, 0.0, 0.0);
                        let p1 = Self::transform_point(ctm, 1.0, 0.0);
                        let p2 = Self::transform_point(ctm, 0.0, 1.0);
                        let p3 = Self::transform_point(ctm, 1.0, 1.0);
                        let xs = [p0.0, p1.0, p2.0, p3.0];
                        let ys = [p0.1, p1.1, p2.1, p3.1];
                        let mut x_min = xs[0];
                        let mut x_max = xs[0];
                        let mut y_min = ys[0];
                        let mut y_max = ys[0];
                        for x in xs {
                            x_min = x_min.min(x);
                            x_max = x_max.max(x);
                        }
                        for y in ys {
                            y_min = y_min.min(y);
                            y_max = y_max.max(y);
                        }

                        hits.push(ImageHit {
                            resource_name: name,
                            stream_id,
                            do_op_index: op_index,
                            cm_op_index,
                            matrix: ctm,
                            x_min,
                            x_max,
                            y_min,
                            y_max,
                        });
                    }
                    _ => {}
                }
            }
        }

        hits.pop()
    }

    fn find_image_hit_by_target(
        &self,
        target_stream_id: lopdf::ObjectId,
        target_do_op_index: usize,
    ) -> Option<ImageHit> {
        let doc = self.document.as_ref()?;
        let page = doc.get_page(self.current_page).ok()?;
        let inner = doc.inner();
        let stream_ids = Self::page_content_stream_ids(inner, page.object_id);
        if stream_ids.is_empty() {
            return None;
        }

        for stream_id in stream_ids {
            if stream_id != target_stream_id {
                continue;
            }

            let Ok(stream_obj) = inner.get_object(stream_id) else {
                continue;
            };
            let Ok(stream) = stream_obj.as_stream() else {
                continue;
            };
            let Some(content) = Self::decode_stream_content(stream) else {
                continue;
            };

            let mut ctm = [1.0f32, 0.0, 0.0, 1.0, 0.0, 0.0];
            let mut stack: Vec<[f32; 6]> = Vec::new();
            let mut last_cm_index: Option<usize> = None;
            let mut cm_index_stack: Vec<Option<usize>> = Vec::new();

            for (op_index, op) in content.operations.iter().enumerate() {
                match op.operator.as_str() {
                    "q" => {
                        stack.push(ctm);
                        cm_index_stack.push(last_cm_index);
                    }
                    "Q" => {
                        if let Some(prev) = stack.pop() {
                            ctm = prev;
                        }
                        if let Some(prev_idx) = cm_index_stack.pop() {
                            last_cm_index = prev_idx;
                        }
                    }
                    "cm" => {
                        if let Some(m) = Self::matrix_from_cm_operands(&op.operands) {
                            ctm = Self::concat_matrix(ctm, m);
                            last_cm_index = Some(op_index);
                        }
                    }
                    "Do" => {
                        if op_index != target_do_op_index {
                            continue;
                        }

                        let cm_op_index = last_cm_index.unwrap_or(op_index);

                        let Some(Object::Name(name_bytes)) = op.operands.first() else {
                            continue;
                        };
                        let name = String::from_utf8_lossy(name_bytes).into_owned();

                        let p0 = Self::transform_point(ctm, 0.0, 0.0);
                        let p1 = Self::transform_point(ctm, 1.0, 0.0);
                        let p2 = Self::transform_point(ctm, 0.0, 1.0);
                        let p3 = Self::transform_point(ctm, 1.0, 1.0);
                        let xs = [p0.0, p1.0, p2.0, p3.0];
                        let ys = [p0.1, p1.1, p2.1, p3.1];
                        let mut x_min = xs[0];
                        let mut x_max = xs[0];
                        let mut y_min = ys[0];
                        let mut y_max = ys[0];
                        for x in xs {
                            x_min = x_min.min(x);
                            x_max = x_max.max(x);
                        }
                        for y in ys {
                            y_min = y_min.min(y);
                            y_max = y_max.max(y);
                        }

                        return Some(ImageHit {
                            resource_name: name,
                            stream_id,
                            do_op_index: op_index,
                            cm_op_index,
                            matrix: ctm,
                            x_min,
                            x_max,
                            y_min,
                            y_max,
                        });
                    }
                    _ => {}
                }
            }
        }

        None
    }

    fn decode_pdf_text_bytes(bytes: &[u8]) -> String {
        let obj = Object::String(bytes.to_vec(), lopdf::StringFormat::Literal);
        lopdf::decode_text_string(&obj).unwrap_or_else(|_| String::from_utf8_lossy(bytes).into_owned())
    }

    fn decode_stream_content(stream: &lopdf::Stream) -> Option<Content> {
        let bytes = stream
            .decompressed_content()
            .unwrap_or_else(|_| stream.content.clone());
        Content::decode(&bytes).ok()
    }

    fn matrix_approx_eq(a: [f32; 6], b: [f32; 6], eps: f32) -> bool {
        (0..6).all(|i| (a[i] - b[i]).abs() <= eps)
    }

    fn rotate_matrix_keep_center(m: [f32; 6], degrees: i32) -> Option<[f32; 6]> {
        let radians = match degrees {
            90 => std::f32::consts::FRAC_PI_2,
            180 => std::f32::consts::PI,
            270 => 3.0 * std::f32::consts::FRAC_PI_2,
            _ => return None,
        };
        Some(Self::rotate_matrix_by_radians_keep_center(m, radians))
    }

    fn rotate_matrix_by_radians_keep_center(m: [f32; 6], radians: f32) -> [f32; 6] {
        let cos_t = radians.cos();
        let sin_t = radians.sin();
        let u = (m[0], m[1]);
        let v = (m[2], m[3]);
        let up = (u.0 * cos_t - u.1 * sin_t, u.0 * sin_t + u.1 * cos_t);
        let vp = (v.0 * cos_t - v.1 * sin_t, v.0 * sin_t + v.1 * cos_t);
        let cx = m[4] + (u.0 + v.0) * 0.5;
        let cy = m[5] + (u.1 + v.1) * 0.5;
        let ne = cx - (up.0 + vp.0) * 0.5;
        let nf = cy - (up.1 + vp.1) * 0.5;
        [up.0, up.1, vp.0, vp.1, ne, nf]
    }

    fn resize_matrix_keep_orientation(m: [f32; 6], new_w: f32, new_h: f32) -> Option<[f32; 6]> {
        let u = (m[0], m[1]);
        let v = (m[2], m[3]);
        let ow = (u.0 * u.0 + u.1 * u.1).sqrt();
        let oh = (v.0 * v.0 + v.1 * v.1).sqrt();
        if ow < 1e-4 || oh < 1e-4 {
            return None;
        }

        let su = new_w / ow;
        let sv = new_h / oh;
        let up = (u.0 * su, u.1 * su);
        let vp = (v.0 * sv, v.1 * sv);
        let cx = m[4] + (u.0 + v.0) * 0.5;
        let cy = m[5] + (u.1 + v.1) * 0.5;
        let ne = cx - (up.0 + vp.0) * 0.5;
        let nf = cy - (up.1 + vp.1) * 0.5;
        Some([up.0, up.1, vp.0, vp.1, ne, nf])
    }

    fn flip_matrix_horizontal_keep_center(m: [f32; 6]) -> [f32; 6] {
        let u = (m[0], m[1]);
        let v = (m[2], m[3]);
        let up = (-u.0, -u.1);
        let vp = v;
        let cx = m[4] + (u.0 + v.0) * 0.5;
        let cy = m[5] + (u.1 + v.1) * 0.5;
        let ne = cx - (up.0 + vp.0) * 0.5;
        let nf = cy - (up.1 + vp.1) * 0.5;
        [up.0, up.1, vp.0, vp.1, ne, nf]
    }

    fn apply_image_size_from_panel(&mut self, w_text: String, h_text: String, keep_aspect: bool) {
        if !self.ensure_document_open() {
            return;
        }

        self.pin_image_overlay_position();

        let Some(hit) = self
            .selected_image_target
            .and_then(|(sid, do_idx)| self.find_image_hit_by_target(sid, do_idx))
            .or_else(|| {
                self.selected_image_resource_name
                    .as_ref()
                    .and_then(|name| self.find_last_image_hit_by_resource_name(name))
            })
        else {
            self.emit(DocumentEvent::StatusChanged {
                message: "Select an image first".into(),
            });
            return;
        };

        let parsed_w = w_text.trim().parse::<f32>().ok();
        let parsed_h = h_text.trim().parse::<f32>().ok();
        let (mut new_w, mut new_h) = match (parsed_w, parsed_h) {
            (Some(w), Some(h)) if w > 1.0 && h > 1.0 => (w, h),
            _ => {
                self.emit(DocumentEvent::StatusChanged {
                    message: "Invalid width/height".into(),
                });
                return;
            }
        };

        let cur_w = (hit.matrix[0].powi(2) + hit.matrix[1].powi(2)).sqrt().max(1.0);
        let cur_h = (hit.matrix[2].powi(2) + hit.matrix[3].powi(2)).sqrt().max(1.0);
        if keep_aspect {
            let aspect = cur_w / cur_h.max(1.0);
            let dw = (new_w - cur_w).abs() / cur_w.max(1.0);
            let dh = (new_h - cur_h).abs() / cur_h.max(1.0);
            if dw >= dh {
                new_h = (new_w / aspect).max(1.0);
            } else {
                new_w = (new_h * aspect).max(1.0);
            }
        }

        let Some(next) = Self::resize_matrix_keep_orientation(hit.matrix, new_w, new_h) else {
            self.emit(DocumentEvent::StatusChanged {
                message: "Resize is not supported for this transform".into(),
            });
            return;
        };

        let cmd = Box::new(UpdateImageTransformCommand::new(
            self.current_page,
            hit.stream_id,
            hit.cm_op_index,
            hit.matrix,
            next,
        ));
        self.run_tool_command(cmd, "Resized image");

        let refreshed = self
            .selected_image_target
            .and_then(|(sid, do_idx)| self.find_image_hit_by_target(sid, do_idx))
            .or_else(|| {
                self.selected_image_resource_name
                    .as_ref()
                    .and_then(|name| self.find_last_image_hit_by_resource_name(name))
            });
        if let Some(ref h) = refreshed {
            self.selected_image_target = Some((h.stream_id, h.do_op_index));
            self.selected_image_hit = Some(h.clone());
            self.update_image_overlay_display(Some(h));
            self.update_image_selection_display(Some(h));
        }
    }

    fn find_text_hit_at_point(&self, px: f32, py: f32) -> Option<TextHit> {
        let hits = self.collect_text_hits_on_page();
        hits.into_iter()
            .filter(|h| {
                // Keep this tolerant because text bounds are estimated from
                // content operators rather than full glyph shaping.
                let tol = 20.0;
                px >= h.x - tol
                    && px <= h.x + h.width + tol
                    && py >= h.y - tol
                    && py <= h.y + h.height + tol
            })
            .min_by(|a, b| {
            let distance_to_rect = |h: &TextHit| {
                let dx = if px < h.x {
                    h.x - px
                } else if px > h.x + h.width {
                    px - (h.x + h.width)
                } else {
                    0.0
                };
                let dy = if py < h.y {
                    h.y - py
                } else if py > h.y + h.height {
                    py - (h.y + h.height)
                } else {
                    0.0
                };
                (dx * dx + dy * dy).sqrt()
            };
            let text_score = |h: &TextHit| {
                let t = h.text.trim();
                let alnum = t.chars().filter(|c| c.is_alphanumeric()).count() as i32;
                let len = t.chars().count() as i32;
                alnum * 4 + len
            };

            let da = distance_to_rect(a);
            let db = distance_to_rect(b);
            da.partial_cmp(&db)
                .unwrap_or(std::cmp::Ordering::Equal)
                .then_with(|| text_score(b).cmp(&text_score(a)))
                .then_with(|| {
                    let aa = (a.width * a.height) as i32;
                    let ab = (b.width * b.height) as i32;
                    ab.cmp(&aa)
                })
        })
    }

    fn find_nearest_text_hit_at_point(&self, px: f32, py: f32) -> Option<TextHit> {
        let max_distance = 48.0f32;
        self.collect_text_hits_on_page()
            .into_iter()
            .map(|h| {
                let dx = if px < h.x {
                    h.x - px
                } else if px > h.x + h.width {
                    px - (h.x + h.width)
                } else {
                    0.0
                };
                let dy = if py < h.y {
                    h.y - py
                } else if py > h.y + h.height {
                    py - (h.y + h.height)
                } else {
                    0.0
                };
                let dist = (dx * dx + dy * dy).sqrt();
                (h, dist)
            })
            .filter(|(_, dist)| *dist <= max_distance)
            .filter(|(h, dist)| {
                let trimmed = h.text.trim();
                let total = trimmed.chars().count();
                let alnum = trimmed.chars().filter(|c| c.is_alphanumeric()).count();
                let area = h.width * h.height;

                // Avoid snapping to tiny symbol-like fragments unless click is very close.
                if alnum == 0 && total <= 2 {
                    return *dist <= 4.0;
                }

                // Tiny low-information hits are noisy in many PDFs.
                if area < 120.0 && alnum <= 1 {
                    return *dist <= 10.0;
                }

                true
            })
            .min_by(|a, b| {
                let ta = a.0.text.trim();
                let tb = b.0.text.trim();
                let qa = ta.chars().filter(|c| c.is_alphanumeric()).count() as i32 * 4
                    + ta.chars().count() as i32;
                let qb = tb.chars().filter(|c| c.is_alphanumeric()).count() as i32 * 4
                    + tb.chars().count() as i32;

                a.1.partial_cmp(&b.1)
                    .unwrap_or(std::cmp::Ordering::Equal)
                    .then_with(|| qb.cmp(&qa))
                    .then_with(|| {
                        let aa = (a.0.width * a.0.height) as i32;
                        let ab = (b.0.width * b.0.height) as i32;
                        ab.cmp(&aa)
                    })
            })
            .map(|(h, _)| h)
    }

    fn find_text_hit_by_target(
        &self,
        target_stream_id: lopdf::ObjectId,
        target_op_index: usize,
    ) -> Option<TextHit> {
        self.collect_text_hits_on_page().into_iter().find(|h| {
            h.editable && h.stream_id == target_stream_id && h.op_index == target_op_index
        })
    }

    fn find_nearest_editable_text_hit(
        &self,
        px: f32,
        py: f32,
        expected_text: Option<&str>,
    ) -> Option<TextHit> {
        let expected = expected_text.map(str::trim).filter(|s| !s.is_empty());
        self.collect_text_hits_on_page()
            .into_iter()
            .filter(|h| h.editable)
            .map(|h| {
                let dx = if px < h.x {
                    h.x - px
                } else if px > h.x + h.width {
                    px - (h.x + h.width)
                } else {
                    0.0
                };
                let dy = if py < h.y {
                    h.y - py
                } else if py > h.y + h.height {
                    py - (h.y + h.height)
                } else {
                    0.0
                };
                let dist = (dx * dx + dy * dy).sqrt();

                let mut text_score = 0i32;
                if let Some(exp) = expected {
                    let ht = h.text.trim();
                    if ht == exp {
                        text_score += 100;
                    } else if ht.contains(exp) || exp.contains(ht) {
                        text_score += 40;
                    }
                }

                let quality = h
                    .text
                    .trim()
                    .chars()
                    .filter(|c| c.is_alphanumeric())
                    .count() as i32;

                (h, dist, text_score, quality)
            })
            .min_by(|a, b| {
                a.1.partial_cmp(&b.1)
                    .unwrap_or(std::cmp::Ordering::Equal)
                    .then_with(|| b.2.cmp(&a.2))
                    .then_with(|| b.3.cmp(&a.3))
            })
            .map(|(h, _, _, _)| h)
    }

    fn compose_text_run_around_hit(&self, hit: &TextHit) -> String {
        let all = self.collect_text_hits_on_page();
        if all.is_empty() {
            return hit.text.clone();
        }

        let target_cy = hit.y + hit.height * 0.5;
        let target_cx = hit.x + hit.width * 0.5;
        let line_tol = (hit.height * 0.85).max(10.0);

        let mut line_hits: Vec<TextHit> = all
            .into_iter()
            .filter(|h| {
                let cy = h.y + h.height * 0.5;
                (cy - target_cy).abs() <= line_tol
            })
            .collect();

        if line_hits.is_empty() {
            return hit.text.clone();
        }

        line_hits.sort_by(|a, b| a.x.partial_cmp(&b.x).unwrap_or(std::cmp::Ordering::Equal));

        let Some(anchor_idx) = line_hits
            .iter()
            .enumerate()
            .min_by(|(_, a), (_, b)| {
                let acx = a.x + a.width * 0.5;
                let bcx = b.x + b.width * 0.5;
                let ad = (acx - target_cx).abs();
                let bd = (bcx - target_cx).abs();
                ad.partial_cmp(&bd).unwrap_or(std::cmp::Ordering::Equal)
            })
            .map(|(i, _)| i)
        else {
            return hit.text.clone();
        };

        let gap_threshold = (hit.height * 1.3).max(18.0);
        let mut left = anchor_idx;
        while left > 0 {
            let prev = &line_hits[left - 1];
            let cur = &line_hits[left];
            let gap = cur.x - (prev.x + prev.width);
            if gap > gap_threshold {
                break;
            }
            left -= 1;
        }

        let mut right = anchor_idx;
        while right + 1 < line_hits.len() {
            let cur = &line_hits[right];
            let next = &line_hits[right + 1];
            let gap = next.x - (cur.x + cur.width);
            if gap > gap_threshold {
                break;
            }
            right += 1;
        }

        let mut out = String::new();
        for idx in left..=right {
            let chunk = line_hits[idx].text.trim();
            if chunk.is_empty() {
                continue;
            }
            if !out.is_empty() {
                let prev = &line_hits[idx - 1];
                let cur = &line_hits[idx];
                let gap = cur.x - (prev.x + prev.width);
                if gap > (hit.height * 0.35).max(4.0) {
                    out.push(' ');
                }
            }
            out.push_str(chunk);
        }

        if out.trim().is_empty() {
            hit.text.clone()
        } else {
            out
        }
    }

    fn collect_text_hits_on_page(&self) -> Vec<TextHit> {
        let Some(doc) = self.document.as_ref() else {
            return Vec::new();
        };
        let Ok(page) = doc.get_page(self.current_page) else {
            return Vec::new();
        };
        let inner = doc.inner();
        let stream_ids = Self::page_content_stream_ids(inner, page.object_id);
        if stream_ids.is_empty() {
            return Vec::new();
        }

        let mut hits = Vec::new();
        for stream_id in stream_ids {
            let Ok(stream_obj) = inner.get_object(stream_id) else {
                continue;
            };
            let Ok(stream) = stream_obj.as_stream() else {
                continue;
            };
            let Some(content) = Self::decode_stream_content(stream) else {
                continue;
            };

            let mut ctm = [1.0f32, 0.0, 0.0, 1.0, 0.0, 0.0];
            let mut ctm_stack: Vec<[f32; 6]> = Vec::new();
            let mut in_text = false;
            let mut font_size = 12.0f32;
            let mut leading = 0.0f32;
            let mut text_matrix = [1.0f32, 0.0, 0.0, 1.0, 0.0, 0.0];
            let mut line_matrix = [1.0f32, 0.0, 0.0, 1.0, 0.0, 0.0];

            for (op_index, op) in content.operations.iter().enumerate() {
                match op.operator.as_str() {
                    "q" => ctm_stack.push(ctm),
                    "Q" => {
                        if let Some(prev) = ctm_stack.pop() {
                            ctm = prev;
                        }
                    }
                    "cm" => {
                        if let Some(m) = Self::matrix_from_cm_operands(&op.operands) {
                            ctm = Self::concat_matrix(ctm, m);
                        }
                    }
                    "BT" => {
                        in_text = true;
                        text_matrix = [1.0, 0.0, 0.0, 1.0, 0.0, 0.0];
                        line_matrix = [1.0, 0.0, 0.0, 1.0, 0.0, 0.0];
                    }
                    "ET" => in_text = false,
                    "Tf" if in_text => {
                        if let Some(size) = op.operands.get(1).and_then(Self::operand_to_f32) {
                            font_size = size.max(1.0);
                        }
                    }
                    "TL" if in_text => {
                        if let Some(v) = op.operands.first().and_then(Self::operand_to_f32) {
                            leading = v;
                        }
                    }
                    "Td" if in_text => {
                        if let (Some(dx), Some(dy)) = (
                            op.operands.get(0).and_then(Self::operand_to_f32),
                            op.operands.get(1).and_then(Self::operand_to_f32),
                        ) {
                            let t = [1.0, 0.0, 0.0, 1.0, dx, dy];
                            line_matrix = Self::concat_matrix(line_matrix, t);
                            text_matrix = line_matrix;
                        }
                    }
                    "T*" if in_text => {
                        let dy = if leading == 0.0 { -font_size * 1.2 } else { -leading };
                        let t = [1.0, 0.0, 0.0, 1.0, 0.0, dy];
                        line_matrix = Self::concat_matrix(line_matrix, t);
                        text_matrix = line_matrix;
                    }
                    "Tm" if in_text => {
                        if let Some(tm) = Self::matrix_from_cm_operands(&op.operands) {
                            text_matrix = tm;
                            line_matrix = tm;
                        }
                    }
                    "Tj" if in_text => {
                        let Some(Object::String(bytes, _)) = op.operands.first() else {
                            continue;
                        };
                        let text = Self::decode_pdf_text_bytes(bytes);
                        if text.trim().is_empty() {
                            continue;
                        }

                        let text_width = (text.chars().count() as f32 * font_size * 0.55).max(10.0);
                        let y_bottom = -font_size * 0.35;
                        let y_top = font_size * 0.95;
                        let trm = Self::concat_matrix(ctm, text_matrix);
                        let p0 = Self::transform_point(trm, 0.0, y_bottom);
                        let p1 = Self::transform_point(trm, text_width, y_bottom);
                        let p2 = Self::transform_point(trm, 0.0, y_top);
                        let p3 = Self::transform_point(trm, text_width, y_top);
                        let xs = [p0.0, p1.0, p2.0, p3.0];
                        let ys = [p0.1, p1.1, p2.1, p3.1];
                        let mut x_min = xs[0];
                        let mut x_max = xs[0];
                        let mut y_min = ys[0];
                        let mut y_max = ys[0];
                        for x in xs {
                            x_min = x_min.min(x);
                            x_max = x_max.max(x);
                        }
                        for y in ys {
                            y_min = y_min.min(y);
                            y_max = y_max.max(y);
                        }

                        hits.push(TextHit {
                            text,
                            stream_id,
                            op_index,
                            editable: true,
                            x: x_min,
                            y: y_min,
                            width: (x_max - x_min).max(8.0),
                            height: (y_max - y_min).max(8.0),
                        });

                        // Advance text matrix for subsequent Tj operations.
                        text_matrix = Self::concat_matrix(
                            text_matrix,
                            [1.0, 0.0, 0.0, 1.0, text_width, 0.0],
                        );
                    }
                    "TJ" if in_text => {
                        let Some(Object::Array(parts)) = op.operands.first() else {
                            continue;
                        };

                        let mut text = String::new();
                        let mut spacing_adjust = 0.0f32;
                        for part in parts {
                            match part {
                                Object::String(bytes, _) => {
                                    text.push_str(&Self::decode_pdf_text_bytes(bytes));
                                }
                                Object::Integer(v) => {
                                    // In TJ, positive values tighten spacing; negative expand.
                                    spacing_adjust += -(*v as f32) * font_size / 1000.0;
                                }
                                Object::Real(v) => {
                                    spacing_adjust += -(*v) * font_size / 1000.0;
                                }
                                _ => {}
                            }
                        }
                        if text.trim().is_empty() {
                            continue;
                        }

                        let text_width =
                            (text.chars().count() as f32 * font_size * 0.55 + spacing_adjust)
                                .max(10.0);
                        let y_bottom = -font_size * 0.35;
                        let y_top = font_size * 0.95;
                        let trm = Self::concat_matrix(ctm, text_matrix);
                        let p0 = Self::transform_point(trm, 0.0, y_bottom);
                        let p1 = Self::transform_point(trm, text_width, y_bottom);
                        let p2 = Self::transform_point(trm, 0.0, y_top);
                        let p3 = Self::transform_point(trm, text_width, y_top);
                        let xs = [p0.0, p1.0, p2.0, p3.0];
                        let ys = [p0.1, p1.1, p2.1, p3.1];
                        let mut x_min = xs[0];
                        let mut x_max = xs[0];
                        let mut y_min = ys[0];
                        let mut y_max = ys[0];
                        for x in xs {
                            x_min = x_min.min(x);
                            x_max = x_max.max(x);
                        }
                        for y in ys {
                            y_min = y_min.min(y);
                            y_max = y_max.max(y);
                        }

                        hits.push(TextHit {
                            text,
                            stream_id,
                            op_index,
                            editable: false,
                            x: x_min,
                            y: y_min,
                            width: (x_max - x_min).max(8.0),
                            height: (y_max - y_min).max(8.0),
                        });

                        text_matrix = Self::concat_matrix(
                            text_matrix,
                            [1.0, 0.0, 0.0, 1.0, text_width, 0.0],
                        );
                    }
                    "'" if in_text => {
                        // Move to next text line and show text.
                        let dy = if leading == 0.0 { -font_size * 1.2 } else { -leading };
                        let t = [1.0, 0.0, 0.0, 1.0, 0.0, dy];
                        line_matrix = Self::concat_matrix(line_matrix, t);
                        text_matrix = line_matrix;

                        let Some(Object::String(bytes, _)) = op.operands.first() else {
                            continue;
                        };
                        let text = Self::decode_pdf_text_bytes(bytes);
                        if text.trim().is_empty() {
                            continue;
                        }

                        let text_width = (text.chars().count() as f32 * font_size * 0.55).max(10.0);
                        let y_bottom = -font_size * 0.35;
                        let y_top = font_size * 0.95;
                        let trm = Self::concat_matrix(ctm, text_matrix);
                        let p0 = Self::transform_point(trm, 0.0, y_bottom);
                        let p1 = Self::transform_point(trm, text_width, y_bottom);
                        let p2 = Self::transform_point(trm, 0.0, y_top);
                        let p3 = Self::transform_point(trm, text_width, y_top);
                        let xs = [p0.0, p1.0, p2.0, p3.0];
                        let ys = [p0.1, p1.1, p2.1, p3.1];
                        let mut x_min = xs[0];
                        let mut x_max = xs[0];
                        let mut y_min = ys[0];
                        let mut y_max = ys[0];
                        for x in xs {
                            x_min = x_min.min(x);
                            x_max = x_max.max(x);
                        }
                        for y in ys {
                            y_min = y_min.min(y);
                            y_max = y_max.max(y);
                        }

                        hits.push(TextHit {
                            text,
                            stream_id,
                            op_index,
                            editable: false,
                            x: x_min,
                            y: y_min,
                            width: (x_max - x_min).max(8.0),
                            height: (y_max - y_min).max(8.0),
                        });

                        text_matrix = Self::concat_matrix(
                            text_matrix,
                            [1.0, 0.0, 0.0, 1.0, text_width, 0.0],
                        );
                    }
                    "\"" if in_text => {
                        // Set spacing and move to next line, then show text.
                        let dy = if leading == 0.0 { -font_size * 1.2 } else { -leading };
                        let t = [1.0, 0.0, 0.0, 1.0, 0.0, dy];
                        line_matrix = Self::concat_matrix(line_matrix, t);
                        text_matrix = line_matrix;

                        let Some(Object::String(bytes, _)) = op.operands.get(2) else {
                            continue;
                        };
                        let text = Self::decode_pdf_text_bytes(bytes);
                        if text.trim().is_empty() {
                            continue;
                        }

                        let text_width = (text.chars().count() as f32 * font_size * 0.55).max(10.0);
                        let y_bottom = -font_size * 0.35;
                        let y_top = font_size * 0.95;
                        let trm = Self::concat_matrix(ctm, text_matrix);
                        let p0 = Self::transform_point(trm, 0.0, y_bottom);
                        let p1 = Self::transform_point(trm, text_width, y_bottom);
                        let p2 = Self::transform_point(trm, 0.0, y_top);
                        let p3 = Self::transform_point(trm, text_width, y_top);
                        let xs = [p0.0, p1.0, p2.0, p3.0];
                        let ys = [p0.1, p1.1, p2.1, p3.1];
                        let mut x_min = xs[0];
                        let mut x_max = xs[0];
                        let mut y_min = ys[0];
                        let mut y_max = ys[0];
                        for x in xs {
                            x_min = x_min.min(x);
                            x_max = x_max.max(x);
                        }
                        for y in ys {
                            y_min = y_min.min(y);
                            y_max = y_max.max(y);
                        }

                        hits.push(TextHit {
                            text,
                            stream_id,
                            op_index,
                            editable: false,
                            x: x_min,
                            y: y_min,
                            width: (x_max - x_min).max(8.0),
                            height: (y_max - y_min).max(8.0),
                        });

                        text_matrix = Self::concat_matrix(
                            text_matrix,
                            [1.0, 0.0, 0.0, 1.0, text_width, 0.0],
                        );
                    }
                    _ => {}
                }
            }
        }

        hits
    }

    fn find_image_hit_at_point(&self, px: f32, py: f32) -> Option<ImageHit> {
        let doc = self.document.as_ref()?;
        let page = doc.get_page(self.current_page).ok()?;
        let inner = doc.inner();
        let stream_ids = Self::page_content_stream_ids(inner, page.object_id);
        if stream_ids.is_empty() {
            return None;
        }

        let known_names: BTreeSet<String> = self.page_image_resource_names().into_iter().collect();

        let mut candidates = Vec::new();
        for stream_id in stream_ids {
            let Ok(stream_obj) = inner.get_object(stream_id) else {
                continue;
            };
            let Ok(stream) = stream_obj.as_stream() else {
                continue;
            };
            let Some(content) = Self::decode_stream_content(stream) else {
                continue;
            };

            let mut ctm = [1.0f32, 0.0, 0.0, 1.0, 0.0, 0.0];
            let mut stack: Vec<[f32; 6]> = Vec::new();
            let mut last_cm_index: Option<usize> = None;
            let mut cm_index_stack: Vec<Option<usize>> = Vec::new();

            for (op_index, op) in content.operations.iter().enumerate() {
                match op.operator.as_str() {
                    "q" => {
                        stack.push(ctm);
                        cm_index_stack.push(last_cm_index);
                    }
                    "Q" => {
                        if let Some(prev) = stack.pop() {
                            ctm = prev;
                        }
                        if let Some(prev_idx) = cm_index_stack.pop() {
                            last_cm_index = prev_idx;
                        }
                    }
                    "cm" => {
                        if let Some(m) = Self::matrix_from_cm_operands(&op.operands) {
                            ctm = Self::concat_matrix(ctm, m);
                            last_cm_index = Some(op_index);
                        }
                    }
                    "Do" => {
                        let cm_op_index = last_cm_index.unwrap_or(op_index);
                        let Some(Object::Name(name_bytes)) = op.operands.first() else {
                            continue;
                        };
                        let name = String::from_utf8_lossy(name_bytes).into_owned();
                        // Prefer names resolved from page resources, but keep a
                        // permissive fallback for image-like Do names so newly
                        // inserted images remain clickable across PDFs.
                        if !known_names.is_empty()
                            && !known_names.contains(&name)
                            && !name.starts_with("Im")
                            && !name.starts_with("Image")
                            && !name.starts_with('I')
                        {
                            continue;
                        }

                        let p0 = Self::transform_point(ctm, 0.0, 0.0);
                        let p1 = Self::transform_point(ctm, 1.0, 0.0);
                        let p2 = Self::transform_point(ctm, 0.0, 1.0);
                        let p3 = Self::transform_point(ctm, 1.0, 1.0);
                        let xs = [p0.0, p1.0, p2.0, p3.0];
                        let ys = [p0.1, p1.1, p2.1, p3.1];
                        let mut x_min = xs[0];
                        let mut x_max = xs[0];
                        let mut y_min = ys[0];
                        let mut y_max = ys[0];
                        for x in xs {
                            x_min = x_min.min(x);
                            x_max = x_max.max(x);
                        }
                        for y in ys {
                            y_min = y_min.min(y);
                            y_max = y_max.max(y);
                        }
                        let tol = 12.0;
                        if px >= x_min - tol
                            && px <= x_max + tol
                            && py >= y_min - tol
                            && py <= y_max + tol
                        {
                            candidates.push(ImageHit {
                                resource_name: name,
                                stream_id,
                                do_op_index: op_index,
                                cm_op_index,
                                matrix: ctm,
                                x_min,
                                x_max,
                                y_min,
                                y_max,
                            });
                        }
                    }
                    _ => {}
                }
            }
        }

        candidates.into_iter().min_by(|a, b| {
            let acx = (a.x_min + a.x_max) * 0.5;
            let acy = (a.y_min + a.y_max) * 0.5;
            let bcx = (b.x_min + b.x_max) * 0.5;
            let bcy = (b.y_min + b.y_max) * 0.5;
            let da = (acx - px).powi(2) + (acy - py).powi(2);
            let db = (bcx - px).powi(2) + (bcy - py).powi(2);
            da.partial_cmp(&db).unwrap_or(std::cmp::Ordering::Equal)
        })
    }

    fn find_nearest_image_hit_at_point(&self, px: f32, py: f32) -> Option<ImageHit> {
        let doc = self.document.as_ref()?;
        let page = doc.get_page(self.current_page).ok()?;
        let inner = doc.inner();
        let stream_ids = Self::page_content_stream_ids(inner, page.object_id);
        if stream_ids.is_empty() {
            return None;
        }

        let known_names: BTreeSet<String> = self.page_image_resource_names().into_iter().collect();

        let mut candidates = Vec::new();
        for stream_id in stream_ids {
            let Ok(stream_obj) = inner.get_object(stream_id) else {
                continue;
            };
            let Ok(stream) = stream_obj.as_stream() else {
                continue;
            };
            let Some(content) = Self::decode_stream_content(stream) else {
                continue;
            };

            let mut ctm = [1.0f32, 0.0, 0.0, 1.0, 0.0, 0.0];
            let mut stack: Vec<[f32; 6]> = Vec::new();
            let mut last_cm_index: Option<usize> = None;
            let mut cm_index_stack: Vec<Option<usize>> = Vec::new();

            for (op_index, op) in content.operations.iter().enumerate() {
                match op.operator.as_str() {
                    "q" => {
                        stack.push(ctm);
                        cm_index_stack.push(last_cm_index);
                    }
                    "Q" => {
                        if let Some(prev) = stack.pop() {
                            ctm = prev;
                        }
                        if let Some(prev_idx) = cm_index_stack.pop() {
                            last_cm_index = prev_idx;
                        }
                    }
                    "cm" => {
                        if let Some(m) = Self::matrix_from_cm_operands(&op.operands) {
                            ctm = Self::concat_matrix(ctm, m);
                            last_cm_index = Some(op_index);
                        }
                    }
                    "Do" => {
                        let cm_op_index = last_cm_index.unwrap_or(op_index);
                        let Some(Object::Name(name_bytes)) = op.operands.first() else {
                            continue;
                        };
                        let name = String::from_utf8_lossy(name_bytes).into_owned();
                        if !known_names.is_empty()
                            && !known_names.contains(&name)
                            && !name.starts_with("Im")
                            && !name.starts_with("Image")
                            && !name.starts_with('I')
                        {
                            continue;
                        }

                        let p0 = Self::transform_point(ctm, 0.0, 0.0);
                        let p1 = Self::transform_point(ctm, 1.0, 0.0);
                        let p2 = Self::transform_point(ctm, 0.0, 1.0);
                        let p3 = Self::transform_point(ctm, 1.0, 1.0);
                        let xs = [p0.0, p1.0, p2.0, p3.0];
                        let ys = [p0.1, p1.1, p2.1, p3.1];
                        let mut x_min = xs[0];
                        let mut x_max = xs[0];
                        let mut y_min = ys[0];
                        let mut y_max = ys[0];
                        for x in xs {
                            x_min = x_min.min(x);
                            x_max = x_max.max(x);
                        }
                        for y in ys {
                            y_min = y_min.min(y);
                            y_max = y_max.max(y);
                        }

                        candidates.push(ImageHit {
                            resource_name: name,
                            stream_id,
                            do_op_index: op_index,
                            cm_op_index,
                            matrix: ctm,
                            x_min,
                            x_max,
                            y_min,
                            y_max,
                        });
                    }
                    _ => {}
                }
            }
        }

        let max_distance = 36.0f32;
        candidates
            .into_iter()
            .map(|h| {
                let dx = if px < h.x_min {
                    h.x_min - px
                } else if px > h.x_max {
                    px - h.x_max
                } else {
                    0.0
                };
                let dy = if py < h.y_min {
                    h.y_min - py
                } else if py > h.y_max {
                    py - h.y_max
                } else {
                    0.0
                };
                let dist = (dx * dx + dy * dy).sqrt();
                (h, dist)
            })
            .filter(|(_, dist)| *dist <= max_distance)
            .min_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal))
            .map(|(h, _)| h)
    }

    fn page_content_stream_ids(
        inner: &lopdf::Document,
        page_id: lopdf::ObjectId,
    ) -> Vec<lopdf::ObjectId> {
        let contents = inner
            .get_object(page_id)
            .ok()
            .and_then(|o| o.as_dict().ok())
            .and_then(|d| d.get(b"Contents").ok());

        match contents {
            Some(Object::Reference(id)) => vec![*id],
            Some(Object::Array(arr)) => arr.iter().filter_map(|o| o.as_reference().ok()).collect(),
            _ => Vec::new(),
        }
    }

    fn operand_to_f32(obj: &Object) -> Option<f32> {
        match obj {
            Object::Integer(v) => Some(*v as f32),
            Object::Real(v) => Some(*v),
            _ => None,
        }
    }

    fn matrix_from_cm_operands(operands: &[Object]) -> Option<[f32; 6]> {
        if operands.len() < 6 {
            return None;
        }
        Some([
            Self::operand_to_f32(&operands[0])?,
            Self::operand_to_f32(&operands[1])?,
            Self::operand_to_f32(&operands[2])?,
            Self::operand_to_f32(&operands[3])?,
            Self::operand_to_f32(&operands[4])?,
            Self::operand_to_f32(&operands[5])?,
        ])
    }

    fn concat_matrix(m1: [f32; 6], m2: [f32; 6]) -> [f32; 6] {
        [
            m1[0] * m2[0] + m1[2] * m2[1],
            m1[1] * m2[0] + m1[3] * m2[1],
            m1[0] * m2[2] + m1[2] * m2[3],
            m1[1] * m2[2] + m1[3] * m2[3],
            m1[0] * m2[4] + m1[2] * m2[5] + m1[4],
            m1[1] * m2[4] + m1[3] * m2[5] + m1[5],
        ]
    }

    fn transform_point(m: [f32; 6], x: f32, y: f32) -> (f32, f32) {
        (m[0] * x + m[2] * y + m[4], m[1] * x + m[3] * y + m[5])
    }

    fn tool_set_password(&mut self) {
        let Some(password) = Self::prompt_input("Set Password", "New password:", "") else {
            self.emit(DocumentEvent::StatusChanged {
                message: "Set password canceled".into(),
            });
            return;
        };
        let cmd = Box::new(SetPasswordCommand::new(password));
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
        let default_name = format!("Field{}", self.current_page + 1);
        let Some(raw_name) = Self::prompt_input(
            "Create Form Field",
            "Field name:",
            &default_name,
        ) else {
            self.emit(DocumentEvent::StatusChanged {
                message: "Create field canceled".into(),
            });
            return;
        };
        let field_name = if raw_name.trim().is_empty() {
            default_name
        } else {
            raw_name.trim().to_owned()
        };
        let cmd = Box::new(CreateFieldCommand::new(
            field_name,
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
                _ => {
                    let default_text = match &field.value {
                        FormFieldValue::Text(s) => s.clone(),
                        FormFieldValue::Selected(s) => s.clone(),
                        _ => String::new(),
                    };
                    let Some(input) = Self::prompt_input(
                        "Set Field Value",
                        &format!("New value for '{}':", field.full_name),
                        &default_text,
                    ) else {
                        self.emit(DocumentEvent::StatusChanged {
                            message: "Set field value canceled".into(),
                        });
                        return;
                    };
                    FormFieldValue::Text(input)
                }
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
            let path = FileDialog::new()
                .add_filter("JSON", &["json"])
                .set_file_name("form-data.json")
                .save_file();
            let Some(path) = path else {
                self.emit(DocumentEvent::StatusChanged {
                    message: "Export fields canceled".into(),
                });
                return;
            };
            match std::fs::write(&path, json) {
                Ok(()) => self.emit(DocumentEvent::StatusChanged {
                    message: format!("Exported form data to {}", path.display()),
                }),
                Err(e) => self.emit(DocumentEvent::Error {
                    message: format!("Failed to export form data: {e}"),
                }),
            }
        }
    }

    fn dispatch_menu_action(&mut self, menu_index: i32, action_index: i32) {
        if action_index <= 0 {
            return;
        }

        match (menu_index, action_index) {
            (0, 1) => self.open_document_dialog(),
            (0, 2) => self.save_document(),
            (0, 3) => self.save_document_as_dialog(),
            (0, 4) => self.close_document(),

            (1, 1) => self.undo(),
            (1, 2) => self.redo(),

            (2, 1) => self.set_zoom(self.zoom * 0.8),
            (2, 2) => self.set_zoom(self.zoom * 1.25),
            (2, 3) => self.set_zoom(1.0),

            (3, 1) => self.add_highlight_annotation(),
            (3, 2) => self.add_note_annotation(),

            (4, 1) => {
                if self.current_page > 0 {
                    self.current_page -= 1;
                    self.render_current_page();
                    self.emit(DocumentEvent::PageChanged {
                        index: self.current_page,
                    });
                }
            }
            (4, 2) => {
                if let Some(doc) = &self.document {
                    let count = doc.page_count();
                    if self.current_page + 1 < count {
                        self.current_page += 1;
                        self.render_current_page();
                        self.emit(DocumentEvent::PageChanged {
                            index: self.current_page,
                        });
                    }
                }
            }
            (4, 3) => self.delete_current_page(),
            (4, 4) => self.rotate_current_page(),

            (5, 1) => self.tool_insert_text(),
            (5, 2) => self.tool_font_substitution(),
            (5, 3) => self.tool_insert_image(),
            (5, 4) => self.tool_replace_image(),
            (5, 5) => self.tool_set_password(),
            (5, 6) => self.tool_redact_region(),
            (5, 7) => self.tool_apply_ocr(),
            (5, 8) => self.tool_reorder_pages(),
            (5, 9) => self.tool_merge_document(),
            (5, 10) => self.tool_create_field(),
            (5, 11) => self.tool_set_field_value(),
            (5, 12) => self.tool_detect_fields(),
            (5, 13) => self.tool_export_form_data(),

            (6, 1) => self.emit(DocumentEvent::StatusChanged {
                message: "Visit https://example.com/upgrade to purchase a commercial license.".into(),
            }),
            (6, 2) => self.activate_license_dialog(),

            _ => self.emit(DocumentEvent::StatusChanged {
                message: "Unknown menu action".into(),
            }),
        }
    }

    fn render_current_page(&mut self) {
        self.update_image_resource_menu_display();
        if let Some(doc) = &self.document {
            if doc.page_count() > 0 {
                let page = self.current_page.min(doc.page_count() - 1);
                if let Ok(page_obj) = doc.get_page(page) {
                    if let Some(win) = self.window.upgrade() {
                        win.set_page_width(page_obj.media_box.width as f32);
                        win.set_page_height(page_obj.media_box.height as f32);
                    }
                }
            }
        }
        let refreshed_selection = self
            .selected_image_target
            .and_then(|(sid, do_idx)| self.find_image_hit_by_target(sid, do_idx))
            .or_else(|| {
                self.selected_image_resource_name
                    .as_ref()
                    .and_then(|name| self.find_last_image_hit_by_resource_name(name))
            });
        if let Some(hit) = refreshed_selection {
            self.selected_image_resource_name = Some(hit.resource_name.clone());
            self.selected_image_target = Some((hit.stream_id, hit.do_op_index));
            self.selected_image_hit = Some(hit.clone());
            self.image_overlay_visible = true;
            self.update_image_overlay_display(Some(&hit));
            self.update_image_selection_display(Some(&hit));
        } else {
            self.selected_image_hit = None;
            self.update_image_overlay_display(None);
            self.update_image_selection_display(None);
        }
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
        if let Some(doc) = &mut self.document {
            let page_obj = match doc.get_page(page) {
                Ok(p) => p,
                Err(e) => {
                    self.emit(DocumentEvent::Error {
                        message: e.to_string(),
                    });
                    return;
                }
            };
            if let Some(win) = self.window.upgrade() {
                win.set_page_width(page_obj.media_box.width as f32);
                win.set_page_height(page_obj.media_box.height as f32);
            }
            let doc_bytes = match doc.to_bytes() {
                Ok(bytes) => bytes,
                Err(e) => {
                    self.emit(DocumentEvent::Error {
                        message: format!("Failed to prepare render bytes: {e}"),
                    });
                    return;
                }
            };
            let task = RenderTask {
                doc_id,
                doc_bytes,
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

    fn add_recent_document(&mut self, path: PathBuf) {
        let key = path.to_string_lossy().to_lowercase();
        self.recent_documents
            .retain(|p| p.to_string_lossy().to_lowercase() != key);
        self.recent_documents.insert(0, path);
        if self.recent_documents.len() > MAX_RECENT_DOCUMENTS {
            self.recent_documents.truncate(MAX_RECENT_DOCUMENTS);
        }
        self.save_recent_documents();
        self.update_recent_documents_display();
    }

    fn recent_documents_storage_path() -> Option<PathBuf> {
        #[cfg(windows)]
        {
            let appdata = std::env::var_os("APPDATA")?;
            return Some(PathBuf::from(appdata).join("free-pdf-editor").join("recent_documents.txt"));
        }

        #[cfg(not(windows))]
        {
            let home = std::env::var_os("HOME")?;
            return Some(PathBuf::from(home).join(".free-pdf-editor").join("recent_documents.txt"));
        }
    }

    fn load_recent_documents(&mut self) {
        let Some(path) = Self::recent_documents_storage_path() else {
            return;
        };
        let Ok(raw) = fs::read_to_string(path) else {
            return;
        };

        let mut loaded = Vec::new();
        for line in raw.lines() {
            let trimmed = line.trim();
            if trimmed.is_empty() {
                continue;
            }
            let p = PathBuf::from(trimmed);
            if !loaded.iter().any(|e: &PathBuf| e == &p) {
                loaded.push(p);
            }
            if loaded.len() >= MAX_RECENT_DOCUMENTS {
                break;
            }
        }
        self.recent_documents = loaded;
    }

    fn save_recent_documents(&self) {
        let Some(path) = Self::recent_documents_storage_path() else {
            return;
        };
        if let Some(parent) = path.parent() {
            if fs::create_dir_all(parent).is_err() {
                return;
            }
        }
        let body = self
            .recent_documents
            .iter()
            .take(MAX_RECENT_DOCUMENTS)
            .map(|p| p.to_string_lossy().to_string())
            .collect::<Vec<String>>()
            .join("\n");
        let _ = fs::write(path, body);
    }

    fn update_recent_documents_display(&self) {
        if let Some(win) = self.window.upgrade() {
            let full_entry = |idx: usize| {
                self.recent_documents
                    .get(idx)
                    .map(|p| p.display().to_string())
                    .unwrap_or_default()
                    .into()
            };
            let display_entry = |idx: usize| {
                self.recent_documents
                    .get(idx)
                    .map(|p| Self::middle_ellipsize_path(p.as_path(), MAX_RECENT_DISPLAY_CHARS))
                    .unwrap_or_default()
                    .into()
            };
            win.set_recent_count(self.recent_documents.len() as i32);
            win.set_recent_a(full_entry(0));
            win.set_recent_b(full_entry(1));
            win.set_recent_c(full_entry(2));
            win.set_recent_d(full_entry(3));
            win.set_recent_e(full_entry(4));
            win.set_recent_a_display(display_entry(0));
            win.set_recent_b_display(display_entry(1));
            win.set_recent_c_display(display_entry(2));
            win.set_recent_d_display(display_entry(3));
            win.set_recent_e_display(display_entry(4));
        }
    }

    fn update_image_resource_menu_display(&mut self) {
        if let Some(win) = self.window.upgrade() {
            let names = if self.document.is_some() {
                self.page_image_resource_names()
            } else {
                Vec::new()
            };

            if names.is_empty() {
                self.selected_image_resource_name = None;
            } else {
                let current = self.selected_image_resource_name.clone();
                if current
                    .as_ref()
                    .map(|s| names.iter().any(|n| n == s))
                    .unwrap_or(false)
                {
                    // keep current selection
                } else {
                    self.selected_image_resource_name = names.last().cloned();
                }
            }

            if names.is_empty() {
                self.image_resource_menu_offset = 0;
            } else {
                let max_start = ((names.len() - 1) / MAX_IMAGE_RESOURCE_MENU_ITEMS)
                    * MAX_IMAGE_RESOURCE_MENU_ITEMS;
                if self.image_resource_menu_offset > max_start {
                    self.image_resource_menu_offset = max_start;
                }
            }

            let start = self.image_resource_menu_offset;
            let end = (start + MAX_IMAGE_RESOURCE_MENU_ITEMS).min(names.len());
            let visible_count = end.saturating_sub(start);
            let entry = |idx: usize| names.get(start + idx).cloned().unwrap_or_default().into();
            let has_prev = start > 0;
            let has_next = end < names.len();
            let selected_name = self.selected_image_resource_name.clone().unwrap_or_default();
            let selected_visible_index = if selected_name.is_empty() {
                -1
            } else {
                names[start..end]
                    .iter()
                    .position(|n| n == &selected_name)
                    .map(|idx| idx as i32)
                    .unwrap_or(-1)
            };
            let page_label: slint::SharedString = if names.is_empty() {
                "".into()
            } else {
                format!("Resources {}-{} of {}", start + 1, end, names.len()).into()
            };

            win.set_image_resource_count(visible_count as i32);
            win.set_image_resource_a(entry(0));
            win.set_image_resource_b(entry(1));
            win.set_image_resource_c(entry(2));
            win.set_image_resource_d(entry(3));
            win.set_image_resource_e(entry(4));
            win.set_image_resource_f(entry(5));
            win.set_image_resource_g(entry(6));
            win.set_image_resource_h(entry(7));
            win.set_image_resource_i(entry(8));
            win.set_image_resource_j(entry(9));
            win.set_image_resource_has_prev(has_prev);
            win.set_image_resource_has_next(has_next);
            win.set_image_resource_page_label(page_label);
            win.set_image_resource_selected_visible_index(selected_visible_index);
            win.set_image_resource_selected_name(selected_name.into());
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
                let result = MuPdfRenderer::render_from_bytes(
                    &task.doc_bytes,
                    task.page_index,
                    task.zoom,
                )
                .or_else(|mupdf_err| {
                    tracing::debug!(
                        "MuPDF render from bytes failed ({mupdf_err}), trying file path"
                    );
                    MuPdfRenderer::render_from_path(&task.doc_path, task.page_index, task.zoom)
                })
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
