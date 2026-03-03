use pdf_annotations::{AddAnnotationCommand, types::{Annotation, AnnotationKind, Color, Rect}};
use pdf_core::{
    command::CommandHistory,
    document::Document,
    event::{DocumentEvent, EventBus},
};
use pdf_editor::{DeletePageCommand, RotatePageCommand};
use pdf_render::{PageCache, RenderEngine, SoftwareRenderer};
use slint::{Image, Rgba8Pixel, SharedPixelBuffer, Weak};
use std::sync::mpsc::Sender;

use crate::AppWindow;

const CACHE_CAPACITY: usize = 50;
const HISTORY_DEPTH: usize = 100;

pub struct AppController {
    window: Weak<AppWindow>,
    evt_tx: Sender<DocumentEvent>,
    document: Option<Document>,
    history: CommandHistory,
    cache: PageCache,
    renderer: SoftwareRenderer,
    zoom: f32,
    current_page: u32,
    bus: EventBus,
}

impl AppController {
    pub fn new(window: Weak<AppWindow>, evt_tx: Sender<DocumentEvent>) -> Self {
        Self {
            window,
            evt_tx,
            document: None,
            history: CommandHistory::new(HISTORY_DEPTH),
            cache: PageCache::new(CACHE_CAPACITY),
            renderer: SoftwareRenderer,
            zoom: 1.0,
            current_page: 0,
            bus: EventBus::new(),
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
                    me.emit(DocumentEvent::PageChanged { index: me.current_page });
                }
            }
        });

        win.on_prev_page(move || {
            let me = unsafe { &mut *ptr };
            if me.current_page > 0 {
                me.current_page -= 1;
                me.render_current_page();
                me.emit(DocumentEvent::PageChanged { index: me.current_page });
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
    }

    fn open_document_dialog(&mut self) {
        let path = match std::env::var("OPEN_PDF") {
            Ok(p) => std::path::PathBuf::from(p),
            Err(_) => {
                self.emit(DocumentEvent::StatusChanged {
                    message: "Set OPEN_PDF env var to open a file".into(),
                });
                return;
            }
        };
        match Document::open(&path) {
            Ok(doc) => {
                let title = doc.title.clone();
                let page_count = doc.page_count();
                self.document = Some(doc);
                self.current_page = 0;
                self.history.clear();
                self.cache.evict_document(self.document.as_ref().unwrap().id);
                self.render_current_page();
                self.emit(DocumentEvent::DocumentOpened { title, page_count });
                self.update_undo_redo_state();
            }
            Err(e) => {
                self.emit(DocumentEvent::Error { message: e.to_string() });
            }
        }
    }

    fn save_document(&mut self) {
        if let Some(doc) = &mut self.document {
            match doc.save() {
                Ok(()) => {
                    let path = doc.path.display().to_string();
                    self.emit(DocumentEvent::DocumentSaved { path });
                }
                Err(e) => self.emit(DocumentEvent::Error { message: e.to_string() }),
            }
        }
    }

    fn save_document_as_dialog(&mut self) {
        let path = match std::env::var("SAVE_PDF") {
            Ok(p) => std::path::PathBuf::from(p),
            Err(_) => {
                self.emit(DocumentEvent::StatusChanged {
                    message: "Set SAVE_PDF env var to save to a path".into(),
                });
                return;
            }
        };
        if let Some(doc) = &mut self.document {
            match doc.save_to(&path) {
                Ok(()) => self.emit(DocumentEvent::DocumentSaved {
                    path: path.display().to_string(),
                }),
                Err(e) => self.emit(DocumentEvent::Error { message: e.to_string() }),
            }
        }
    }

    fn close_document(&mut self) {
        if let Some(doc) = self.document.take() {
            self.cache.evict_document(doc.id);
        }
        self.current_page = 0;
        self.history.clear();
        self.emit(DocumentEvent::DocumentClosed);
    }

    fn set_zoom(&mut self, zoom: f32) {
        let zoom = zoom.clamp(0.1, 10.0);
        if let Some(doc) = &self.document {
            self.cache.evict_document(doc.id);
        }
        self.zoom = zoom;
        self.render_current_page();
        self.emit(DocumentEvent::ZoomChanged { factor: zoom });
    }

    fn undo(&mut self) {
        if let Some(doc) = &mut self.document {
            if let Err(e) = self.history.undo(doc) {
                self.emit(DocumentEvent::Error { message: e.to_string() });
            } else {
                self.render_current_page();
                self.update_undo_redo_state();
            }
        }
    }

    fn redo(&mut self) {
        if let Some(doc) = &mut self.document {
            if let Err(e) = self.history.redo(doc) {
                self.emit(DocumentEvent::Error { message: e.to_string() });
            } else {
                self.render_current_page();
                self.update_undo_redo_state();
            }
        }
    }

    fn add_highlight_annotation(&mut self) {
        if self.document.is_none() { return; }
        let annotation = Annotation::new(
            self.current_page,
            Rect { x: 72.0, y: 700.0, width: 200.0, height: 20.0 },
            AnnotationKind::Highlight { color: Color::yellow() },
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
                Err(e) => self.emit(DocumentEvent::Error { message: e.to_string() }),
            }
        }
    }

    fn add_note_annotation(&mut self) {
        if self.document.is_none() { return; }
        let annotation = Annotation::new(
            self.current_page,
            Rect { x: 500.0, y: 750.0, width: 20.0, height: 20.0 },
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
                Err(e) => self.emit(DocumentEvent::Error { message: e.to_string() }),
            }
        }
    }

    fn delete_current_page(&mut self) {
        if self.document.is_none() { return; }
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
                Err(e) => self.emit(DocumentEvent::Error { message: e.to_string() }),
            }
        }
    }

    fn rotate_current_page(&mut self) {
        if self.document.is_none() { return; }
        let page = self.current_page;
        let cmd = Box::new(RotatePageCommand::new(page, 90));
        if let Some(doc) = &mut self.document {
            match self.history.execute(cmd, doc) {
                Ok(()) => {
                    if let Some(ref doc) = self.document {
                        self.cache.evict_document(doc.id);
                    }
                    self.render_current_page();
                    self.emit(DocumentEvent::PageRotated { index: page, angle: 90 });
                    self.update_undo_redo_state();
                }
                Err(e) => self.emit(DocumentEvent::Error { message: e.to_string() }),
            }
        }
    }

    fn render_current_page(&mut self) {
        let (doc_id, page_count) = match &self.document {
            Some(d) => (d.id, d.page_count()),
            None => return,
        };
        if page_count == 0 { return; }
        let page = self.current_page.min(page_count - 1);

        let key = pdf_render::types::CacheKey::new(doc_id, page, self.zoom);
        if self.cache.get(&key).is_none() {
            if let Some(doc) = &self.document {
                match self.renderer.render_page(doc, page, self.zoom) {
                    Ok(rendered) => {
                        let k2 = pdf_render::types::CacheKey::new(doc_id, page, self.zoom);
                        self.cache.insert(k2, rendered);
                    }
                    Err(e) => {
                        self.emit(DocumentEvent::Error { message: e.to_string() });
                        return;
                    }
                }
            }
        }

        let key = pdf_render::types::CacheKey::new(doc_id, page, self.zoom);
        if let Some(rendered) = self.cache.get(&key) {
            let w = rendered.width;
            let h = rendered.height;
            let data = rendered.data.clone();
            let _ = rendered; // end borrow so we can use self.window below

            let mut buf = SharedPixelBuffer::<Rgba8Pixel>::new(w, h);
            buf.make_mut_bytes().copy_from_slice(&data);
            let image = Image::from_rgba8(buf);

            if let Some(win) = self.window.upgrade() {
                win.set_page_image(image);
                win.set_current_page(page as i32 + 1);
                win.set_page_count(page_count as i32);
            }
        }
    }

    fn update_undo_redo_state(&self) {
        if let Some(win) = self.window.upgrade() {
            win.set_can_undo(self.history.can_undo());
            win.set_can_redo(self.history.can_redo());
        }
    }

    fn emit(&self, event: DocumentEvent) {
        let _ = self.evt_tx.send(event);
    }
}
