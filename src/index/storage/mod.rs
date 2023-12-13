pub mod default;
pub mod passthrough;

use crate::Result;

/// Trait defining behavior of storages in an index.
pub trait IndexStorage<S> {
    /// Returns an item from the storage by its id.
    fn get_item(&self, id: usize) -> Result<S>;

    /// Should return the amount of items in the storage.
    fn len(&self) -> usize;

    /// Returns `true` if the storage has an item with the given ID.
    #[inline]
    fn has_item(&self, id: usize) -> bool {
        self.get_item(id).is_ok()
    }

    /// Returns `true` if the storage is empty.
    #[inline]
    fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

pub trait EditableIndexStorage<S>: IndexStorage<S> {
    type Editor<'a>: IndexStorageEditor<S>
    where
        Self: 'a;

    fn editor(&mut self) -> Self::Editor<'_>;
}

/// Edit an indexes storage.
pub trait IndexStorageEditor<S> {
    fn insert_items(&mut self, items: &[S]) -> Result<StorageInsertionResult>;
}

pub enum StorageInsertionResult {
    Ids(Vec<u64>),
    First(u64),
}
