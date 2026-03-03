use crate::types::{Annotation, AnnotationKind, AnnotationId, Color, Rect};
use lopdf::{Dictionary, Object, ObjectId};
use pdf_core::{Document, PdfCoreError};
use tracing::{debug, instrument};

#[instrument(skip(doc, annotation), fields(page = annotation.page_index, kind = annotation.pdf_subtype()))]
pub fn write_annotation(
    doc: &mut Document,
    annotation: &mut Annotation,
) -> Result<ObjectId, PdfCoreError> {
    let page = doc.get_page(annotation.page_index)?;
    let page_id = page.object_id;

    let r = &annotation.rect;
    let rect = Object::Array(vec![
        Object::Real(r.x),
        Object::Real(r.y),
        Object::Real(r.x + r.width),
        Object::Real(r.y + r.height),
    ]);

    let mut dict = Dictionary::new();
    dict.set("Type", Object::Name(b"Annot".to_vec()));
    dict.set("Subtype", Object::Name(annotation.pdf_subtype().as_bytes().to_vec()));
    dict.set("Rect", rect);
    dict.set("NM", Object::string_literal(annotation.id.0.clone()));

    match &annotation.kind {
        AnnotationKind::Highlight { color }
        | AnnotationKind::Underline { color }
        | AnnotationKind::Strikeout { color } => {
            dict.set("C", color_array(color));
            dict.set("CA", Object::Real(color.a));
        }
        AnnotationKind::Note { author, content } => {
            dict.set("T", Object::string_literal(author.clone()));
            dict.set("Contents", Object::string_literal(content.clone()));
            dict.set("Open", Object::Boolean(false));
        }
        AnnotationKind::Drawing { color, line_width, points } => {
            dict.set("C", color_array(color));
            dict.set("BS", Object::Dictionary({
                let mut bs = Dictionary::new();
                bs.set("W", Object::Real(*line_width));
                bs
            }));
            let ink_list: Vec<Object> = points
                .iter()
                .flat_map(|(x, y)| [Object::Real(*x), Object::Real(*y)])
                .collect();
            dict.set("InkList", Object::Array(vec![Object::Array(ink_list)]));
        }
        AnnotationKind::Stamp { label } => {
            dict.set("Name", Object::Name(label.as_bytes().to_vec()));
        }
    }

    let annot_id = doc.inner_mut().add_object(Object::Dictionary(dict));
    annotation.object_id = Some(annot_id);

    let inner = doc.inner_mut();
    let page_dict = inner
        .get_object_mut(page_id)
        .map_err(|e| PdfCoreError::LopdfError(e.to_string()))?
        .as_dict_mut()
        .map_err(|e| PdfCoreError::LopdfError(e.to_string()))?;

    match page_dict.get(b"Annots") {
        Ok(Object::Array(existing)) => {
            let mut arr = existing.clone();
            arr.push(Object::Reference(annot_id));
            page_dict.set("Annots", Object::Array(arr));
        }
        _ => {
            page_dict.set("Annots", Object::Array(vec![Object::Reference(annot_id)]));
        }
    }

    debug!(annot_id = ?annot_id, "annotation written to PDF");
    Ok(annot_id)
}

pub fn remove_annotation(
    doc: &mut Document,
    page_index: u32,
    annotation_id: &AnnotationId,
) -> Result<(), PdfCoreError> {
    let page = doc.get_page(page_index)?;
    let page_id = page.object_id;
    let inner = doc.inner_mut();

    let annots_arr = inner
        .get_object(page_id)
        .map_err(|e| PdfCoreError::LopdfError(e.to_string()))?
        .as_dict()
        .map_err(|e| PdfCoreError::LopdfError(e.to_string()))?
        .get(b"Annots")
        .ok()
        .and_then(|o| o.as_array().ok())
        .cloned()
        .unwrap_or_default();

    let mut target_id: Option<ObjectId> = None;
    for item in &annots_arr {
        if let Object::Reference(ref_id) = item {
            if let Ok(obj) = inner.get_object(*ref_id) {
                if let Ok(dict) = obj.as_dict() {
                    if let Ok(nm) = dict.get(b"NM") {
                        let nm_str = nm.as_str().ok()
                            .map(|b| String::from_utf8_lossy(b).into_owned())
                            .unwrap_or_default();
                        if nm_str == annotation_id.0 {
                            target_id = Some(*ref_id);
                            break;
                        }
                    }
                }
            }
        }
    }

    let target = target_id.ok_or_else(|| PdfCoreError::AnnotationNotFound(annotation_id.0.clone()))?;

    let new_annots: Vec<Object> = annots_arr
        .into_iter()
        .filter(|o| o.as_reference().ok() != Some(target))
        .collect();

    let page_dict = inner
        .get_object_mut(page_id)
        .map_err(|e| PdfCoreError::LopdfError(e.to_string()))?
        .as_dict_mut()
        .map_err(|e| PdfCoreError::LopdfError(e.to_string()))?;
    page_dict.set("Annots", Object::Array(new_annots));

    Ok(())
}

/// Find the lopdf `ObjectId` of the annotation whose `NM` field matches `annotation_id`.
///
/// The returned id refers to the annotation dictionary already present in the document's
/// object store. Because `remove_annotation` only drops the reference from the page's
/// `Annots` array (it never deletes the object itself), the id remains valid after
/// removal and can be used to re-attach the annotation on undo.
pub fn find_annotation_object_id(
    doc: &Document,
    page_index: u32,
    annotation_id: &AnnotationId,
) -> Result<ObjectId, PdfCoreError> {
    let page = doc.get_page(page_index)?;
    let inner = doc.inner();

    let annots_arr = inner
        .get_object(page.object_id)
        .ok()
        .and_then(|o| o.as_dict().ok())
        .and_then(|d| d.get(b"Annots").ok())
        .and_then(|o| o.as_array().ok())
        .cloned()
        .unwrap_or_default();

    for item in &annots_arr {
        if let Object::Reference(ref_id) = item {
            if let Ok(obj) = inner.get_object(*ref_id) {
                if let Ok(dict) = obj.as_dict() {
                    if let Ok(nm) = dict.get(b"NM") {
                        let nm_str = nm.as_str().ok()
                            .map(|b| String::from_utf8_lossy(b).into_owned())
                            .unwrap_or_default();
                        if nm_str == annotation_id.0 {
                            return Ok(*ref_id);
                        }
                    }
                }
            }
        }
    }
    Err(PdfCoreError::AnnotationNotFound(annotation_id.0.clone()))
}

pub fn read_annotations(doc: &Document, page_index: u32) -> Vec<Annotation> {
    let page = match doc.get_page(page_index) {
        Ok(p) => p,
        Err(_) => return Vec::new(),
    };
    let inner = doc.inner();

    let annots_arr = inner
        .get_object(page.object_id)
        .ok()
        .and_then(|o| o.as_dict().ok())
        .and_then(|d| d.get(b"Annots").ok())
        .and_then(|o| o.as_array().ok())
        .cloned()
        .unwrap_or_default();

    let mut result = Vec::new();
    for item in &annots_arr {
        if let Object::Reference(ref_id) = item {
            if let Ok(obj) = inner.get_object(*ref_id) {
                if let Ok(dict) = obj.as_dict() {
                    if let Some(annot) = parse_annotation(dict, page_index, *ref_id) {
                        result.push(annot);
                    }
                }
            }
        }
    }
    result
}

fn parse_annotation(dict: &Dictionary, page_index: u32, obj_id: ObjectId) -> Option<Annotation> {
    let subtype = dict.get(b"Subtype").ok()?
        .as_name().ok()
        .map(|b| String::from_utf8_lossy(b).into_owned())?;
    let rect_arr = dict.get(b"Rect").ok()?.as_array().ok()?;
    let nums: Vec<f32> = rect_arr.iter().filter_map(|o| match o {
        Object::Integer(i) => Some(*i as f32),
        Object::Real(r)    => Some(*r),
        _ => None,
    }).collect();
    if nums.len() < 4 { return None; }
    let rect = Rect { x: nums[0], y: nums[1], width: nums[2] - nums[0], height: nums[3] - nums[1] };

    let nm = dict.get(b"NM").ok()
        .and_then(|o| o.as_str().ok())
        .map(|b| String::from_utf8_lossy(b).into_owned())
        .unwrap_or_else(|| format!("{}-{}", obj_id.0, obj_id.1));

    let kind = match subtype.as_str() {
        "Highlight" => AnnotationKind::Highlight { color: Color::yellow() },
        "Underline" => AnnotationKind::Underline { color: Color::black() },
        "StrikeOut" => AnnotationKind::Strikeout { color: Color::red() },
        "Text" => {
            let content = dict.get(b"Contents").ok()
                .and_then(|o| o.as_str().ok())
                .map(|b| String::from_utf8_lossy(b).into_owned())
                .unwrap_or_default();
            let author = dict.get(b"T").ok()
                .and_then(|o| o.as_str().ok())
                .map(|b| String::from_utf8_lossy(b).into_owned())
                .unwrap_or_default();
            AnnotationKind::Note { author, content }
        }
        "Ink" => AnnotationKind::Drawing {
            color: Color::black(),
            line_width: 1.0,
            points: Vec::new(),
        },
        "Stamp" => {
            let label = dict.get(b"Name").ok()
                .and_then(|o| o.as_name().ok())
                .map(|b| String::from_utf8_lossy(b).into_owned())
                .unwrap_or_else(|| "Draft".to_owned());
            AnnotationKind::Stamp { label }
        }
        _ => return None,
    };

    Some(Annotation {
        id: AnnotationId(nm),
        page_index,
        rect,
        kind,
        object_id: Some(obj_id),
    })
}

fn color_array(color: &Color) -> Object {
    Object::Array(vec![
        Object::Real(color.r),
        Object::Real(color.g),
        Object::Real(color.b),
    ])
}
