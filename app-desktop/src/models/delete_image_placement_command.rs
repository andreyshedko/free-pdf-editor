#[derive(Debug)]
pub(crate) struct DeleteImagePlacementCommand {
    pub(crate) page_index: u32,
    pub(crate) stream_id: lopdf::ObjectId,
    pub(crate) do_op_index: usize,
    pub(crate) snapshot: Option<Vec<u8>>,
}
