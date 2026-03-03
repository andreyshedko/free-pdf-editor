use crate::error::PdfCoreError;
use lopdf::{Document as LopdfDoc, Object};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::path::{Path, PathBuf};
use tracing::{instrument, info, debug};

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
        Self { x: 0.0, y: 0.0, width: 595.28, height: 841.89 }
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
        let inner = LopdfDoc::load(path)
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
        Ok(Self { id, title, path: path.to_path_buf(), inner })
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
        Ok(Page { index, object_id: obj_id, media_box })
    }

    pub fn extract_text(&self, page_index: u32) -> Result<String, PdfCoreError> {
        let page_num = page_index + 1;
        let text = self.inner.extract_text(&[page_num])
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
        self.inner.save(path)
            .map_err(|e| PdfCoreError::Save(format!("{}: {}", path.display(), e)))?;
        self.path = path.to_path_buf();
        info!(path = %path.display(), "document saved");
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
            return Err(PdfCoreError::InvalidArgument(
                format!("rotation angle must be a multiple of 90, got {angle}"),
            ));
        }
        let count = self.page_count();
        if index >= count {
            return Err(PdfCoreError::PageOutOfRange(index));
        }
        let page = self.get_page(index)?;
        let page_id = page.object_id;
        let page_dict = self.inner
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
            return Err(PdfCoreError::InvalidArgument(
                format!("new_order length {} != page_count {}", new_order.len(), count),
            ));
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
            let catalog = self.inner.catalog()
                .map_err(|e| PdfCoreError::LopdfError(e.to_string()))?;
            catalog.get(b"Pages")
                .map_err(|e| PdfCoreError::LopdfError(e.to_string()))?
                .as_reference()
                .map_err(|e| PdfCoreError::LopdfError(e.to_string()))?
        };
        let pages_dict = self.inner
            .get_object_mut(pages_id)
            .map_err(|e| PdfCoreError::LopdfError(e.to_string()))?
            .as_dict_mut()
            .map_err(|e| PdfCoreError::LopdfError(e.to_string()))?;
        pages_dict.set("Kids", Object::Array(new_kids));
        Ok(())
    }

    pub fn merge_document(&mut self, other: &mut Document) -> Result<(), PdfCoreError> {
        let added_pages = other.page_count();
        // Determine the max object id in self to offset other's IDs
        let max_id = self.inner.max_id;
        // Copy all objects from other, offsetting their IDs
        let other_objects: Vec<(ObjectId, Object)> = other.inner.objects
            .iter()
            .map(|(&id, obj)| {
                let new_id = (id.0 + max_id, id.1);
                (new_id, obj.clone())
            })
            .collect();
        for (id, obj) in other_objects {
            self.inner.objects.insert(id, obj);
        }
        // Update max_id
        self.inner.max_id += other.inner.max_id;
        // Get other's page IDs (before offset was applied to get_pages)
        let other_page_ids: Vec<ObjectId> = other.inner.get_pages()
            .values()
            .map(|&id| (id.0 + max_id, id.1))
            .collect();
        // Get self's Pages node id
        let pages_id = {
            let catalog = self.inner.catalog()
                .map_err(|e| PdfCoreError::LopdfError(e.to_string()))?;
            catalog.get(b"Pages")
                .map_err(|e| PdfCoreError::LopdfError(e.to_string()))?
                .as_reference()
                .map_err(|e| PdfCoreError::LopdfError(e.to_string()))?
        };
        // Extend Kids array
        let pages_dict = self.inner
            .get_object_mut(pages_id)
            .map_err(|e| PdfCoreError::LopdfError(e.to_string()))?
            .as_dict_mut()
            .map_err(|e| PdfCoreError::LopdfError(e.to_string()))?;
        let mut kids = pages_dict.get(b"Kids")
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
        let arr = dict.get(b"MediaBox")
            .ok()
            .and_then(|o| o.as_array().ok())?;
        if arr.len() < 4 {
            return None;
        }
        let nums: Vec<f64> = arr.iter().filter_map(|o| match o {
            Object::Integer(i) => Some(*i as f64),
            Object::Real(r) => Some(*r as f64),
            _ => None,
        }).collect();
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
