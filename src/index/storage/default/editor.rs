use crate::error::Error;
use crate::index::storage::default::DefaultStorage;
use crate::index::storage::{IndexStorage, IndexStorageEditor, StorageInsertionResult};
use crate::Result;
use bytestore::backend::growable::GrowableBackend;
use bytestore::traits::deser::Deser;

pub struct StorageEditor<'a, B, S> {
    storage: &'a mut DefaultStorage<B, S>,
}

impl<'a, B, S> StorageEditor<'a, B, S> {
    #[inline]
    pub(super) fn new(storage: &'a mut DefaultStorage<B, S>) -> Self {
        Self { storage }
    }
}

impl<'a, B, S> IndexStorageEditor<S> for StorageEditor<'a, B, S>
where
    B: GrowableBackend,
    S: Deser,
{
    #[inline]
    fn insert_items(&mut self, items: &[S]) -> Result<StorageInsertionResult> {
        if items.is_empty() {
            return Err(Error::UnsupportedOperation);
        }

        let first_id = self.storage.len() as u64;
        self.storage.backend.extend(items);
        Ok(StorageInsertionResult::First(first_id))
    }
}
