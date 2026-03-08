#[derive(Debug, Clone)]
pub(crate) struct InsertedTextAnchor {
    pub(crate) text: String,
    pub(crate) x: f32,
    pub(crate) y: f32,
    pub(crate) width: f32,
    pub(crate) height: f32,
    pub(crate) target_stream_id: Option<lopdf::ObjectId>,
    pub(crate) target_op_index: Option<usize>,
}
