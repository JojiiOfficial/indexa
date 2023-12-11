pub mod simple;

use crate::Result;
use std::borrow::Borrow;

pub trait IndexDictionary<T> {
    /// Returns the unique ID of the given term.
    fn term_id(&self, term: &T) -> Option<u32>;

    /// Returns `true` if the given term is in the dictionary.
    #[inline]
    fn has_term(&self, term: &T) -> bool {
        self.term_id(term).is_some()
    }

    /// Returns the amount of terms in the dictionary.
    #[inline]
    fn len(&self) -> usize;

    /// Returns `true` if the dictionary is empty
    #[inline]
    fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

pub trait EditIndexDictionary<T> {
    /// Should insert all non existent terms from `terms`. Returns their IDs in the same order.
    fn insert_or_get<I: IntoIterator<Item = T>>(&mut self, terms: I) -> Result<Vec<u32>>;
}
