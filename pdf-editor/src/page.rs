use pdf_core::{Document, DocumentCommand, PdfCoreError};

#[derive(Debug)]
pub struct DeletePageCommand {
    page_index: u32,
    snapshot: Option<Vec<u8>>,
}

impl DeletePageCommand {
    pub fn new(page_index: u32) -> Self {
        Self { page_index, snapshot: None }
    }
}

fn snapshot_doc(doc: &mut Document) -> Result<Vec<u8>, PdfCoreError> {
    let mut buf = std::io::Cursor::new(Vec::new());
    doc.inner_mut().save_to(&mut buf)
        .map_err(|e| PdfCoreError::LopdfError(e.to_string()))?;
    Ok(buf.into_inner())
}

impl DocumentCommand for DeletePageCommand {
    fn description(&self) -> &str { "Delete page" }

    fn execute(&mut self, doc: &mut Document) -> Result<(), PdfCoreError> {
        self.snapshot = Some(snapshot_doc(doc)?);
        doc.delete_page(self.page_index)
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
pub struct RotatePageCommand {
    page_index: u32,
    angle: i64,
    previous_angle: i64,
}

impl RotatePageCommand {
    pub fn new(page_index: u32, angle: i64) -> Self {
        Self { page_index, angle, previous_angle: 0 }
    }
}

impl DocumentCommand for RotatePageCommand {
    fn description(&self) -> &str { "Rotate page" }

    fn execute(&mut self, doc: &mut Document) -> Result<(), PdfCoreError> {
        let page = doc.get_page(self.page_index)?;
        let current_rotation = doc.inner()
            .get_object(page.object_id)
            .ok()
            .and_then(|o| o.as_dict().ok())
            .and_then(|d| d.get(b"Rotate").ok())
            .and_then(|o| o.as_i64().ok())
            .unwrap_or(0);
        self.previous_angle = current_rotation;
        doc.rotate_page(self.page_index, self.angle)
    }

    fn undo(&mut self, doc: &mut Document) -> Result<(), PdfCoreError> {
        doc.rotate_page(self.page_index, self.previous_angle)
    }
}

#[derive(Debug)]
pub struct ReorderPagesCommand {
    new_order: Vec<u32>,
    original_order: Vec<u32>,
}

impl ReorderPagesCommand {
    pub fn new(new_order: Vec<u32>) -> Self {
        let len = new_order.len();
        Self {
            new_order,
            original_order: (0..len as u32).collect(),
        }
    }
}

impl DocumentCommand for ReorderPagesCommand {
    fn description(&self) -> &str { "Reorder pages" }

    fn execute(&mut self, doc: &mut Document) -> Result<(), PdfCoreError> {
        let n = doc.page_count() as usize;
        self.original_order = (0..n as u32).collect();
        doc.reorder_pages(&self.new_order)
    }

    fn undo(&mut self, doc: &mut Document) -> Result<(), PdfCoreError> {
        doc.reorder_pages(&self.original_order)
    }
}

#[derive(Debug)]
pub struct MergeDocumentCommand {
    other: Document,
    added_page_count: u32,
}

impl MergeDocumentCommand {
    pub fn new(other: Document) -> Self {
        let added_page_count = other.page_count();
        Self { other, added_page_count }
    }
}

impl DocumentCommand for MergeDocumentCommand {
    fn description(&self) -> &str { "Merge document" }

    fn execute(&mut self, doc: &mut Document) -> Result<(), PdfCoreError> {
        doc.merge_document(&mut self.other)
    }

    fn undo(&mut self, doc: &mut Document) -> Result<(), PdfCoreError> {
        let total = doc.page_count();
        let first_added = total.saturating_sub(self.added_page_count);
        for i in (first_added..total).rev() {
            doc.delete_page(i)?;
        }
        Ok(())
    }
}
