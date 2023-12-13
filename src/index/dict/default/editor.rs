use crate::index::dict::default::DefaultDict;
use crate::index::dict::IndexDictionaryEditor;
use crate::Result;
use bytestore::backend::growable::GrowableBackend;
use bytestore::backend::Backend;
use bytestore::components::map::hashing;
use bytestore::traits::deser::Deser;
use std::cmp::Ordering;

/// Edits SimpleDicts and allows insertion of new terms.
pub struct DictEditor<'a, B, T> {
    dict: &'a mut DefaultDict<B, T>,
}

impl<'a, B, T> DictEditor<'a, B, T> {
    #[inline]
    pub(crate) fn new(dict: &'a mut DefaultDict<B, T>) -> Self {
        Self { dict }
    }
}

impl<'a, B, T> DictEditor<'a, B, T>
where
    B: Backend,
    T: Deser + Ord + hashing::Hash,
{
    /// Improves lookup time for items returning a higher ordering than other items.
    #[inline]
    pub fn optimize<C>(&mut self, mut cmp: C) -> Result<()>
    where
        C: FnMut(&T, &T) -> Ordering,
    {
        self.dict
            .map
            .rehash_with_relevance(|a, b| cmp(a.key(), b.key()))?;
        Ok(())
    }

    /// Improves lookup time for terms with higher frequency.
    #[inline]
    pub fn optimize_with_termfreqs<F>(&mut self, mut tf: F)
    where
        F: FnMut(&T) -> usize,
    {
        self.optimize(|a, b| tf(a).cmp(&tf(b))).unwrap();
    }
}

impl<'a, B, T> IndexDictionaryEditor<T> for DictEditor<'a, B, T>
where
    B: GrowableBackend,
    T: Deser + Ord + Clone + hashing::Hash,
{
    #[inline]
    fn announce_new_terms(&mut self, terms: usize, term_size: usize) -> Result<()> {
        let map = &mut self.dict.map;
        map.grow_to(map.len() + terms)?;
        map.reserve_storage(terms, term_size * terms)?;
        Ok(())
    }

    #[inline]
    fn insert_or_get_single(&mut self, term: &T) -> Result<u32> {
        if let Some(k) = self.dict.map.get(term) {
            return Ok(k);
        }
        let id = self.dict.map.len();
        Ok(self.dict.map.insert(term, &(id as u32))?)
    }
}
