pub mod editor;

use crate::error::Error;
use crate::index::posting::{EditableIndexPosting, IndexPosting};
use editor::DefaultPostingEditor;
use bytestore::backend::full::FullBackend;
use bytestore::backend::growable::GrowableBackend;
use bytestore::backend::Backend;
use bytestore::components::indexed_file::IndexedFile;
use bytestore::components::multi_file::entry_mut::MFileEntryMut;
use bytestore::components::multi_file::MultiFile;
use bytestore::components::number_seq::iter::OwnedNumberSeqIterator;
use bytestore::components::number_seq::NumberSequence;
use bytestore::traits::collection::Collection;
use bytestore::traits::creatable::Creatable;
use bytestore::traits::initiable::Initiable;

pub struct DefaultPostings<B> {
    pub(crate) postings: MultiFile<B>,
}

impl<B> Creatable<B> for DefaultPostings<B>
where
    B: GrowableBackend,
{
    #[inline]
    fn with_capacity(backend: B, capacity: usize) -> bytestore::Result<Self> {
        let postings = MultiFile::with_capacity(backend, capacity)?;
        Ok(Self { postings })
    }
}

impl<B> Initiable<B> for DefaultPostings<B>
where
    B: Backend,
{
    #[inline]
    fn init(backend: B) -> bytestore::Result<Self> {
        let postings = MultiFile::init(backend)?;
        Ok(Self { postings })
    }
}

impl<B> DefaultPostings<B>
where
    B: GrowableBackend,
{
    #[inline]
    pub(crate) fn posting_list_mut(
        &mut self,
        posting_id: usize,
    ) -> Result<IndexedFile<MFileEntryMut<B>>, Error> {
        if !self.postings.has_id(posting_id) {
            let needed = posting_id - self.postings.count();
            for _ in 0..=needed {
                self.postings
                    .insert_new_backend::<IndexedFile<_>>()
                    .unwrap();
            }
        }
        let backend = self
            .postings
            .get_backend_mut::<IndexedFile<_>>(posting_id)
            .unwrap();
        Ok(backend)
    }
}

impl<B> EditableIndexPosting for DefaultPostings<B>
where
    B: GrowableBackend,
{
    type Editor<'a> = DefaultPostingEditor<'a, B> where Self: 'a, B: 'a;

    #[inline]
    fn editor(&mut self) -> Self::Editor<'_> {
        DefaultPostingEditor::new(self)
    }
}

impl<B> DefaultPostings<B>
where
    B: Backend,
{
    /// Returns the amount of posting lists.
    #[inline]
    pub fn posting_list_count(&self) -> usize {
        self.postings.count()
    }

    #[inline]
    fn posting_backend<'a>(&self, post_id: usize, term_id: u64) -> Option<FullBackend<&'a [u8]>> {
        let ifile: IndexedFile<_> = self.postings.get_backend(post_id)?;
        let data = ifile.get_backend(term_id.try_into().unwrap()).ok()?;

        // Safety:
        // The actual lifetime of the data is bound to <B> which &self is also bound to.
        Some(unsafe { data.ignore_lifetimes() })
    }
}

impl<B> IndexPosting for DefaultPostings<B>
where
    B: Backend,
{
    type PostingRetriever<'a> = OwnedNumberSeqIterator<FullBackend<&'a [u8]>, u64, 8>
        where Self: 'a;

    #[inline]
    fn posting_retriever(
        &self,
        post_id: usize,
        term_id: u64,
    ) -> Option<Self::PostingRetriever<'_>> {
        let backend = self.posting_backend(post_id, term_id)?;
        let collection = NumberSequence::init(backend).expect("Failed to init backend");
        Some(collection.into_iter())
    }

    #[inline]
    fn len(&self) -> usize {
        self.postings.count()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::index::posting::IndexPostingEditor;
    use bytestore::traits::creatable::MemCreatable;
    use rand::distributions::Uniform;
    use rand::prelude::Distribution;
    use std::time::Instant;

    #[test]
    fn posting() {
        let mut postings = DefaultPostings::create_mem_with_capacity(10).unwrap();

        let mut editor = postings.editor();
        editor.announce_term_count(2).unwrap();
        editor.insert_posts(0, 0, &[0, 1]).unwrap();
        editor.commit().unwrap();

        let r = postings.posting_retriever(0, 1).unwrap();
        assert_eq!(r.collect::<Vec<_>>(), vec![0]);

        let r = postings.posting_retriever(0, 0).unwrap();
        assert_eq!(r.collect::<Vec<_>>(), vec![0]);

        let mut postings = DefaultPostings::create_mem_with_capacity(10).unwrap();

        let insert_count = 1_000_000u64;

        let mut rand = rand::thread_rng();
        let step = Uniform::new(0, insert_count as usize);

        for _ in 0..4 {
            let mut editor = postings.editor();
            editor.announce_term_count(insert_count as usize).unwrap();

            for i in 0..insert_count {
                let n = (i % 20) as usize + 1;
                let term_ids: Vec<_> = step.sample_iter(&mut rand).take(n).collect();
                editor.insert_posts(0, i, &term_ids).unwrap();
            }

            println!("\nStart committing");
            let start = Instant::now();
            editor.commit().unwrap();
            println!(
                "committing {insert_count} storage elements took: {:?}",
                start.elapsed(),
            );
        }
    }
}
