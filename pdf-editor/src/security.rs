use lopdf::{
    content::{Content, Operation},
    encryption::{EncryptionVersion, Permissions},
    Object, Stream,
};
use pdf_core::{Document, DocumentCommand, PdfCoreError};

/// Generate a 16-byte unique identifier for the PDF file `/ID` entry.
///
/// The value combines the current time's sub-second nanoseconds with a
/// monotonically increasing counter so that every call produces a different
/// result, which is the primary requirement for a PDF file identifier.
fn generate_file_id() -> Vec<u8> {
    use std::sync::atomic::{AtomicU64, Ordering};
    use std::time::{SystemTime, UNIX_EPOCH};

    static COUNTER: AtomicU64 = AtomicU64::new(1);

    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.subsec_nanos() as u64)
        .unwrap_or(0);
    let count = COUNTER.fetch_add(1, Ordering::Relaxed);

    // Combine the two 64-bit values into 16 bytes.
    let mut id = Vec::with_capacity(16);
    id.extend_from_slice(&nanos.to_le_bytes());
    id.extend_from_slice(&count.to_le_bytes());
    id
}

/// Ensure the document trailer contains a `/ID` array.  PDF encryption
/// algorithms require this field; if it is missing we generate a random one.
fn ensure_file_id(doc: &mut lopdf::Document) {
    if doc.trailer.get(b"ID").is_err() {
        let id = generate_file_id();
        doc.trailer.set(
            "ID",
            Object::Array(vec![
                Object::string_literal(id.as_slice()),
                Object::string_literal(id.as_slice()),
            ]),
        );
    }
}

#[derive(Debug)]
pub struct SetPasswordCommand {
    password: String,
    snapshot: Option<Vec<u8>>,
}

impl SetPasswordCommand {
    pub fn new(password: impl Into<String>) -> Self {
        Self {
            password: password.into(),
            snapshot: None,
        }
    }
}

impl DocumentCommand for SetPasswordCommand {
    fn description(&self) -> &str {
        "Set password"
    }

    fn execute(&mut self, doc: &mut Document) -> Result<(), PdfCoreError> {
        // Snapshot the current state so we can undo.
        let mut buf = std::io::Cursor::new(Vec::new());
        doc.inner_mut()
            .save_to(&mut buf)
            .map_err(|e| PdfCoreError::LopdfError(e.to_string()))?;
        self.snapshot = Some(buf.into_inner());

        if self.password.is_empty() {
            // Nothing to do for an empty password.
            tracing::info!("password cleared (no encryption applied for empty password)");
            return Ok(());
        }

        // The PDF encryption algorithm requires a file identifier in /ID.
        ensure_file_id(doc.inner_mut());

        // Build an RC4-128 encryption state (PDF 1.4 / V2) using the
        // provided password as both the user and owner password.
        let enc_version = EncryptionVersion::V2 {
            document: doc.inner(),
            owner_password: &self.password,
            user_password: &self.password,
            key_length: 128,
            permissions: Permissions::default(),
        };
        let enc_state = lopdf::EncryptionState::try_from(enc_version)
            .map_err(|e| PdfCoreError::InvalidArgument(e.to_string()))?;

        doc.inner_mut()
            .encrypt(&enc_state)
            .map_err(|e| PdfCoreError::LopdfError(e.to_string()))?;

        tracing::info!("document encrypted with RC4-128 password protection");
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
        Self {
            page_index,
            x,
            y,
            width,
            height,
            snapshot: None,
        }
    }
}

impl DocumentCommand for RedactRegionCommand {
    fn description(&self) -> &str {
        "Redact region"
    }

    fn execute(&mut self, doc: &mut Document) -> Result<(), PdfCoreError> {
        let mut buf = std::io::Cursor::new(Vec::new());
        doc.inner_mut()
            .save_to(&mut buf)
            .map_err(|e| PdfCoreError::LopdfError(e.to_string()))?;
        self.snapshot = Some(buf.into_inner());

        let page = doc.get_page(self.page_index)?;
        let page_id = page.object_id;

        // Attempt to permanently remove content from the page's content streams
        // before painting the visual redaction rectangle on top.
        let redacted_via_stream =
            try_true_redact(doc, page_id, self.x, self.y, self.width, self.height);

        if redacted_via_stream {
            tracing::info!(
                page_index = self.page_index,
                "region truly redacted (content removed from stream)"
            );
        } else {
            // Fall back: just append the black rectangle to the existing streams.
            // Content beneath is visually hidden but remains in the document model.
            tracing::warn!(
                page_index = self.page_index,
                "true content removal failed; falling back to visual redaction only"
            );
            append_black_rect(doc, page_id, self.x, self.y, self.width, self.height)?;
        }

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

// ---------------------------------------------------------------------------
// True redaction helpers
// ---------------------------------------------------------------------------

/// Attempt to permanently remove text content within `(x, y, w, h)` from the
/// page's content streams.  Returns `true` if the operation succeeded and
/// **replaced** the page's `/Contents` entry with a new, filtered stream.
///
/// The algorithm:
/// 1. Decompresses all streams so every content stream is readable as raw bytes.
/// 2. Collects operations from every content stream attached to the page.
/// 3. Drops any `BT … ET` block whose text drawing position falls inside the
///    target rectangle.
/// 4. Appends a filled black rectangle to visually confirm the redaction.
/// 5. Encodes the result into a single new content stream and replaces `/Contents`.
///
/// Returns `false` if any step fails (e.g., content stream could not be parsed),
/// leaving the caller to apply a visual-only fallback instead.
fn try_true_redact(
    doc: &mut Document,
    page_id: lopdf::ObjectId,
    x: f32,
    y: f32,
    width: f32,
    height: f32,
) -> bool {
    // Decompress all streams so we can read content bytes directly.
    doc.inner_mut().decompress();

    // Collect content stream IDs for this page.
    let content_ids: Vec<lopdf::ObjectId> = {
        let inner = doc.inner();
        let contents = inner
            .get_object(page_id)
            .ok()
            .and_then(|o| o.as_dict().ok())
            .and_then(|d| d.get(b"Contents").ok());

        match contents {
            Some(Object::Reference(id)) => vec![*id],
            Some(Object::Array(arr)) => arr.iter().filter_map(|o| o.as_reference().ok()).collect(),
            _ => return false,
        }
    };

    // Parse each content stream, filter text within the rect, collect ops.
    let mut filtered_ops: Vec<Operation> = Vec::new();
    for stream_id in &content_ids {
        let bytes: Option<Vec<u8>> = {
            let inner = doc.inner();
            inner
                .get_object(*stream_id)
                .ok()
                .and_then(|o| o.as_stream().ok())
                .map(|s| s.content.clone())
        };

        match bytes.and_then(|b| Content::decode(&b).ok()) {
            Some(parsed) => {
                filtered_ops.extend(filter_text_in_rect(parsed.operations, x, y, width, height));
            }
            None => {
                // If a content stream cannot be decoded, skip it rather than
                // aborting and triggering a fallback path after decompression.
                continue;
            }
        }
    }

    // Append the visual black-rectangle redaction marker.
    filtered_ops.extend([
        Operation::new("q", vec![]),
        Operation::new("g", vec![Object::Real(0.0)]),
        Operation::new(
            "re",
            vec![
                Object::Real(x),
                Object::Real(y),
                Object::Real(width),
                Object::Real(height),
            ],
        ),
        Operation::new("f", vec![]),
        Operation::new("Q", vec![]),
    ]);

    let encoded = match (Content {
        operations: filtered_ops,
    })
    .encode()
    {
        Ok(e) => e,
        Err(_) => return false,
    };

    let new_id = doc
        .inner_mut()
        .add_object(Stream::new(lopdf::dictionary! {}, encoded));

    // Replace /Contents with the single merged+filtered stream.
    match doc
        .inner_mut()
        .get_object_mut(page_id)
        .ok()
        .and_then(|o| o.as_dict_mut().ok())
    {
        Some(page_dict) => {
            page_dict.set("Contents", Object::Reference(new_id));
            true
        }
        None => false,
    }
}

/// Append a filled black rectangle as a new content stream (visual-only fallback).
fn append_black_rect(
    doc: &mut Document,
    page_id: lopdf::ObjectId,
    x: f32,
    y: f32,
    width: f32,
    height: f32,
) -> Result<(), PdfCoreError> {
    let ops = vec![
        Operation::new("q", vec![]),
        Operation::new("g", vec![Object::Real(0.0)]),
        Operation::new(
            "re",
            vec![
                Object::Real(x),
                Object::Real(y),
                Object::Real(width),
                Object::Real(height),
            ],
        ),
        Operation::new("f", vec![]),
        Operation::new("Q", vec![]),
    ];
    let encoded = Content { operations: ops }
        .encode()
        .map_err(|e| PdfCoreError::LopdfError(e.to_string()))?;

    let new_id = doc
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
            arr.push(Object::Reference(new_id));
            page_dict.set("Contents", Object::Array(arr));
        }
        Ok(Object::Reference(old_id)) => {
            let old_id = *old_id;
            page_dict.set(
                "Contents",
                Object::Array(vec![Object::Reference(old_id), Object::Reference(new_id)]),
            );
        }
        Ok(existing) => {
            // Preserve any existing direct /Contents object (e.g., a Stream) by
            // wrapping it together with the new content in an array.
            page_dict.set(
                "Contents",
                Object::Array(vec![existing.clone(), Object::Reference(new_id)]),
            );
        }
        Err(_) => {
            page_dict.set("Contents", Object::Reference(new_id));
        }
    }
    Ok(())
}

/// Remove `BT … ET` blocks whose text-drawing position lies within the
/// rectangle `(rx, ry, rw, rh)` from an operation list.
fn filter_text_in_rect(ops: Vec<Operation>, rx: f32, ry: f32, rw: f32, rh: f32) -> Vec<Operation> {
    let mut result = Vec::new();
    let mut i = 0;

    while i < ops.len() {
        if ops[i].operator == "BT" {
            let bt_start = i;
            i += 1;
            while i < ops.len() && ops[i].operator != "ET" {
                i += 1;
            }
            let et_end = i.min(ops.len().saturating_sub(1));

            let block = &ops[bt_start..=et_end];
            if !block_intersects_rect(block, rx, ry, rw, rh) {
                result.extend_from_slice(block);
            }
            // Skip past the ET.
            i = et_end + 1;
        } else {
            result.push(ops[i].clone());
            i += 1;
        }
    }
    result
}

/// Return `true` if any text-drawing operator in a `BT … ET` block has a
/// current text position that falls within `(rx, ry, rw, rh)`.
fn block_intersects_rect(block: &[Operation], rx: f32, ry: f32, rw: f32, rh: f32) -> bool {
    let mut tx = 0.0f32;
    let mut ty = 0.0f32;
    let mut line_start_x = 0.0f32;
    let mut line_start_y = 0.0f32;

    for op in block {
        match op.operator.as_str() {
            "Tm" if op.operands.len() >= 6 => {
                tx = op_to_f32(&op.operands[4]);
                ty = op_to_f32(&op.operands[5]);
                line_start_x = tx;
                line_start_y = ty;
            }
            "Td" | "TD" if op.operands.len() >= 2 => {
                line_start_x += op_to_f32(&op.operands[0]);
                line_start_y += op_to_f32(&op.operands[1]);
                tx = line_start_x;
                ty = line_start_y;
            }
            "Tj" | "TJ" | "'" | "\"" => {
                if tx >= rx && tx <= rx + rw && ty >= ry && ty <= ry + rh {
                    return true;
                }
            }
            _ => {}
        }
    }
    false
}

fn op_to_f32(obj: &Object) -> f32 {
    match obj {
        Object::Integer(i) => *i as f32,
        Object::Real(r) => *r,
        _ => 0.0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use lopdf::{content::Content, dictionary, Document as LopdfDoc, Object, Stream};
    use pdf_core::Document;
    use tempfile::NamedTempFile;

    /// Build a single-page PDF whose content stream contains one BT…ET block
    /// with a `Tj` operator at position `(tx, ty)` using `Td` for placement.
    fn pdf_with_text_at(tx: f32, ty: f32) -> NamedTempFile {
        use lopdf::content::Operation;
        let ops = vec![
            Operation::new("BT", vec![]),
            Operation::new(
                "Tf",
                vec![Object::Name(b"Helvetica".to_vec()), Object::Real(12.0)],
            ),
            Operation::new("Td", vec![Object::Real(tx), Object::Real(ty)]),
            Operation::new("Tj", vec![Object::string_literal("secret")]),
            Operation::new("ET", vec![]),
        ];
        let bytes = Content { operations: ops }.encode().expect("encode");
        build_pdf_with_content(bytes)
    }

    fn build_pdf_with_content(content_bytes: Vec<u8>) -> NamedTempFile {
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

    /// Collect all `Tj`/`TJ` operations from every content stream on a page.
    fn page_text_operators(doc: &Document, page_index: u32) -> Vec<String> {
        use lopdf::content::Content;
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

        let mut text_ops = Vec::new();
        for stream_id in content_ids {
            let bytes = inner
                .get_object(stream_id)
                .ok()
                .and_then(|o| o.as_stream().ok())
                .map(|s| s.content.clone())
                .unwrap_or_default();
            if let Ok(parsed) = Content::decode(&bytes) {
                for op in parsed.operations {
                    if op.operator == "Tj" || op.operator == "TJ" {
                        text_ops.push(op.operator.clone());
                    }
                }
            }
        }
        text_ops
    }

    /// After redacting the region that contains the text, no `Tj`/`TJ`
    /// operators should remain in the page's content streams.
    #[test]
    fn redact_removes_text_in_region() {
        let f = pdf_with_text_at(100.0, 700.0);
        let mut doc = Document::open(f.path()).expect("open");

        // Verify text operator is present before redaction.
        assert!(
            !page_text_operators(&doc, 0).is_empty(),
            "Tj should be present before redaction"
        );

        // Rect (50, 680, 300, 40) covers y=[680..720] which includes y=700.
        let mut cmd = RedactRegionCommand::new(0, 50.0, 680.0, 300.0, 40.0);
        cmd.execute(&mut doc).expect("execute");

        assert!(
            page_text_operators(&doc, 0).is_empty(),
            "Tj should be removed after true redaction"
        );
    }

    /// Text operators outside the redact rectangle must be preserved.
    #[test]
    fn redact_preserves_text_outside_region() {
        // Text at y=300, far outside rect y=[680..720].
        let f = pdf_with_text_at(100.0, 300.0);
        let mut doc = Document::open(f.path()).expect("open");

        let mut cmd = RedactRegionCommand::new(0, 50.0, 680.0, 300.0, 40.0);
        cmd.execute(&mut doc).expect("execute");

        assert!(
            !page_text_operators(&doc, 0).is_empty(),
            "Tj outside the redact region should survive"
        );
    }

    /// Undo must restore the original content streams (snapshot round-trip).
    #[test]
    fn redact_undo_restores_document() {
        let f = pdf_with_text_at(100.0, 700.0);
        let mut doc = Document::open(f.path()).expect("open");

        let ops_before = page_text_operators(&doc, 0).len();

        let mut cmd = RedactRegionCommand::new(0, 50.0, 680.0, 300.0, 40.0);
        cmd.execute(&mut doc).expect("execute");
        cmd.undo(&mut doc).expect("undo");

        let ops_after_undo = page_text_operators(&doc, 0).len();
        assert_eq!(
            ops_before, ops_after_undo,
            "undo should restore the original number of Tj operators"
        );
    }

    #[test]
    fn redact_out_of_range_page_fails() {
        let f = pdf_with_text_at(100.0, 700.0);
        let mut doc = Document::open(f.path()).expect("open");
        let mut cmd = RedactRegionCommand::new(99, 0.0, 0.0, 100.0, 100.0);
        assert!(cmd.execute(&mut doc).is_err());
    }

    /// After executing SetPasswordCommand the document should be encrypted.
    #[test]
    fn set_password_encrypts_document() {
        let f = pdf_with_text_at(100.0, 100.0);
        let mut doc = Document::open(f.path()).expect("open");

        let mut cmd = SetPasswordCommand::new("s3cr3t");
        cmd.execute(&mut doc).expect("execute");

        assert!(
            doc.inner().is_encrypted(),
            "document should be encrypted after SetPasswordCommand"
        );
    }

    /// Undo must restore the unencrypted document.
    #[test]
    fn set_password_undo_removes_encryption() {
        let f = pdf_with_text_at(100.0, 100.0);
        let mut doc = Document::open(f.path()).expect("open");

        let was_encrypted_before = doc.inner().is_encrypted();

        let mut cmd = SetPasswordCommand::new("s3cr3t");
        cmd.execute(&mut doc).expect("execute");
        cmd.undo(&mut doc).expect("undo");

        assert_eq!(
            doc.inner().is_encrypted(),
            was_encrypted_before,
            "undo should restore the pre-encryption state"
        );
    }

    /// An empty password should be a no-op (document remains unencrypted).
    #[test]
    fn set_password_empty_is_noop() {
        let f = pdf_with_text_at(100.0, 100.0);
        let mut doc = Document::open(f.path()).expect("open");

        let mut cmd = SetPasswordCommand::new("");
        cmd.execute(&mut doc).expect("execute");

        assert!(
            !doc.inner().is_encrypted(),
            "empty password should not encrypt the document"
        );
    }
}
