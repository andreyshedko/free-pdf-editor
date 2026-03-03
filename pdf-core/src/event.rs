use serde::{Deserialize, Serialize};
use std::sync::mpsc::{self, Receiver, Sender};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DocumentEvent {
    DocumentOpened { title: String, page_count: u32 },
    DocumentClosed,
    DocumentModified,
    DocumentSaved { path: String },
    PageChanged { index: u32 },
    PageAdded { index: u32 },
    PageDeleted { index: u32 },
    PageRotated { index: u32, angle: i32 },
    PagesReordered,
    AnnotationAdded { page: u32, annotation_id: String },
    AnnotationRemoved { page: u32, annotation_id: String },
    SelectionChanged { page: u32, x: f64, y: f64, width: f64, height: f64 },
    FormFieldChanged { field_name: String },
    ZoomChanged { factor: f32 },
    StatusChanged { message: String },
    Error { message: String },
}

pub struct EventBus {
    subscribers: Vec<Sender<DocumentEvent>>,
}

impl std::fmt::Debug for EventBus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("EventBus")
            .field("subscribers", &self.subscribers.len())
            .finish()
    }
}

impl EventBus {
    pub fn new() -> Self {
        Self { subscribers: Vec::new() }
    }

    pub fn subscribe(&mut self) -> Receiver<DocumentEvent> {
        let (tx, rx) = mpsc::channel();
        self.subscribers.push(tx);
        rx
    }

    pub fn publish(&mut self, event: DocumentEvent) {
        self.subscribers.retain(|tx| tx.send(event.clone()).is_ok());
    }
}

impl Default for EventBus {
    fn default() -> Self { Self::new() }
}
