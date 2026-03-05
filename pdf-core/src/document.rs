use crate::error::PdfCoreError;
use lopdf::{Document as LopdfDoc, IncrementalDocument, Object};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::path::{Path, PathBuf};
use std::sync::Arc;
use tracing::{debug, info, instrument};

pub use lopdf::ObjectId;

#[derive(Debug, Clone, PartialEq)]
pub struct MediaBox {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
}

impl Default for MediaBox {
    fn default() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            width: 595.28,
            height: 841.89,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Page {
    pub index: u32,
    pub object_id: ObjectId,
    pub media_box: MediaBox,
}

pub struct Document {
    pub id: u64,
    pub title: String,
    pub path: PathBuf,
    inner: LopdfDoc,
    /// Raw bytes of the file as it was originally loaded. Stored behind an
    /// [`Arc`] so that cloning the reference during incremental saves is O(1)
    /// and avoids doubling peak memory usage for large PDFs.
    original_bytes: Option<Arc<[u8]>>,
}

impl std::fmt::Debug for Document {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Document")
            .field("id", &self.id)
            .field("title", &self.title)
            .field("path", &self.path)
            .field("page_count", &self.page_count())
            .finish()
    }
}

impl Document {
    #[instrument(name = "document_open", fields(path = %path.as_ref().display()))]
    pub fn open(path: impl AsRef<Path>) -> Result<Self, PdfCoreError> {
        let path = path.as_ref();
        let original_bytes = std::fs::read(path)
            .map_err(|e| PdfCoreError::Open(format!("{}: {}", path.display(), e)))?;
        let inner = LopdfDoc::load_mem(&original_bytes)
            .map_err(|e| PdfCoreError::Open(format!("{}: {}", path.display(), e)))?;

        let mut h = DefaultHasher::new();
        path.hash(&mut h);
        let id = h.finish();

        let title = path
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("document")
            .to_owned();

        info!(id, %title, page_count = inner.get_pages().len(), "document opened");

        Ok(Self {
            id,
            title,
            path: path.to_path_buf(),
            inner,
            original_bytes: Some(Arc::from(original_bytes.as_slice())),
        })
    }

    pub fn create_new(path: impl AsRef<Path>) -> Result<Self, PdfCoreError> {
        let path = path.as_ref();
        let inner = LopdfDoc::with_version("1.7");
        let mut h = DefaultHasher::new();
        path.hash(&mut h);
        let id = h.finish();
        let title = path
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("untitled")
            .to_owned();
        Ok(Self {
            id,
            title,
            path: path.to_path_buf(),
            inner,
            original_bytes: None,
        })
    }

    pub fn page_count(&self) -> u32 {
        self.inner.get_pages().len() as u32
    }

    pub fn pages(&self) -> Vec<Page> {
        self.inner
            .get_pages()
            .iter()
            .enumerate()
            .map(|(i, (_page_num, &obj_id))| {
                let media_box = self.get_media_box(obj_id).unwrap_or_default();
                Page {
                    index: i as u32,
                    object_id: obj_id,
                    media_box,
                }
            })
            .collect()
    }

    pub fn get_page(&self, index: u32) -> Result<Page, PdfCoreError> {
        let pages = self.inner.get_pages();
        if index as usize >= pages.len() {
            return Err(PdfCoreError::PageOutOfRange(index));
        }
        let obj_id = pages
            .values()
            .nth(index as usize)
            .copied()
            .ok_or(PdfCoreError::PageOutOfRange(index))?;
        let media_box = self.get_media_box(obj_id).unwrap_or_default();
        Ok(Page {
            index,
            object_id: obj_id,
            media_box,
        })
    }

    pub fn extract_text(&self, page_index: u32) -> Result<String, PdfCoreError> {
        let page_num = page_index + 1;
        let text = self
            .inner
            .extract_text(&[page_num])
            .map_err(|e| PdfCoreError::LopdfError(e.to_string()))?;
        Ok(text)
    }

    #[instrument(name = "document_save", skip(self), fields(path = %self.path.display()))]
    pub fn save(&mut self) -> Result<(), PdfCoreError> {
        let path = self.path.clone();
        self.save_to(&path)
    }

    #[instrument(name = "document_save_as", skip(self), fields(path = %path.as_ref().display()))]
    pub fn save_to(&mut self, path: impl AsRef<Path>) -> Result<(), PdfCoreError> {
        let path = path.as_ref();
        self.inner
            .save(path)
            .map_err(|e| PdfCoreError::Save(format!("{}: {}", path.display(), e)))?;
        self.path = path.to_path_buf();
        info!(path = %path.display(), "document saved");
        Ok(())
    }

    /// Save the document as an incremental update appended to the original file
    /// bytes.  When the document was opened from disk the original raw bytes are
    /// retained; the new revision is appended, making the resulting PDF smaller
    /// than a full rewrite and preserving the original revision for auditing.
    ///
    /// If the document was created fresh (not loaded from disk) this falls back
    /// to a regular [`save`][Self::save].
    #[instrument(name = "document_save_incremental", skip(self), fields(path = %self.path.display()))]
    pub fn save_incremental(&mut self) -> Result<(), PdfCoreError> {
        let path = self.path.clone();
        self.save_incremental_to(&path)
    }

    /// Like [`save_incremental`][Self::save_incremental] but writes to `path`.
    #[instrument(name = "document_save_incremental_as", skip(self), fields(path = %path.as_ref().display()))]
    pub fn save_incremental_to(&mut self, path: impl AsRef<Path>) -> Result<(), PdfCoreError> {
        let path = path.as_ref();

        let Some(original_bytes) = self.original_bytes.as_ref() else {
            // No original bytes available (new document) – fall back to a
            // regular full save.
            return self.save_to(path);
        };

        // Clone the Arc pointer (O(1), no data copy) so we can pass a
        // Vec<u8> to create_from without holding two full copies of the
        // file bytes simultaneously.
        let bytes_arc = Arc::clone(original_bytes);

        // Load the original document to use as the base for the incremental update.
        let prev_doc =
            LopdfDoc::load_mem(&bytes_arc).map_err(|e| PdfCoreError::LopdfError(e.to_string()))?;

        let mut incr = IncrementalDocument::create_from(bytes_arc.to_vec(), prev_doc);

        // Only write changed or newly created objects into the incremental
        // section. Objects that are identical to those in the original
        // document are left out to keep the incremental update small.
        for (&id, obj) in &self.inner.objects {
            match incr.get_prev_documents().objects.get(&id) {
                // If the object exists in the original document and is
                // byte-for-byte equal, we can skip re‑writing it.
                Some(original_obj) if original_obj == obj => {}
                // New or changed object: add it to the incremental section.
                _ => {
                    incr.new_document.set_object(id, obj.clone());
                }
            }
        }
        // Ensure max_id is at least as large as any object we inserted.
        if self.inner.max_id > incr.new_document.max_id {
            incr.new_document.max_id = self.inner.max_id;
        }

        incr.save(path)
            .map_err(|e| PdfCoreError::Save(format!("{}: {}", path.display(), e)))?;

        // Update original_bytes to reflect the latest on-disk revision so that
        // subsequent incremental saves chain correctly.
        let new_bytes = std::fs::read(path)
            .map_err(|e| PdfCoreError::Save(format!("{}: {}", path.display(), e)))?;
        self.original_bytes = Some(Arc::from(new_bytes.as_slice()));
        self.path = path.to_path_buf();
        info!(path = %path.display(), "document saved incrementally");
        Ok(())
    }

    pub fn delete_page(&mut self, index: u32) -> Result<(), PdfCoreError> {
        let count = self.page_count();
        if index >= count {
            return Err(PdfCoreError::PageOutOfRange(index));
        }
        let page_num = index + 1;
        self.inner.delete_pages(&[page_num]);
        debug!(index, "page deleted");
        Ok(())
    }

    pub fn rotate_page(&mut self, index: u32, angle: i64) -> Result<(), PdfCoreError> {
        if angle % 90 != 0 {
            return Err(PdfCoreError::InvalidArgument(format!(
                "rotation angle must be a multiple of 90, got {angle}"
            )));
        }
        let count = self.page_count();
        if index >= count {
            return Err(PdfCoreError::PageOutOfRange(index));
        }
        let page = self.get_page(index)?;
        let page_id = page.object_id;
        let page_dict = self
            .inner
            .get_object_mut(page_id)
            .map_err(|e| PdfCoreError::LopdfError(e.to_string()))?
            .as_dict_mut()
            .map_err(|e| PdfCoreError::LopdfError(e.to_string()))?;
        page_dict.set("Rotate", Object::Integer(angle));
        debug!(index, angle, "page rotated");
        Ok(())
    }

    pub fn reorder_pages(&mut self, new_order: &[u32]) -> Result<(), PdfCoreError> {
        let count = self.page_count() as usize;
        if new_order.len() != count {
            return Err(PdfCoreError::InvalidArgument(format!(
                "new_order length {} != page_count {}",
                new_order.len(),
                count
            )));
        }
        // Collect current page object IDs in order
        let pages_map = self.inner.get_pages();
        let page_ids: Vec<ObjectId> = pages_map.values().copied().collect();
        // Build the new Kids array in the requested order
        let new_kids: Vec<Object> = new_order
            .iter()
            .map(|&i| {
                let id = page_ids
                    .get(i as usize)
                    .copied()
                    .ok_or(PdfCoreError::PageOutOfRange(i))?;
                Ok(Object::Reference(id))
            })
            .collect::<Result<Vec<_>, PdfCoreError>>()?;
        // Find the Pages node (parent) from catalog
        let pages_id = {
            let catalog = self
                .inner
                .catalog()
                .map_err(|e| PdfCoreError::LopdfError(e.to_string()))?;
            catalog
                .get(b"Pages")
                .map_err(|e| PdfCoreError::LopdfError(e.to_string()))?
                .as_reference()
                .map_err(|e| PdfCoreError::LopdfError(e.to_string()))?
        };
        let pages_dict = self
            .inner
            .get_object_mut(pages_id)
            .map_err(|e| PdfCoreError::LopdfError(e.to_string()))?
            .as_dict_mut()
            .map_err(|e| PdfCoreError::LopdfError(e.to_string()))?;
        pages_dict.set("Kids", Object::Array(new_kids));
        Ok(())
    }

    pub fn merge_document(&mut self, other: &mut Document) -> Result<(), PdfCoreError> {
        let added_pages = other.page_count();
        // Determine the offset: all IDs from other will be shifted by this amount
        let id_offset = self.inner.max_id;

        // Remap a single ObjectId from other's namespace to self's namespace
        let remap_id = |id: ObjectId| -> ObjectId { (id.0 + id_offset, id.1) };

        // Recursively remap all Reference objects within a cloned object tree
        fn remap_object(obj: Object, offset: u32) -> Object {
            match obj {
                Object::Reference((n, g)) => Object::Reference((n + offset, g)),
                Object::Array(arr) => {
                    Object::Array(arr.into_iter().map(|o| remap_object(o, offset)).collect())
                }
                Object::Dictionary(mut dict) => {
                    for val in dict.iter_mut() {
                        let remapped = remap_object(std::mem::replace(val.1, Object::Null), offset);
                        *val.1 = remapped;
                    }
                    Object::Dictionary(dict)
                }
                Object::Stream(mut s) => {
                    for val in s.dict.iter_mut() {
                        let remapped = remap_object(std::mem::replace(val.1, Object::Null), offset);
                        *val.1 = remapped;
                    }
                    Object::Stream(s)
                }
                other => other,
            }
        }

        // Copy all objects from other with remapped IDs and internal references
        let other_objects: Vec<(ObjectId, Object)> = other
            .inner
            .objects
            .iter()
            .map(|(&id, obj)| (remap_id(id), remap_object(obj.clone(), id_offset)))
            .collect();
        for (id, obj) in other_objects {
            self.inner.objects.insert(id, obj);
        }

        // Update max_id
        self.inner.max_id += other.inner.max_id;

        // Get other's page IDs in self's namespace
        let other_page_ids: Vec<ObjectId> = other
            .inner
            .get_pages()
            .values()
            .map(|&id| remap_id(id))
            .collect();

        // Get self's Pages node id
        let pages_id = {
            let catalog = self
                .inner
                .catalog()
                .map_err(|e| PdfCoreError::LopdfError(e.to_string()))?;
            catalog
                .get(b"Pages")
                .map_err(|e| PdfCoreError::LopdfError(e.to_string()))?
                .as_reference()
                .map_err(|e| PdfCoreError::LopdfError(e.to_string()))?
        };

        // Extend the Pages Kids array
        let pages_dict = self
            .inner
            .get_object_mut(pages_id)
            .map_err(|e| PdfCoreError::LopdfError(e.to_string()))?
            .as_dict_mut()
            .map_err(|e| PdfCoreError::LopdfError(e.to_string()))?;
        let mut kids = pages_dict
            .get(b"Kids")
            .ok()
            .and_then(|o| o.as_array().ok())
            .cloned()
            .unwrap_or_default();
        for id in &other_page_ids {
            kids.push(Object::Reference(*id));
        }
        let new_count = kids.len() as i64;
        pages_dict.set("Kids", Object::Array(kids));
        pages_dict.set("Count", Object::Integer(new_count));
        info!(added_pages, "documents merged");
        Ok(())
    }

    pub fn inner(&self) -> &LopdfDoc {
        &self.inner
    }

    pub fn inner_mut(&mut self) -> &mut LopdfDoc {
        &mut self.inner
    }

    fn get_media_box(&self, obj_id: ObjectId) -> Option<MediaBox> {
        let obj = self.inner.get_object(obj_id).ok()?;
        let dict = obj.as_dict().ok()?;
        let arr = dict.get(b"MediaBox").ok().and_then(|o| o.as_array().ok())?;
        if arr.len() < 4 {
            return None;
        }
        let nums: Vec<f64> = arr
            .iter()
            .filter_map(|o| match o {
                Object::Integer(i) => Some(*i as f64),
                Object::Real(r) => Some(*r as f64),
                _ => None,
            })
            .collect();
        if nums.len() < 4 {
            return None;
        }
        Some(MediaBox {
            x: nums[0],
            y: nums[1],
            width: nums[2] - nums[0],
            height: nums[3] - nums[1],
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use lopdf::{dictionary, Document as LopdfDoc, Object, Stream};
    use tempfile::NamedTempFile;

    /// Build a minimal valid single-page PDF using lopdf and save to a temp file.
    fn minimal_pdf() -> NamedTempFile {
        let mut doc = LopdfDoc::with_version("1.7");

        let pages_id = doc.new_object_id();
        let page_id = doc.new_object_id();

        let content_stream = Stream::new(
            dictionary! {},
            b"BT /F1 12 Tf 100 700 Td (Hello, world!) Tj ET".to_vec(),
        );
        let content_id = doc.add_object(content_stream);

        let page = lopdf::Object::Dictionary(dictionary! {
            "Type"      => Object::Name(b"Page".to_vec()),
            "Parent"    => Object::Reference(pages_id),
            "MediaBox"  => Object::Array(vec![
                Object::Integer(0), Object::Integer(0),
                Object::Integer(595), Object::Integer(842),
            ]),
            "Contents"  => Object::Reference(content_id),
        });
        doc.objects.insert(page_id, page);

        let pages = lopdf::Object::Dictionary(dictionary! {
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

        let mut f = NamedTempFile::new().expect("temp file");
        doc.save_to(f.as_file_mut()).expect("save");
        f
    }

    #[test]
    fn open_returns_correct_page_count() {
        let f = minimal_pdf();
        let doc = Document::open(f.path()).expect("open");
        assert_eq!(doc.page_count(), 1);
    }

    #[test]
    fn open_nonexistent_returns_error() {
        let result = Document::open("/no/such/file.pdf");
        assert!(matches!(result, Err(PdfCoreError::Open(_))));
    }

    #[test]
    fn get_page_out_of_range_returns_error() {
        let f = minimal_pdf();
        let doc = Document::open(f.path()).expect("open");
        assert!(matches!(
            doc.get_page(99),
            Err(PdfCoreError::PageOutOfRange(99))
        ));
    }

    #[test]
    fn get_page_returns_media_box() {
        let f = minimal_pdf();
        let doc = Document::open(f.path()).expect("open");
        let page = doc.get_page(0).expect("get page");
        assert!(page.media_box.width > 0.0);
        assert!(page.media_box.height > 0.0);
    }

    #[test]
    fn delete_page_reduces_count() {
        let f = minimal_pdf();
        let mut doc = Document::open(f.path()).expect("open");
        assert_eq!(doc.page_count(), 1);
        doc.delete_page(0).expect("delete");
        assert_eq!(doc.page_count(), 0);
    }

    #[test]
    fn delete_page_out_of_range_returns_error() {
        let f = minimal_pdf();
        let mut doc = Document::open(f.path()).expect("open");
        assert!(matches!(
            doc.delete_page(5),
            Err(PdfCoreError::PageOutOfRange(5))
        ));
    }

    #[test]
    fn save_to_roundtrip() {
        let f = minimal_pdf();
        let mut doc = Document::open(f.path()).expect("open");
        let out = tempfile::NamedTempFile::new().expect("out temp");
        doc.save_to(out.path()).expect("save");
        let doc2 = Document::open(out.path()).expect("re-open");
        assert_eq!(doc2.page_count(), 1);
    }

    #[test]
    fn save_incremental_roundtrip() {
        let f = minimal_pdf();
        let mut doc = Document::open(f.path()).expect("open");
        let out = tempfile::NamedTempFile::new().expect("out temp");
        doc.save_incremental_to(out.path())
            .expect("save incremental");
        // The resulting file must be loadable and preserve the page count.
        let doc2 = Document::open(out.path()).expect("re-open");
        assert_eq!(doc2.page_count(), 1);
    }

    #[test]
    fn save_incremental_file_is_larger_than_original() {
        // An incremental save appends a new revision, so the output is at
        // least as large as the original raw bytes.
        let f = minimal_pdf();
        let original_size = std::fs::metadata(f.path()).unwrap().len();

        let mut doc = Document::open(f.path()).expect("open");
        let out = tempfile::NamedTempFile::new().expect("out temp");
        doc.save_incremental_to(out.path())
            .expect("save incremental");

        let incremental_size = std::fs::metadata(out.path()).unwrap().len();
        assert!(
            incremental_size >= original_size,
            "incremental file ({incremental_size} bytes) should be at least as large as the original ({original_size} bytes)"
        );
    }

    #[test]
    fn save_incremental_new_document_falls_back_to_regular_save() {
        // Documents created with create_new() have no original bytes, so
        // save_incremental() must fall back to a regular save.
        let tmp = tempfile::NamedTempFile::new().expect("temp");
        let mut doc = Document::create_new(tmp.path()).expect("create");
        // Should not panic or error even without original bytes.
        doc.save_incremental_to(tmp.path()).expect("fallback save");
    }
}
