#[derive(Debug, Clone)]
pub(crate) struct TextHit {
    pub(crate) text: String,
    pub(crate) stream_id: lopdf::ObjectId,
    pub(crate) op_index: usize,
    pub(crate) editable: bool,
    pub(crate) x: f32,
    pub(crate) y: f32,
    pub(crate) width: f32,
    pub(crate) height: f32,
}
