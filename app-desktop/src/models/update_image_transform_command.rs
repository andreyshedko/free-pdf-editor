#[derive(Debug)]
pub(crate) struct UpdateImageTransformCommand {
    pub(crate) page_index: u32,
    pub(crate) stream_id: lopdf::ObjectId,
    pub(crate) cm_op_index: usize,
    pub(crate) old_matrix: [f32; 6],
    pub(crate) new_matrix: [f32; 6],
    pub(crate) snapshot: Option<Vec<u8>>,
}
