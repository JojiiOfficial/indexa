use crate::error::Error;
use crate::index::posting::default::DefaultPostings;
use crate::index::posting::IndexPostingEditor;
use crate::Result;
use bytestore::backend::growable::GrowableBackend;
use bytestore::backend::Backend;
use bytestore::components::indexed_file::IndexedFile;
use bytestore::components::number_seq::NumberSequence;
use bytestore::traits::initiable::Initiable;
use std::collections::HashMap;

pub struct DefaultPostingEditor<'a, B> {
    postings: &'a mut DefaultPostings<B>,

    /// All pending insertions. Maps term_ids to its storage IDs.
    pending: Vec<HashMap<usize, Vec<u8>>>,
}

impl<'a, B> DefaultPostingEditor<'a, B> {
    #[inline]
    pub(super) fn new(postings: &'a mut DefaultPostings<B>) -> Self {
        Self {
            postings,
            pending: vec![],
        }
    }
}

impl<'a, B> DefaultPostingEditor<'a, B>
where
    B: GrowableBackend,
{
    pub fn commit_postings(
        &mut self,
        post_id: usize,
        postings: HashMap<usize, Vec<u8>>,
    ) -> Result<()> {
        let mut terms = postings.into_iter().collect::<Vec<_>>();
        terms.sort_unstable_by(|a, b| a.0.cmp(&b.0));

        let mut posting_list = self.postings.posting_list_mut(post_id)?;

        // Pregrow whole posting list to not need a lot of small allocations in the loop below.
        // let total_other: usize = terms.iter().map(|i| i.1.len()).sum();
        // posting_list.grow(terms.len(), total_other)?;

        let max_tid = terms.iter().max_by_key(|i| i.0).map(|i| i.0).unwrap();

        // let start = Instant::now();
        Self::ensure_term_in_posting(&mut posting_list, max_tid)?;
        // println!("Term posting ensuring took: {:?}", start.elapsed());

        // let start = Instant::now();
        posting_list.grow_multiple_fast(&terms).unwrap();
        // println!("Growing multiple took: {:?}", start.elapsed());

        Ok(())
    }

    fn ensure_term_in_posting<B2>(ifile: &mut IndexedFile<B2>, term_id: usize) -> Result<()>
    where
        B2: GrowableBackend,
    {
        let count = ifile.count();
        if term_id < count {
            return Ok(());
        }
        let need_insert = (term_id + 1) - count;
        ifile.push_n_empty(need_insert)?;
        Ok(())
    }
}

impl<'a, B> IndexPostingEditor for DefaultPostingEditor<'a, B>
where
    B: GrowableBackend,
{
    fn announce_term_count(&mut self, count: usize) -> Result<()> {
        self.pending
            .resize_with(count, || HashMap::with_capacity(1));
        Ok(())
    }

    fn insert_posts(&mut self, post_id: u16, storage_id: u64, term_ids: &[usize]) -> Result<()> {
        if term_ids.is_empty() {
            return Ok(());
        }

        let post_id = post_id as usize;
        if post_id >= self.pending.len() {
            self.pending.resize_with(post_id + 1, HashMap::default);
        }

        let storage_id_enc = storage_id.to_be_bytes();

        // We can unwrap here since we checked the availability of `post_id` a few lines before.
        let post = self.pending.get_mut(post_id).unwrap();

        for term_id in term_ids.iter() {
            let entry = post
                .entry(*term_id)
                .or_insert_with(|| Vec::with_capacity(storage_id_enc.len()));
            entry.extend_from_slice(&storage_id_enc);
        }

        Ok(())
    }

    fn sort_postings(&mut self, posting_id: usize, term_id: usize) -> Result<()> {
        if !self.pending.is_empty() {
            return Err(Error::UnsupportedOperation);
        }

        let mut posting_list = self.postings.posting_list_mut(posting_id)?;
        sort_postings_impl(&mut posting_list, term_id)
    }

    fn sort_all_postings(&mut self) -> Result<()> {
        if !self.pending.is_empty() {
            return Err(Error::UnsupportedOperation);
        }

        let posting_list_count = self.postings.posting_list_count();
        for postings_list in 0..posting_list_count {
            let mut posting_list = self.postings.posting_list_mut(postings_list)?;
            let term_count = posting_list.count();
            for term_id in 0..term_count {
                sort_postings_impl(&mut posting_list, term_id)?;
            }
        }

        Ok(())
    }

    fn commit(mut self) -> Result<()> {
        let pending = std::mem::take(&mut self.pending);
        for (post_id, new_mappings) in pending.into_iter().enumerate() {
            if !new_mappings.is_empty() {
                self.commit_postings(post_id, new_mappings).unwrap();
            }
        }
        Ok(())
    }
}

/// Sorts the storage ids of a term in a postings list.
#[inline]
fn sort_postings_impl<B: Backend>(posting_list: &mut IndexedFile<B>, term_id: usize) -> Result<()> {
    let backend = posting_list.get_backend_mut(term_id)?;
    NumberSequence::<_, u64, 8>::init(backend)?.sort_unstable();
    Ok(())
}
