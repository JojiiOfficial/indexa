mod dict;
pub mod posting;
pub mod storage;

use crate::Result;
use mapstore::backend::growable::GrowableBackend;
use mapstore::components::multi_file::MultiFile;
use mapstore::traits::creatable::Creatable;
use mapstore::traits::initiable::Initiable;
use std::marker::PhantomData;

/// An Inverted index implementation. This is implemented using the following generic sub-components:
///
/// Dict     - Maps Terms to their IDs
/// Storage  - Containing all Items to look up when searching
/// Postings - Maps Term-IDs to their Storage-IDs
///
/// Due to a high generality, this introduces a lot of Generics:
/// B: Backend (for storing the raw data)
/// T: Type of input terms (usually string or maybe fixed len string for ngrams)
/// S: Storage items. That what the index will return on searches. Maybe IDs of search results
pub struct InvertedIndex<B, T, S> {
    backend: MultiFile<B>,
    _p1: PhantomData<T>,
    _p2: PhantomData<S>,
}

impl<B, T, S> InvertedIndex<B, T, S>
where
    B: GrowableBackend,
{
    pub fn create(backend: B) -> Result<Self> {
        let backend = MultiFile::with_capacity(backend, 10)?;
        Ok(Self {
            backend,
            _p1: PhantomData,
            _p2: PhantomData,
        })
    }

    pub fn init(backend: B) -> Result<Self> {
        let backend = MultiFile::init(backend)?;
        Ok(Self {
            backend,
            _p1: PhantomData,
            _p2: PhantomData,
        })
    }

    /// Returns the underlying backend of the index.
    #[inline]
    pub(crate) fn backend(&self) -> &MultiFile<B> {
        &self.backend
    }

    /// Returns the underlying backend of the index mutable.
    #[inline]
    pub(crate) fn backend_mut(&mut self) -> &mut MultiFile<B> {
        &mut self.backend
    }
}
