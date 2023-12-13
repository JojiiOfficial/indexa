use std::mem::take;

/// Index insertion item.
#[derive(Clone, Debug)]
pub struct NewItem<T, S> {
    terms: Vec<T>,
    store_item: S,
}

impl<T, S> NewItem<T, S> {
    #[inline]
    pub fn new(terms: Vec<T>, store_item: S) -> Self {
        Self { terms, store_item }
    }

    #[inline]
    pub fn terms(&self) -> &[T] {
        &self.terms
    }

    #[inline]
    pub fn store_item(&self) -> &S {
        &self.store_item
    }

    #[inline]
    pub(crate) fn take_terms(&mut self) -> Vec<T> {
        take(&mut self.terms)
    }

    #[inline]
    pub(crate) fn into_store_item(self) -> S {
        self.store_item
    }
}
