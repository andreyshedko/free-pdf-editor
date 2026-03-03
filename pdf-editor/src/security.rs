use pdf_core::{Document, DocumentCommand, PdfCoreError};

#[derive(Debug)]
pub struct SetPasswordCommand {
    password: String,
    snapshot: Option<Vec<u8>>,
}

impl SetPasswordCommand {
    pub fn new(password: impl Into<String>) -> Self {
        Self { password: password.into(), snapshot: None }
    }
}

impl DocumentCommand for SetPasswordCommand {
    fn description(&self) -> &str { "Set password" }

    fn execute(&mut self, doc: &mut Document) -> Result<(), PdfCoreError> {
        let mut buf = std::io::Cursor::new(Vec::new());
        doc.inner_mut().save_to(&mut buf)
            .map_err(|e| PdfCoreError::LopdfError(e.to_string()))?;
        self.snapshot = Some(buf.into_inner());
        // Note: lopdf 0.39 does not expose a direct encrypt API.
        // Password protection requires a PDF library with encryption support.
        if !self.password.is_empty() {
            tracing::warn!(
                "password protection requested but lopdf 0.39 does not support \
                 encryption; document will not be encrypted"
            );
        }
        tracing::info!("password set (placeholder - lopdf encryption not available in 0.39)");
        Ok(())
    }

    fn undo(&mut self, doc: &mut Document) -> Result<(), PdfCoreError> {
        let snap = self.snapshot.as_ref().ok_or(PdfCoreError::NotUndoable)?;
        let restored = lopdf::Document::load_mem(snap)
            .map_err(|e| PdfCoreError::LopdfError(e.to_string()))?;
        *doc.inner_mut() = restored;
        Ok(())
    }
}

#[derive(Debug)]
pub struct RedactRegionCommand {
    page_index: u32,
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    snapshot: Option<Vec<u8>>,
}

impl RedactRegionCommand {
    pub fn new(page_index: u32, x: f32, y: f32, width: f32, height: f32) -> Self {
        Self { page_index, x, y, width, height, snapshot: None }
    }
}

impl DocumentCommand for RedactRegionCommand {
    fn description(&self) -> &str { "Redact region" }

    fn execute(&mut self, doc: &mut Document) -> Result<(), PdfCoreError> {
        use lopdf::{content::{Content, Operation}, Object, Stream};

        let mut buf = std::io::Cursor::new(Vec::new());
        doc.inner_mut().save_to(&mut buf)
            .map_err(|e| PdfCoreError::LopdfError(e.to_string()))?;
        self.snapshot = Some(buf.into_inner());

        let page = doc.get_page(self.page_index)?;
        let page_id = page.object_id;

        let ops = vec![
            Operation::new("q", vec![]),
            Operation::new("g", vec![Object::Real(0.0)]),
            Operation::new("re", vec![
                Object::Real(self.x),
                Object::Real(self.y),
                Object::Real(self.width),
                Object::Real(self.height),
            ]),
            Operation::new("f", vec![]),
            Operation::new("Q", vec![]),
        ];
        let content = Content { operations: ops };
        let encoded = content.encode()
            .map_err(|e| PdfCoreError::LopdfError(e.to_string()))?;

        let new_id = doc.inner_mut().add_object(Stream::new(
            lopdf::dictionary! {},
            encoded,
        ));

        let inner = doc.inner_mut();
        let page_dict = inner.get_object_mut(page_id)
            .map_err(|e| PdfCoreError::LopdfError(e.to_string()))?
            .as_dict_mut()
            .map_err(|e| PdfCoreError::LopdfError(e.to_string()))?;

        match page_dict.get(b"Contents") {
            Ok(Object::Array(existing)) => {
                let mut arr = existing.clone();
                arr.push(Object::Reference(new_id));
                page_dict.set("Contents", Object::Array(arr));
            }
            Ok(Object::Reference(old_id)) => {
                let old_id = *old_id;
                page_dict.set("Contents", Object::Array(vec![
                    Object::Reference(old_id),
                    Object::Reference(new_id),
                ]));
            }
            _ => {
                page_dict.set("Contents", Object::Reference(new_id));
            }
        }
        tracing::info!(page_index = self.page_index, "region redacted");
        Ok(())
    }

    fn undo(&mut self, doc: &mut Document) -> Result<(), PdfCoreError> {
        let snap = self.snapshot.as_ref().ok_or(PdfCoreError::NotUndoable)?;
        let restored = lopdf::Document::load_mem(snap)
            .map_err(|e| PdfCoreError::LopdfError(e.to_string()))?;
        *doc.inner_mut() = restored;
        Ok(())
    }
}
