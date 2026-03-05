use lopdf::{
    content::{Content, Operation},
    dictionary, Object, Stream,
};
use pdf_core::{Document, DocumentCommand, OcrResult, PdfCoreError};

/// Name used to reference the invisible text font in the page resource dict.
const OCR_FONT_RESOURCE_NAME: &[u8] = b"OcrHelvetica";

/// Applies pre-computed OCR results to a PDF page by adding an invisible
/// text layer.
///
/// Each recognised [`pdf_core::TextRegion`] is written as a hidden `BT … ET`
/// block positioned at the region's coordinates.  PDF viewers can select and
/// copy the text while the original page imagery remains unchanged.
///
/// The Helvetica standard font is registered in the page's `/Resources/Font`
/// dictionary under the key `OcrHelvetica` so that the text operators are
/// valid according to the PDF specification.
///
/// # Undo
/// A snapshot of the document is taken before any changes so the command can
/// be undone via [`DocumentCommand::undo`].
#[derive(Debug)]
pub struct ApplyOcrCommand {
    result: OcrResult,
    snapshot: Option<Vec<u8>>,
}

impl ApplyOcrCommand {
    pub fn new(result: OcrResult) -> Self {
        Self {
            result,
            snapshot: None,
        }
    }
}

impl DocumentCommand for ApplyOcrCommand {
    fn description(&self) -> &str {
        "Apply OCR text layer"
    }

    fn execute(&mut self, doc: &mut Document) -> Result<(), PdfCoreError> {
        // If every region is empty there is nothing to write — return without
        // touching the document.
        let has_text = self
            .result
            .regions
            .iter()
            .any(|r| !r.text.is_empty());
        if !has_text {
            return Ok(());
        }

        // Snapshot the document for undo.
        let mut buf = std::io::Cursor::new(Vec::new());
        doc.inner_mut()
            .save_to(&mut buf)
            .map_err(|e| PdfCoreError::LopdfError(e.to_string()))?;
        self.snapshot = Some(buf.into_inner());

        let page = doc.get_page(self.result.page_index)?;
        let page_id = page.object_id;

        // Register a standard Helvetica font resource in the page so the Tf
        // operator below is valid per the PDF specification.
        ensure_ocr_font(doc, page_id)?;

        // Build an invisible text stream: render mode 3 (invisible).
        let mut ops: Vec<Operation> = Vec::new();
        ops.push(Operation::new("q", vec![]));

        for region in &self.result.regions {
            if region.text.is_empty() {
                continue;
            }

            ops.push(Operation::new("BT", vec![]));
            // Reference the font we registered above.
            ops.push(Operation::new(
                "Tf",
                vec![
                    Object::Name(OCR_FONT_RESOURCE_NAME.to_vec()),
                    Object::Real(region.height as f32),
                ],
            ));
            // Render mode 3 = invisible (does not modify clipping path).
            ops.push(Operation::new("Tr", vec![Object::Integer(3)]));
            // Position the text at the region origin (PDF y-axis up).
            ops.push(Operation::new(
                "Td",
                vec![
                    Object::Real(region.x as f32),
                    Object::Real(region.y as f32),
                ],
            ));
            ops.push(Operation::new(
                "Tj",
                vec![Object::string_literal(region.text.as_str())],
            ));
            ops.push(Operation::new("ET", vec![]));
        }

        ops.push(Operation::new("Q", vec![]));

        let encoded = Content { operations: ops }
            .encode()
            .map_err(|e| PdfCoreError::LopdfError(e.to_string()))?;

        let new_stream_id = doc
            .inner_mut()
            .add_object(Stream::new(dictionary! {}, encoded));

        // Append the new stream to the page's /Contents.
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

        tracing::info!(
            page_index = self.result.page_index,
            regions = self.result.regions.len(),
            "OCR text layer applied"
        );
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

/// Ensure the page's `/Resources/Font` dictionary contains an entry for the
/// invisible text font used by the OCR text layer.
///
/// The font is a standard Type1 Helvetica font which is guaranteed to be
/// available in all conforming PDF processors.
///
/// This function walks the `/Parent` chain to find the effective `/Resources`
/// dictionary (including inherited resources) and dereferences indirect
/// `/Resources` and `/Font` objects before merging in the new font entry.
/// The merged dictionary is written inline on the page node so that the
/// page-level resources shadow any inherited ancestor resources.
///
/// The font object is only created and added when `OcrHelvetica` is not
/// already present in the resolved font dictionary, making repeated calls
/// idempotent.
fn ensure_ocr_font(doc: &mut Document, page_id: lopdf::ObjectId) -> Result<(), PdfCoreError> {
    // Walk up the /Parent chain to find the nearest /Resources dict,
    // resolving indirect references along the way.
    let mut resources_dict: lopdf::Dictionary = {
        let inner = doc.inner();
        let mut current_id = page_id;
        let mut found: Option<lopdf::Dictionary> = None;

        loop {
            let dict_opt = inner
                .get_object(current_id)
                .ok()
                .and_then(|o| o.as_dict().ok());
            let dict = match dict_opt {
                Some(d) => d,
                None => break,
            };

            if let Ok(res_obj) = dict.get(b"Resources") {
                let resolved = if let Ok(res_id) = res_obj.as_reference() {
                    inner
                        .get_object(res_id)
                        .ok()
                        .and_then(|ro| ro.as_dict().ok())
                        .cloned()
                } else {
                    res_obj.as_dict().ok().cloned()
                };
                if let Some(d) = resolved {
                    found = Some(d);
                    break;
                }
            }

            // No /Resources here — follow /Parent if present.
            match dict.get(b"Parent").ok().and_then(|p| p.as_reference().ok()) {
                Some(parent_id) => current_id = parent_id,
                None => break,
            }
        }

        found.unwrap_or_default()
    };

    // Resolve the /Font sub-dictionary (may itself be an indirect reference).
    let mut font_dict: lopdf::Dictionary = {
        let font_obj = resources_dict.get(b"Font").ok().cloned();
        match font_obj {
            Some(Object::Reference(font_ref_id)) => doc
                .inner()
                .get_object(font_ref_id)
                .ok()
                .and_then(|o| o.as_dict().ok())
                .cloned()
                .unwrap_or_else(lopdf::Dictionary::new),
            Some(Object::Dictionary(d)) => d,
            _ => lopdf::Dictionary::new(),
        }
    };

    // Only create and register the font object when the entry is absent.
    if font_dict.get(OCR_FONT_RESOURCE_NAME).is_err() {
        let mut d = lopdf::Dictionary::new();
        d.set("Type", Object::Name(b"Font".to_vec()));
        d.set("Subtype", Object::Name(b"Type1".to_vec()));
        d.set("BaseFont", Object::Name(b"Helvetica".to_vec()));
        let font_id = doc.inner_mut().add_object(Object::Dictionary(d));
        font_dict.set(OCR_FONT_RESOURCE_NAME, Object::Reference(font_id));
    }

    resources_dict.set("Font", Object::Dictionary(font_dict));

    // Write the effective Resources dict inline on the page node so it
    // shadows (but does not destroy) ancestor-inherited resources.
    doc.inner_mut()
        .get_object_mut(page_id)
        .map_err(|e| PdfCoreError::LopdfError(e.to_string()))?
        .as_dict_mut()
        .map_err(|e| PdfCoreError::LopdfError(e.to_string()))?
        .set("Resources", Object::Dictionary(resources_dict));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use lopdf::{content::Content, dictionary, Document as LopdfDoc, Object, Stream};
    use pdf_core::{OcrResult, TextRegion};
    use tempfile::NamedTempFile;

    fn minimal_pdf() -> NamedTempFile {
        let mut doc = LopdfDoc::with_version("1.7");
        let pages_id = doc.new_object_id();
        let page_id = doc.new_object_id();
        let content_id = doc.add_object(Stream::new(dictionary! {}, b"".to_vec()));
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

    fn ocr_result_with_text(page_index: u32, text: &str) -> OcrResult {
        OcrResult {
            page_index,
            regions: vec![TextRegion {
                text: text.to_owned(),
                x: 100.0,
                y: 700.0,
                width: 200.0,
                height: 12.0,
                confidence: 0.99,
            }],
            full_text: text.to_owned(),
        }
    }

    fn count_tj_ops(doc: &Document, page_index: u32) -> usize {
        let page_id = doc.get_page(page_index).expect("page").object_id;
        let inner = doc.inner();

        let content_ids: Vec<lopdf::ObjectId> = match inner
            .get_object(page_id)
            .ok()
            .and_then(|o| o.as_dict().ok())
            .and_then(|d| d.get(b"Contents").ok())
        {
            Some(Object::Reference(id)) => vec![*id],
            Some(Object::Array(arr)) => arr.iter().filter_map(|o| o.as_reference().ok()).collect(),
            _ => Vec::new(),
        };

        let mut count = 0;
        for stream_id in content_ids {
            let bytes = inner
                .get_object(stream_id)
                .ok()
                .and_then(|o| o.as_stream().ok())
                .map(|s| s.content.clone())
                .unwrap_or_default();
            if let Ok(parsed) = Content::decode(&bytes) {
                for op in parsed.operations {
                    if op.operator == "Tj" {
                        count += 1;
                    }
                }
            }
        }
        count
    }

    #[test]
    fn apply_ocr_adds_text_stream() {
        let f = minimal_pdf();
        let mut doc = Document::open(f.path()).expect("open");

        assert_eq!(count_tj_ops(&doc, 0), 0, "no Tj before OCR");

        let result = ocr_result_with_text(0, "Hello OCR");
        let mut cmd = ApplyOcrCommand::new(result);
        cmd.execute(&mut doc).expect("execute");

        assert_eq!(count_tj_ops(&doc, 0), 1, "one Tj after OCR");
    }

    #[test]
    fn apply_ocr_undo_removes_text_layer() {
        let f = minimal_pdf();
        let mut doc = Document::open(f.path()).expect("open");

        let before = count_tj_ops(&doc, 0);

        let result = ocr_result_with_text(0, "Hello OCR");
        let mut cmd = ApplyOcrCommand::new(result);
        cmd.execute(&mut doc).expect("execute");
        cmd.undo(&mut doc).expect("undo");

        assert_eq!(
            count_tj_ops(&doc, 0),
            before,
            "undo should restore the original Tj count"
        );
    }

    #[test]
    fn apply_ocr_out_of_range_page_fails() {
        let f = minimal_pdf();
        let mut doc = Document::open(f.path()).expect("open");

        let result = OcrResult {
            page_index: 99,
            regions: vec![TextRegion {
                text: "hello".to_owned(),
                x: 0.0,
                y: 0.0,
                width: 100.0,
                height: 12.0,
                confidence: 0.9,
            }],
            full_text: "hello".to_owned(),
        };
        let mut cmd = ApplyOcrCommand::new(result);
        assert!(cmd.execute(&mut doc).is_err());
    }

    #[test]
    fn apply_ocr_empty_regions_is_noop() {
        let f = minimal_pdf();
        let mut doc = Document::open(f.path()).expect("open");

        let result = OcrResult {
            page_index: 0,
            regions: vec![],
            full_text: String::new(),
        };
        let mut cmd = ApplyOcrCommand::new(result);
        cmd.execute(&mut doc).expect("execute");

        // Empty regions should not add any Tj operators.
        assert_eq!(count_tj_ops(&doc, 0), 0);
    }
}
