#[derive(Clone)]
pub struct RetrieveOptions {
    // Input data for retrieving
    pub(crate) posting_lists: Vec<u16>,
    pub(crate) term_ids: Vec<u64>,

    // Options
    pub(crate) unique: bool,
    pub(crate) limit: usize,
}

impl RetrieveOptions {}

impl Default for RetrieveOptions {
    #[inline]
    fn default() -> Self {
        Self {
            posting_lists: vec![0],
            term_ids: vec![],
            unique: false,
            limit: 0,
        }
    }
}
