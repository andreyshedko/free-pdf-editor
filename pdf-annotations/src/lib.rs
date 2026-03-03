pub mod commands;
pub mod io;
pub mod types;

pub use commands::{AddAnnotationCommand, RemoveAnnotationCommand};
pub use types::{Annotation, AnnotationId, AnnotationKind, Color, Rect};
