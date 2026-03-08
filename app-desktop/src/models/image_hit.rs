#[derive(Debug, Clone)]
pub(crate) struct ImageHit {
    pub(crate) resource_name: String,
    pub(crate) stream_id: lopdf::ObjectId,
    pub(crate) do_op_index: usize,
    pub(crate) cm_op_index: usize,
    pub(crate) matrix: [f32; 6],
    pub(crate) x_min: f32,
    pub(crate) x_max: f32,
    pub(crate) y_min: f32,
    pub(crate) y_max: f32,
}
