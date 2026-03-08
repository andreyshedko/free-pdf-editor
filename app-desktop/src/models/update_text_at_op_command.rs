#[derive(Debug)]
pub(crate) struct UpdateTextAtOpCommand {
    pub(crate) page_index: u32,
    pub(crate) stream_id: lopdf::ObjectId,
    pub(crate) op_index: usize,
    pub(crate) old_text: String,
    pub(crate) new_text: String,
    pub(crate) snapshot: Option<Vec<u8>>,
}
