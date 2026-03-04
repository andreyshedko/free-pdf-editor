use lopdf::{
    content::{Content, Operation},
    dictionary,
    Object, Stream,
};
use pdf_core::{Document, DocumentCommand, PdfCoreError};
use std::sync::atomic::{AtomicU32, Ordering};

/// Inserts a raw RGB image into a page as a proper PDF Image XObject.
///
/// `data` must be raw, uncompressed pixels in 24-bit RGB order (3 bytes per
/// pixel, top-to-bottom, left-to-right).  The image is placed at PDF point
/// coordinates `(x, y)` (lower-left corner) and drawn with the given
/// `display_width` × `display_height` in PDF user units (points).
#[derive(Debug)]
pub struct InsertImageCommand {
    page_index: u32,
    data: Vec<u8>,
    img_width: u32,
    img_height: u32,
    x: f32,
    y: f32,
    display_width: f32,
    display_height: f32,
    resource_name: String,
    snapshot: Option<Vec<u8>>,
}

impl InsertImageCommand {
    pub fn new(
        page_index: u32,
        data: Vec<u8>,
        img_width: u32,
        img_height: u32,
        x: f32,
        y: f32,
        display_width: f32,
        display_height: f32,
    ) -> Self {
        static COUNTER: AtomicU32 = AtomicU32::new(1);
        let n = COUNTER.fetch_add(1, Ordering::Relaxed);
        Self {
            page_index,
            data,
            img_width,
            img_height,
            x,
            y,
            display_width,
            display_height,
            resource_name: format!("Im{n}"),
            snapshot: None,
        }
    }
}

fn snapshot_doc(doc: &mut Document) -> Result<Vec<u8>, PdfCoreError> {
    let mut buf = std::io::Cursor::new(Vec::new());
    doc.inner_mut()
        .save_to(&mut buf)
        .map_err(|e| PdfCoreError::LopdfError(e.to_string()))?;
    Ok(buf.into_inner())
}

impl DocumentCommand for InsertImageCommand {
    fn description(&self) -> &str {
        "Insert image"
    }

    fn execute(&mut self, doc: &mut Document) -> Result<(), PdfCoreError> {
        let expected = self.img_width as usize * self.img_height as usize * 3;
        if self.data.len() != expected {
            return Err(PdfCoreError::InvalidArgument(format!(
                "expected {} bytes for {}×{} RGB image, got {}",
                expected,
                self.img_width,
                self.img_height,
                self.data.len()
            )));
        }

        self.snapshot = Some(snapshot_doc(doc)?);

        let page_id = doc.get_page(self.page_index)?.object_id;

        // 1. Create the Image XObject (raw, uncompressed DeviceRGB).
        let img_dict = lopdf::dictionary! {
            "Type"             => Object::Name(b"XObject".to_vec()),
            "Subtype"          => Object::Name(b"Image".to_vec()),
            "Width"            => Object::Integer(self.img_width as i64),
            "Height"           => Object::Integer(self.img_height as i64),
            "ColorSpace"       => Object::Name(b"DeviceRGB".to_vec()),
            "BitsPerComponent" => Object::Integer(8),
        };
        let img_id = doc.inner_mut().add_object(Stream::new(img_dict, self.data.clone()));

        // 2. Build a content stream that draws the image.
        //
        //   q
        //   <dw> 0 0 <dh> <x> <y>  cm   ← place & scale the image
        //   /<name> Do                   ← paint it
        //   Q
        let name_bytes = self.resource_name.as_bytes().to_vec();
        let ops = vec![
            Operation::new("q", vec![]),
            Operation::new(
                "cm",
                vec![
                    Object::Real(self.display_width),
                    Object::Real(0.0),
                    Object::Real(0.0),
                    Object::Real(self.display_height),
                    Object::Real(self.x),
                    Object::Real(self.y),
                ],
            ),
            Operation::new("Do", vec![Object::Name(name_bytes.clone())]),
            Operation::new("Q", vec![]),
        ];
        let encoded = Content { operations: ops }
            .encode()
            .map_err(|e| PdfCoreError::LopdfError(e.to_string()))?;
        let content_id =
            doc.inner_mut().add_object(Stream::new(lopdf::dictionary! {}, encoded));

        // 3. Clone the page's existing Resources dict (if any).
        let mut resources_dict: lopdf::Dictionary = {
            let inner = doc.inner();
            let resources_ref: Option<lopdf::ObjectId> = inner
                .get_object(page_id)
                .ok()
                .and_then(|o| o.as_dict().ok())
                .and_then(|d| d.get(b"Resources").ok())
                .and_then(|o| o.as_reference().ok());

            if let Some(res_id) = resources_ref {
                // Resources is an indirect object — follow the reference.
                inner
                    .get_object(res_id)
                    .ok()
                    .and_then(|o| o.as_dict().ok())
                    .cloned()
                    .unwrap_or_else(lopdf::Dictionary::new)
            } else {
                // Resources is inline or absent.
                inner
                    .get_object(page_id)
                    .ok()
                    .and_then(|o| o.as_dict().ok())
                    .and_then(|d| d.get(b"Resources").ok())
                    .and_then(|o| o.as_dict().ok())
                    .cloned()
                    .unwrap_or_else(lopdf::Dictionary::new)
            }
        };

        // 4. Get or create the XObject sub-dictionary.
        let xobject_ref: Option<lopdf::ObjectId> =
            resources_dict.get(b"XObject").ok().and_then(|o| o.as_reference().ok());

        let mut xobject_dict: lopdf::Dictionary = if let Some(xo_id) = xobject_ref {
            doc.inner()
                .get_object(xo_id)
                .ok()
                .and_then(|o| o.as_dict().ok())
                .cloned()
                .unwrap_or_else(lopdf::Dictionary::new)
        } else {
            resources_dict
                .get(b"XObject")
                .ok()
                .and_then(|o| o.as_dict().ok())
                .cloned()
                .unwrap_or_else(lopdf::Dictionary::new)
        };

        // 5. Register the image under its resource name.
        xobject_dict.set(name_bytes, Object::Reference(img_id));
        resources_dict.set("XObject", Object::Dictionary(xobject_dict));

        // 6. Write the updated Resources and the new content stream back.
        let inner = doc.inner_mut();
        let page_dict = inner
            .get_object_mut(page_id)
            .map_err(|e| PdfCoreError::LopdfError(e.to_string()))?
            .as_dict_mut()
            .map_err(|e| PdfCoreError::LopdfError(e.to_string()))?;

        page_dict.set("Resources", Object::Dictionary(resources_dict));

        match page_dict.get(b"Contents") {
            Ok(Object::Array(existing)) => {
                let mut arr = existing.clone();
                arr.push(Object::Reference(content_id));
                page_dict.set("Contents", Object::Array(arr));
            }
            Ok(Object::Reference(old_id)) => {
                let old_id = *old_id;
                page_dict.set(
                    "Contents",
                    Object::Array(vec![
                        Object::Reference(old_id),
                        Object::Reference(content_id),
                    ]),
                );
            }
            _ => {
                page_dict.set("Contents", Object::Reference(content_id));
            }
        }

        tracing::debug!(
            page_index = self.page_index,
            img_width = self.img_width,
            img_height = self.img_height,
            "image inserted"
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

    fn rgb_pixels(w: u32, h: u32) -> Vec<u8> {
        vec![128u8; (w * h * 3) as usize]
    }

    #[test]
    fn insert_image_execute_and_undo() {
        let f = single_page_pdf();
        let mut doc = Document::open(f.path()).expect("open");
        let data = rgb_pixels(4, 4);
        let mut cmd = InsertImageCommand::new(0, data, 4, 4, 100.0, 600.0, 72.0, 72.0);
        cmd.execute(&mut doc).expect("execute");
        assert_eq!(doc.page_count(), 1, "page count should be unchanged");
        // Verify XObject was registered in page resources
        let page_id = doc.get_page(0).unwrap().object_id;
        let has_xobject = doc
            .inner()
            .get_object(page_id)
            .ok()
            .and_then(|o| o.as_dict().ok())
            .and_then(|d| d.get(b"Resources").ok())
            .and_then(|o| o.as_dict().ok())
            .and_then(|d| d.get(b"XObject").ok())
            .is_some();
        assert!(has_xobject, "page should have an XObject resource after insert");
        cmd.undo(&mut doc).expect("undo");
        assert_eq!(doc.page_count(), 1);
    }

    #[test]
    fn insert_image_wrong_data_size_fails() {
        let f = single_page_pdf();
        let mut doc = Document::open(f.path()).expect("open");
        // Pass 1 byte instead of 4*4*3=48
        let mut cmd = InsertImageCommand::new(0, vec![0u8], 4, 4, 100.0, 600.0, 72.0, 72.0);
        assert!(cmd.execute(&mut doc).is_err());
    }

    #[test]
    fn insert_image_out_of_range_fails() {
        let f = single_page_pdf();
        let mut doc = Document::open(f.path()).expect("open");
        let data = rgb_pixels(2, 2);
        let mut cmd = InsertImageCommand::new(99, data, 2, 2, 0.0, 0.0, 50.0, 50.0);
        assert!(cmd.execute(&mut doc).is_err());
    }

    #[test]
    fn insert_image_multiple_images_get_unique_resource_names() {
        let f = single_page_pdf();
        let mut doc = Document::open(f.path()).expect("open");
        let data = rgb_pixels(2, 2);
        let mut cmd1 = InsertImageCommand::new(0, data.clone(), 2, 2, 10.0, 10.0, 50.0, 50.0);
        let mut cmd2 = InsertImageCommand::new(0, data, 2, 2, 70.0, 10.0, 50.0, 50.0);
        cmd1.execute(&mut doc).expect("first insert");
        cmd2.execute(&mut doc).expect("second insert");
        assert_ne!(cmd1.resource_name, cmd2.resource_name);
    }
}
