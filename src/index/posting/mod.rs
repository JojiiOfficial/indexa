pub mod compressed;
pub mod default;

use crate::Result;

pub trait IndexPosting {
    type PostingRetriever<'a>: IntoIterator<Item = u64> + 'a
    where
        Self: 'a;

    fn posting_retriever(&self, post_id: usize, term_id: u64)
        -> Option<Self::PostingRetriever<'_>>;

    fn len(&self) -> usize;

    #[inline]
    fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

pub trait EditableIndexPosting {
    type Editor<'a>: IndexPostingEditor
    where
        Self: 'a;

    fn editor(&mut self) -> Self::Editor<'_>;
}

/// Trait to add new postings into a postings map.
pub trait IndexPostingEditor {
    fn announce_term_count(&mut self, _count: usize) -> Result<()> {
        Ok(())
    }

    fn insert_posts(&mut self, post_id: u16, storage_id: u64, term_ids: &[usize]) -> Result<()>;

    fn sort_postings(&mut self, posting_id: usize, term_id: usize) -> Result<()>;

    fn sort_all_postings(&mut self) -> Result<()>;

    fn commit(self) -> Result<()>;
}
