use pdf_core::{Document, DocumentCommand, PdfCoreError};
use lopdf::{content::{Content, Operation}, dictionary, Object, Stream};

// ---------------------------------------------------------------------------
// Shared helpers used by text commands
// ---------------------------------------------------------------------------

/// Collect the object IDs of every content stream attached to a page.
fn collect_content_ids(
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
        Some(Object::Array(arr)) => {
            arr.iter().filter_map(|o| o.as_reference().ok()).collect()
        }
        _ => vec![],
    }
}

/// In-place replacement of the matching literal string inside a `Tj` or `TJ`
/// operation.  Returns `true` if a replacement was made.
fn replace_text_in_op(op: &mut Operation, target_bytes: &[u8], new_text: &str) -> bool {
    match op.operator.as_str() {
        "Tj" => {
            if let Some(Object::String(ref bytes, _)) = op.operands.first() {
                if bytes.as_slice() == target_bytes {
                    op.operands[0] = Object::string_literal(new_text.to_owned());
                    return true;
                }
            }
            false
        }
        "TJ" => {
            if let Some(Object::Array(ref arr)) = op.operands.first() {
                let mut changed = false;
                let new_arr: Vec<Object> = arr
                    .iter()
                    .map(|elem| {
                        if let Object::String(ref b, _) = elem {
                            if b.as_slice() == target_bytes {
                                changed = true;
                                return Object::string_literal(new_text.to_owned());
                            }
                        }
                        elem.clone()
                    })
                    .collect();
                if changed {
                    op.operands[0] = Object::Array(new_arr);
                    return true;
                }
            }
            false
        }
        _ => false,
    }
}

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
    fn description(&self) -> &str { "Insert text" }

    fn execute(&mut self, doc: &mut Document) -> Result<(), PdfCoreError> {
        let mut buf = std::io::Cursor::new(Vec::new());
        doc.inner_mut().save_to(&mut buf)
            .map_err(|e| PdfCoreError::LopdfError(e.to_string()))?;
        self.snapshot = Some(buf.into_inner());

        let page = doc.get_page(self.page_index)?;
        let page_id = page.object_id;

        let ops = vec![
            Operation::new("BT", vec![]),
            Operation::new("Tf", vec![
                Object::Name(b"Helvetica".to_vec()),
                Object::Real(self.font_size),
            ]),
            Operation::new("Td", vec![
                Object::Real(self.x),
                Object::Real(self.y),
            ]),
            Operation::new("Tj", vec![Object::string_literal(self.text.clone())]),
            Operation::new("ET", vec![]),
        ];
        let content = Content { operations: ops };
        let encoded = content.encode()
            .map_err(|e| PdfCoreError::LopdfError(e.to_string()))?;

        let new_stream_id = doc.inner_mut().add_object(Stream::new(
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
                arr.push(Object::Reference(new_stream_id));
                page_dict.set("Contents", Object::Array(arr));
            }
            Ok(Object::Reference(old_id)) => {
                let old_id = *old_id;
                page_dict.set("Contents", Object::Array(vec![
                    Object::Reference(old_id),
                    Object::Reference(new_stream_id),
                ]));
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
        let restored = lopdf::Document::load_mem(snap)
            .map_err(|e| PdfCoreError::LopdfError(e.to_string()))?;
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

// ---------------------------------------------------------------------------
// ModifyTextCommand
// ---------------------------------------------------------------------------

/// Finds every literal-string occurrence of `old_text` in `Tj` and `TJ`
/// operators across all content streams on `page_index` and replaces it with
/// `new_text`.
///
/// All content streams on the target page are merged into a single new stream
/// after the replacement (the same approach used by `RedactRegionCommand`).
/// Undo is supported via a full-document snapshot.
#[derive(Debug)]
pub struct ModifyTextCommand {
    page_index: u32,
    old_text: String,
    new_text: String,
    snapshot: Option<Vec<u8>>,
}

impl ModifyTextCommand {
    pub fn new(
        page_index: u32,
        old_text: impl Into<String>,
        new_text: impl Into<String>,
    ) -> Self {
        Self {
            page_index,
            old_text: old_text.into(),
            new_text: new_text.into(),
            snapshot: None,
        }
    }
}

impl DocumentCommand for ModifyTextCommand {
    fn description(&self) -> &str { "Modify text" }

    fn execute(&mut self, doc: &mut Document) -> Result<(), PdfCoreError> {
        let mut buf = std::io::Cursor::new(Vec::new());
        doc.inner_mut()
            .save_to(&mut buf)
            .map_err(|e| PdfCoreError::LopdfError(e.to_string()))?;
        self.snapshot = Some(buf.into_inner());

        // Decompress so we can read raw content bytes.
        doc.inner_mut().decompress();

        let page_id = doc.get_page(self.page_index)?.object_id;

        let content_ids: Vec<lopdf::ObjectId> = {
            let inner = doc.inner();
            collect_content_ids(inner, page_id)
        };

        if content_ids.is_empty() {
            return Ok(());
        }

        let old_bytes = self.old_text.as_bytes().to_vec();
        let new_text = self.new_text.clone();

        // Collect operations from every content stream and apply replacements.
        let mut all_ops: Vec<Operation> = Vec::new();
        for stream_id in &content_ids {
            let bytes: Option<Vec<u8>> = {
                let inner = doc.inner();
                inner
                    .get_object(*stream_id)
                    .ok()
                    .and_then(|o| o.as_stream().ok())
                    .map(|s| s.content.clone())
            };

            match bytes {
                Some(b) => {
                    let parsed = Content::decode(&b)
                        .map_err(|e| PdfCoreError::LopdfError(e.to_string()))?;
                    all_ops.extend(parsed.operations.into_iter().map(|mut op| {
                        replace_text_in_op(&mut op, &old_bytes, &new_text);
                        op
                    }));
                }
                // If there is no stream or we cannot retrieve its bytes, skip it as before.
                None => continue,
            }
        }

        let encoded = Content { operations: all_ops }
            .encode()
            .map_err(|e| PdfCoreError::LopdfError(e.to_string()))?;

        let new_id = doc
            .inner_mut()
            .add_object(Stream::new(lopdf::dictionary! {}, encoded));

        match doc
            .inner_mut()
            .get_object_mut(page_id)
            .ok()
            .and_then(|o| o.as_dict_mut().ok())
        {
            Some(page_dict) => {
                page_dict.set("Contents", Object::Reference(new_id));
            }
            None => {
                return Err(PdfCoreError::LopdfError(
                    "failed to update page /Contents".into(),
                ))
            }
        }

        tracing::debug!(
            page_index = self.page_index,
            old_text = %self.old_text,
            new_text = %self.new_text,
            "text modified"
        );
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

#[cfg(test)]
mod modify_tests {
    use super::*;
    use lopdf::{content::Content as LContent, content::Operation as LOp, dictionary,
                Document as LopdfDoc, Object, Stream};
    use pdf_core::Document;
    use tempfile::NamedTempFile;

    fn pdf_with_text(text: &str) -> NamedTempFile {
        let ops = vec![
            LOp::new("BT", vec![]),
            LOp::new("Tf", vec![
                Object::Name(b"Helvetica".to_vec()),
                Object::Real(12.0),
            ]),
            LOp::new("Td", vec![Object::Real(100.0), Object::Real(700.0)]),
            LOp::new("Tj", vec![Object::string_literal(text.to_owned())]),
            LOp::new("ET", vec![]),
        ];
        let bytes = LContent { operations: ops }.encode().expect("encode");
        build_pdf(bytes)
    }

    fn build_pdf(content_bytes: Vec<u8>) -> NamedTempFile {
        let mut doc = LopdfDoc::with_version("1.7");
        let pages_id = doc.new_object_id();
        let page_id = doc.new_object_id();
        let content_id = doc.add_object(Stream::new(dictionary! {}, content_bytes));
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

    fn page_tj_strings(doc: &Document) -> Vec<Vec<u8>> {
        use lopdf::content::Content;
        let page_id = doc.get_page(0).unwrap().object_id;
        let inner = doc.inner();
        let content_ids: Vec<lopdf::ObjectId> =
            match inner.get_object(page_id).ok()
                .and_then(|o| o.as_dict().ok())
                .and_then(|d| d.get(b"Contents").ok())
            {
                Some(Object::Reference(id)) => vec![*id],
                Some(Object::Array(arr)) => {
                    arr.iter().filter_map(|o| o.as_reference().ok()).collect()
                }
                _ => vec![],
            };
        let mut result = Vec::new();
        for sid in content_ids {
            let bytes = inner.get_object(sid).ok()
                .and_then(|o| o.as_stream().ok())
                .map(|s| s.content.clone())
                .unwrap_or_default();
            if let Ok(parsed) = Content::decode(&bytes) {
                for op in parsed.operations {
                    if op.operator == "Tj" {
                        if let Some(Object::String(b, _)) = op.operands.first() {
                            result.push(b.clone());
                        }
                    }
                }
            }
        }
        result
    }

    #[test]
    fn modify_text_replaces_matching_string() {
        let f = pdf_with_text("hello");
        let mut doc = Document::open(f.path()).expect("open");
        let mut cmd = ModifyTextCommand::new(0, "hello", "world");
        cmd.execute(&mut doc).expect("execute");
        let strings = page_tj_strings(&doc);
        assert!(
            strings.iter().any(|b| b == b"world"),
            "new text should appear in content stream"
        );
        assert!(
            !strings.iter().any(|b| b == b"hello"),
            "old text should not appear after modification"
        );
    }

    #[test]
    fn modify_text_undo_restores_original() {
        let f = pdf_with_text("hello");
        let mut doc = Document::open(f.path()).expect("open");
        let mut cmd = ModifyTextCommand::new(0, "hello", "world");
        cmd.execute(&mut doc).expect("execute");
        cmd.undo(&mut doc).expect("undo");
        let strings = page_tj_strings(&doc);
        assert!(
            strings.iter().any(|b| b == b"hello"),
            "original text should be restored after undo"
        );
    }

    #[test]
    fn modify_text_out_of_range_fails() {
        let f = pdf_with_text("hello");
        let mut doc = Document::open(f.path()).expect("open");
        let mut cmd = ModifyTextCommand::new(99, "hello", "world");
        assert!(cmd.execute(&mut doc).is_err());
    }
}

// ---------------------------------------------------------------------------
// FontSubstitutionCommand
// ---------------------------------------------------------------------------

/// The 14 standard PDF Type 1 font names that can be embedded without
/// additional font data.
const STANDARD_PDF_FONTS: &[&str] = &[
    "Courier",
    "Courier-Bold",
    "Courier-BoldOblique",
    "Courier-Oblique",
    "Helvetica",
    "Helvetica-Bold",
    "Helvetica-BoldOblique",
    "Helvetica-Oblique",
    "Times-Roman",
    "Times-Bold",
    "Times-Italic",
    "Times-BoldItalic",
    "Symbol",
    "ZapfDingbats",
];

/// Replaces all references to `old_font_name` in `Tf` operators across every
/// content stream on `page_index` with `new_font_name`.
///
/// If `new_font_name` is one of the 14 standard PDF fonts and is not yet
/// present in the page's `/Resources/Font` dictionary, a minimal Type1 font
/// entry is added automatically so the document remains valid.
///
/// Undo is supported via a full-document snapshot.
#[derive(Debug)]
pub struct FontSubstitutionCommand {
    page_index: u32,
    old_font_name: String,
    new_font_name: String,
    snapshot: Option<Vec<u8>>,
}

impl FontSubstitutionCommand {
    pub fn new(
        page_index: u32,
        old_font_name: impl Into<String>,
        new_font_name: impl Into<String>,
    ) -> Self {
        Self {
            page_index,
            old_font_name: old_font_name.into(),
            new_font_name: new_font_name.into(),
            snapshot: None,
        }
    }
}

impl DocumentCommand for FontSubstitutionCommand {
    fn description(&self) -> &str { "Substitute font" }

    fn execute(&mut self, doc: &mut Document) -> Result<(), PdfCoreError> {
        let mut buf = std::io::Cursor::new(Vec::new());
        doc.inner_mut()
            .save_to(&mut buf)
            .map_err(|e| PdfCoreError::LopdfError(e.to_string()))?;
        self.snapshot = Some(buf.into_inner());

        doc.inner_mut().decompress();

        let page_id = doc.get_page(self.page_index)?.object_id;

        let content_ids: Vec<lopdf::ObjectId> = {
            let inner = doc.inner();
            collect_content_ids(inner, page_id)
        };

        if content_ids.is_empty() {
            return Ok(());
        }

        let old_name_bytes = self.old_font_name.as_bytes().to_vec();
        let new_name_bytes = self.new_font_name.as_bytes().to_vec();

        // Replace font name in every content stream and merge into one.
        let mut all_ops: Vec<Operation> = Vec::new();
        for stream_id in &content_ids {
            let bytes: Option<Vec<u8>> = {
                let inner = doc.inner();
                inner
                    .get_object(*stream_id)
                    .ok()
                    .and_then(|o| o.as_stream().ok())
                    .map(|s| s.content.clone())
            };

            // If we have a content stream but cannot decode it, fail the command
            // so that the original /Contents remains unchanged.
            if let Some(b) = bytes {
                let parsed = Content::decode(&b)
                    .map_err(|e| PdfCoreError::LopdfError(e.to_string()))?;

                all_ops.extend(parsed.operations.into_iter().map(|mut op| {
                    if op.operator == "Tf" {
                        if let Some(Object::Name(ref name)) = op.operands.first() {
                            if name == &old_name_bytes {
                                op.operands[0] =
                                    Object::Name(new_name_bytes.clone());
                            }
                        }
                    }
                    op
                }));
            } else {
                // If there is no stream content for this ID, keep existing behavior
                // and skip it.
                continue;
            }
        }

        let encoded = Content { operations: all_ops }
            .encode()
            .map_err(|e| PdfCoreError::LopdfError(e.to_string()))?;

        let new_stream_id = doc
            .inner_mut()
            .add_object(Stream::new(lopdf::dictionary! {}, encoded));

        // Update /Contents.
        match doc
            .inner_mut()
            .get_object_mut(page_id)
            .ok()
            .and_then(|o| o.as_dict_mut().ok())
        {
            Some(page_dict) => {
                page_dict.set("Contents", Object::Reference(new_stream_id));
            }
            None => {
                return Err(PdfCoreError::LopdfError(
                    "failed to update page /Contents".into(),
                ))
            }
        }

        // If the new font is a standard PDF font, ensure it exists in
        // /Resources/Font so the document remains viewer-renderable.
        if STANDARD_PDF_FONTS.contains(&self.new_font_name.as_str()) {
            ensure_standard_font(doc, page_id, &self.new_font_name)?;
        }

        tracing::debug!(
            page_index = self.page_index,
            old_font = %self.old_font_name,
            new_font = %self.new_font_name,
            "font substituted"
        );
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

/// Adds a minimal standard Type1 font entry to the page's `/Resources/Font`
/// dictionary if it is not already present.
///
/// Walks the `/Parent` chain to find the effective `/Resources` dict (including
/// inherited resources) and dereferences indirect `/Resources` and `/Font`
/// objects before merging in the new font entry.  The merged dict is written
/// inline on the page node so that the page-level resources shadow any
/// inherited ancestor resources without losing them.
fn ensure_standard_font(
    doc: &mut Document,
    page_id: lopdf::ObjectId,
    font_name: &str,
) -> Result<(), PdfCoreError> {
    let font_name_bytes = font_name.as_bytes().to_vec();

    // Walk up the /Parent chain to find the nearest /Resources dict.
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

            // Try to resolve /Resources at this node (direct or indirect).
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

        found.unwrap_or_else(lopdf::Dictionary::new)
    };

    // Get or create the /Font sub-dictionary, resolving indirect references.
    let mut font_dict: lopdf::Dictionary = {
        let font_obj = resources_dict.get(b"Font").ok().cloned();
        match font_obj {
            Some(Object::Reference(font_id)) => {
                doc.inner()
                    .get_object(font_id)
                    .ok()
                    .and_then(|o| o.as_dict().ok())
                    .cloned()
                    .unwrap_or_else(lopdf::Dictionary::new)
            }
            Some(Object::Dictionary(d)) => d,
            _ => lopdf::Dictionary::new(),
        }
    };

    // Only add the font entry if not already present.
    if font_dict.get(&font_name_bytes).is_err() {
        let font_entry = Object::Dictionary(lopdf::dictionary! {
            "Type"     => Object::Name(b"Font".to_vec()),
            "Subtype"  => Object::Name(b"Type1".to_vec()),
            "BaseFont" => Object::Name(font_name_bytes.clone()),
        });
        font_dict.set(font_name_bytes, font_entry);
    }

    resources_dict.set("Font", Object::Dictionary(font_dict));

    // Write the effective Resources dict inline on the page dictionary so it
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
mod font_tests {
    use super::*;
    use lopdf::{content::Content as LContent, content::Operation as LOp, dictionary,
                Document as LopdfDoc, Object, Stream};
    use pdf_core::Document;
    use tempfile::NamedTempFile;

    fn pdf_with_font(font: &str) -> NamedTempFile {
        let ops = vec![
            LOp::new("BT", vec![]),
            LOp::new(
                "Tf",
                vec![
                    Object::Name(font.as_bytes().to_vec()),
                    Object::Real(12.0),
                ],
            ),
            LOp::new("Td", vec![Object::Real(100.0), Object::Real(700.0)]),
            LOp::new("Tj", vec![Object::string_literal("test".to_owned())]),
            LOp::new("ET", vec![]),
        ];
        let bytes = LContent { operations: ops }.encode().expect("encode");

        let mut doc = LopdfDoc::with_version("1.7");
        let pages_id = doc.new_object_id();
        let page_id = doc.new_object_id();
        let content_id = doc.add_object(Stream::new(dictionary! {}, bytes));
        let page = Object::Dictionary(dictionary! {
            "Type"     => Object::Name(b"Page".to_vec()),
            "Parent"   => Object::Reference(pages_id),
            "MediaBox" => Object::Array(vec![
                Object::Integer(0), Object::Integer(0),
                Object::Integer(595), Object::Integer(842),
            ]),
            "Contents"  => Object::Reference(content_id),
            "Resources" => Object::Dictionary(lopdf::Dictionary::new()),
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

    fn page_tf_font_names(doc: &Document) -> Vec<Vec<u8>> {
        use lopdf::content::Content;
        let page_id = doc.get_page(0).unwrap().object_id;
        let inner = doc.inner();
        let content_ids: Vec<lopdf::ObjectId> =
            match inner.get_object(page_id).ok()
                .and_then(|o| o.as_dict().ok())
                .and_then(|d| d.get(b"Contents").ok())
            {
                Some(Object::Reference(id)) => vec![*id],
                Some(Object::Array(arr)) => {
                    arr.iter().filter_map(|o| o.as_reference().ok()).collect()
                }
                _ => vec![],
            };
        let mut result = Vec::new();
        for sid in content_ids {
            let bytes = inner.get_object(sid).ok()
                .and_then(|o| o.as_stream().ok())
                .map(|s| s.content.clone())
                .unwrap_or_default();
            if let Ok(parsed) = Content::decode(&bytes) {
                for op in parsed.operations {
                    if op.operator == "Tf" {
                        if let Some(Object::Name(b)) = op.operands.first() {
                            result.push(b.clone());
                        }
                    }
                }
            }
        }
        result
    }

    #[test]
    fn font_substitution_replaces_tf_operator() {
        let f = pdf_with_font("Helvetica");
        let mut doc = Document::open(f.path()).expect("open");
        let mut cmd = FontSubstitutionCommand::new(0, "Helvetica", "Times-Roman");
        cmd.execute(&mut doc).expect("execute");

        let fonts = page_tf_font_names(&doc);
        assert!(
            fonts.iter().any(|b| b == b"Times-Roman"),
            "Times-Roman should appear in Tf ops after substitution"
        );
        assert!(
            !fonts.iter().any(|b| b == b"Helvetica"),
            "Helvetica should be gone after substitution"
        );
    }

    #[test]
    fn font_substitution_adds_standard_font_to_resources() {
        let f = pdf_with_font("Helvetica");
        let mut doc = Document::open(f.path()).expect("open");
        let mut cmd = FontSubstitutionCommand::new(0, "Helvetica", "Times-Roman");
        cmd.execute(&mut doc).expect("execute");

        let page_id = doc.get_page(0).unwrap().object_id;
        let has_font = doc.inner()
            .get_object(page_id).ok()
            .and_then(|o| o.as_dict().ok())
            .and_then(|d| d.get(b"Resources").ok())
            .and_then(|o| o.as_dict().ok())
            .and_then(|d| d.get(b"Font").ok())
            .and_then(|o| o.as_dict().ok())
            .map(|font_dict| font_dict.get(b"Times-Roman").is_ok())
            .unwrap_or(false);

        assert!(has_font, "Times-Roman should be added to page /Resources/Font");
    }

    #[test]
    fn font_substitution_undo_restores_original_font() {
        let f = pdf_with_font("Helvetica");
        let mut doc = Document::open(f.path()).expect("open");
        let mut cmd = FontSubstitutionCommand::new(0, "Helvetica", "Times-Roman");
        cmd.execute(&mut doc).expect("execute");
        cmd.undo(&mut doc).expect("undo");

        let fonts = page_tf_font_names(&doc);
        assert!(
            fonts.iter().any(|b| b == b"Helvetica"),
            "original font should be restored after undo"
        );
    }

    #[test]
    fn font_substitution_out_of_range_fails() {
        let f = pdf_with_font("Helvetica");
        let mut doc = Document::open(f.path()).expect("open");
        let mut cmd = FontSubstitutionCommand::new(99, "Helvetica", "Times-Roman");
        assert!(cmd.execute(&mut doc).is_err());
    }
}
