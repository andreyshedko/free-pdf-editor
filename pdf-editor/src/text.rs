use lopdf::{
    content::{Content, Operation},
    Object, Stream,
};
use pdf_core::{Document, DocumentCommand, PdfCoreError};

#[derive(Debug)]
pub struct InsertTextCommand {
    page_index: u32,
    text: String,
    x: f32,
    y: f32,
    font_size: f32,
    snapshot: Option<Vec<u8>>,
}

impl InsertTextCommand {
    pub fn new(page_index: u32, text: impl Into<String>, x: f32, y: f32, font_size: f32) -> Self {
        Self {
            page_index,
            text: text.into(),
            x,
            y,
            font_size,
            snapshot: None,
        }
    }
}

impl DocumentCommand for InsertTextCommand {
    fn description(&self) -> &str {
        "Insert text"
    }

    fn execute(&mut self, doc: &mut Document) -> Result<(), PdfCoreError> {
        let mut buf = std::io::Cursor::new(Vec::new());
        doc.inner_mut()
            .save_to(&mut buf)
            .map_err(|e| PdfCoreError::LopdfError(e.to_string()))?;
        self.snapshot = Some(buf.into_inner());

        let page = doc.get_page(self.page_index)?;
        let page_id = page.object_id;

        let ops = vec![
            Operation::new("BT", vec![]),
            Operation::new(
                "Tf",
                vec![
                    Object::Name(b"Helvetica".to_vec()),
                    Object::Real(self.font_size),
                ],
            ),
            Operation::new("Td", vec![Object::Real(self.x), Object::Real(self.y)]),
            Operation::new("Tj", vec![Object::string_literal(self.text.clone())]),
            Operation::new("ET", vec![]),
        ];
        let content = Content { operations: ops };
        let encoded = content
            .encode()
            .map_err(|e| PdfCoreError::LopdfError(e.to_string()))?;

        let new_stream_id = doc
            .inner_mut()
            .add_object(Stream::new(lopdf::dictionary! {}, encoded));

        let inner = doc.inner_mut();
        let page_dict = inner
            .get_object_mut(page_id)
            .map_err(|e| PdfCoreError::LopdfError(e.to_string()))?
            .as_dict_mut()
            .map_err(|e| PdfCoreError::LopdfError(e.to_string()))?;

        match page_dict.get(b"Contents") {
            Ok(Object::Array(existing)) => {
                let mut arr = existing.clone();
                arr.push(Object::Reference(new_stream_id));
                page_dict.set("Contents", Object::Array(arr));
            }
            Ok(Object::Reference(old_id)) => {
                let old_id = *old_id;
                page_dict.set(
                    "Contents",
                    Object::Array(vec![
                        Object::Reference(old_id),
                        Object::Reference(new_stream_id),
                    ]),
                );
            }
            _ => {
                page_dict.set("Contents", Object::Reference(new_stream_id));
            }
        }
        tracing::debug!(page_index = self.page_index, text = %self.text, "text inserted");
        Ok(())
    }

    fn undo(&mut self, doc: &mut Document) -> Result<(), PdfCoreError> {
        let snap = self.snapshot.as_ref().ok_or(PdfCoreError::NotUndoable)?;
        let restored =
            lopdf::Document::load_mem(snap).map_err(|e| PdfCoreError::LopdfError(e.to_string()))?;
        *doc.inner_mut() = restored;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use lopdf::{dictionary, Document as LopdfDoc, Object, Stream};
    use pdf_core::Document;

    use tempfile::NamedTempFile;

    fn single_page_pdf() -> NamedTempFile {
        let mut doc = LopdfDoc::with_version("1.7");
        let pages_id = doc.new_object_id();
        let page_id = doc.new_object_id();
        let content = Stream::new(dictionary! {}, b"BT ET".to_vec());
        let content_id = doc.add_object(content);
        let page = Object::Dictionary(dictionary! {
            "Type"     => Object::Name(b"Page".to_vec()),
            "Parent"   => Object::Reference(pages_id),
            "MediaBox" => Object::Array(vec![
                Object::Integer(0), Object::Integer(0),
                Object::Integer(595), Object::Integer(842),
            ]),
            "Contents" => Object::Reference(content_id),
        });
        doc.objects.insert(page_id, page);
        let pages = Object::Dictionary(dictionary! {
            "Type"  => Object::Name(b"Pages".to_vec()),
            "Kids"  => Object::Array(vec![Object::Reference(page_id)]),
            "Count" => Object::Integer(1),
        });
        doc.objects.insert(pages_id, pages);
        let catalog_id = doc.add_object(dictionary! {
            "Type"  => Object::Name(b"Catalog".to_vec()),
            "Pages" => Object::Reference(pages_id),
        });
        doc.trailer.set("Root", Object::Reference(catalog_id));
        let mut f = NamedTempFile::new().expect("temp");
        doc.save_to(f.as_file_mut()).expect("save");
        f
    }

    #[test]
    fn insert_text_execute_and_undo() {
        let f = single_page_pdf();
        let mut doc = Document::open(f.path()).expect("open");
        let mut cmd = InsertTextCommand::new(0, "Hello", 100.0, 700.0, 12.0);
        cmd.execute(&mut doc).expect("execute");
        // Page count should remain unchanged
        assert_eq!(doc.page_count(), 1);
        cmd.undo(&mut doc).expect("undo");
        assert_eq!(doc.page_count(), 1);
    }

    #[test]
    fn insert_text_out_of_range_fails() {
        let f = single_page_pdf();
        let mut doc = Document::open(f.path()).expect("open");
        let mut cmd = InsertTextCommand::new(99, "Hello", 100.0, 700.0, 12.0);
        assert!(cmd.execute(&mut doc).is_err());
    }
}
