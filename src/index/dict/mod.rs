pub mod default;
pub mod ngram;

use crate::Result;

pub trait IndexDictionary<T> {
    /// Returns the unique ID of the given term.
    fn term_id(&self, term: &T) -> Option<u32>;

    /// Returns `true` if the given term is in the dictionary.
    #[inline]
    fn has_term(&self, term: &T) -> bool {
        self.term_id(term).is_some()
    }

    /// Returns the amount of terms in the dictionary.
    fn len(&self) -> usize;

    /// Returns `true` if the dictionary is empty
    fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

pub trait EditableIndexDictionary<T>: IndexDictionary<T> {
    type Editor<'a>: IndexDictionaryEditor<T> + 'a
    where
        Self: 'a;

    /// Returns an editor to edit the dictionary.
    fn editor(&mut self) -> Self::Editor<'_>;
}

pub trait IndexDictionaryEditor<T> {
    fn announce_new_terms(&mut self, _terms: usize, _term_size: usize) -> Result<()> {
        Ok(())
    }

    /// Should insert all non existent terms from `terms`. Returns the ID of the term.
    fn insert_or_get_single(&mut self, terms: &T) -> Result<u32>;

    /// Should insert all non existent terms from `terms`. Returns their IDs in the same order.
    fn insert_or_get(&mut self, terms: &[T]) -> Result<Vec<u32>> {
        let mut ids = Vec::with_capacity(terms.len());
        self.announce_new_terms(terms.len(), 500)?;
        for term in terms {
            let id = self.insert_or_get_single(term)?;
            ids.push(id);
        }
        Ok(ids)
    }
}
