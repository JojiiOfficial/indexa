use super::NGramDict;
use super::Ngram;
use crate::index::dict::IndexDictionaryEditor;
use crate::Result;
use bytestore::backend::growable::GrowableBackend;

pub struct DictEditor<'a, B, const N: usize> {
    ngram_dict: &'a mut NGramDict<B, N>,
}

impl<'a, B, const N: usize> DictEditor<'a, B, N> {
    #[inline]
    pub(super) fn new(ngram_dict: &'a mut NGramDict<B, N>) -> Self {
        Self { ngram_dict }
    }
}

impl<'a, B, const N: usize> IndexDictionaryEditor<Ngram<N>> for DictEditor<'a, B, N>
where
    B: GrowableBackend,
{
    #[inline]
    fn announce_new_terms(&mut self, terms: usize, term_size: usize) -> Result<()> {
        let map = &mut self.ngram_dict.backend;
        map.grow_to(map.len() + terms)?;
        map.reserve_storage(terms, term_size * terms)?;
        Ok(())
    }

    #[inline]
    fn insert_or_get_single(&mut self, term: &Ngram<N>) -> Result<u32> {
        let id = self.ngram_dict.backend.len();
        Ok(self.ngram_dict.backend.insert(term, &(id as u32))?)
    }
}
