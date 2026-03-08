#[derive(Debug)]
pub(crate) struct UpdateNoteContentCommand {
    pub(crate) page_index: u32,
    pub(crate) object_id: (u32, u16),
    pub(crate) old_content: String,
    pub(crate) new_content: String,
}
