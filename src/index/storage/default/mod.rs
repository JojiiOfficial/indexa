mod editor;

use crate::index::storage::{EditableIndexStorage, IndexStorage};
use editor::StorageEditor;
use bytestore::backend::growable::GrowableBackend;
use bytestore::backend::Backend;
use bytestore::components::indexed_file::IndexedFile;
use bytestore::traits::creatable::Creatable;
use bytestore::traits::deser::Deser;
use bytestore::traits::initiable::Initiable;
use serde::de::DeserializeOwned;
use std::marker::PhantomData;

/// The default Storage implementation that can be used in most cases.
pub struct DefaultStorage<B, S> {
    pub backend: IndexedFile<B>,
    _p1: PhantomData<S>,
}

impl<B, S> Initiable<B> for DefaultStorage<B, S>
where
    B: Backend,
{
    #[inline]
    fn init(backend: B) -> bytestore::Result<Self> {
        let backend = IndexedFile::init(backend)?;
        Ok(Self {
            backend,
            _p1: PhantomData,
        })
    }
}

impl<B, T> Creatable<B> for DefaultStorage<B, T>
where
    B: GrowableBackend,
{
    #[inline]
    fn with_capacity(backend: B, capacity: usize) -> bytestore::Result<Self> {
        let backend = IndexedFile::with_capacity(backend, capacity)?;
        Ok(Self {
            backend,
            _p1: PhantomData,
        })
    }
}

impl<B, S> IndexStorage<S> for DefaultStorage<B, S>
where
    B: Backend,
    S: DeserializeOwned,
{
    #[inline]
    fn get_item(&self, id: usize) -> crate::Result<S> {
        Ok(self.backend.get_t(id)?)
    }

    #[inline]
    fn len(&self) -> usize {
        self.backend.count()
    }
}

impl<B, S> EditableIndexStorage<S> for DefaultStorage<B, S>
where
    B: GrowableBackend,
    S: Deser,
{
    type Editor<'a> = StorageEditor<'a, B, S> where Self: 'a, S: 'a, B: 'a;

    /// Returns an editor to modify the current storage.
    #[inline]
    fn editor(&mut self) -> Self::Editor<'_> {
        StorageEditor::new(self)
    }
}

#[cfg(test)]
mod test {
    use crate::index::storage::default::DefaultStorage;
    use crate::index::storage::{EditableIndexStorage, IndexStorage, IndexStorageEditor};
    use bytestore::traits::creatable::MemCreatable;

    #[test]
    fn test_all() {
        let mut storage = DefaultStorage::<_, usize>::create_mem_with_capacity(0).unwrap();

        let mut editor = storage.editor();
        editor
            .insert_items(&(0..1000usize).collect::<Vec<_>>())
            .unwrap();

        for i in 0..1000 {
            assert_eq!(storage.get_item(i).ok(), Some(i));
        }

        assert_eq!(storage.get_item(1000).ok(), None);
    }
}
