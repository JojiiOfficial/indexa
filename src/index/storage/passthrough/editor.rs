use crate::error::Error;
use crate::index::storage::IndexStorageEditor;
use crate::index::storage::StorageInsertionResult;
use crate::Result;
use std::marker::PhantomData;

pub struct PassThroughEditor<T> {
    p: PhantomData<T>,
}

impl<T> PassThroughEditor<T> {
    #[inline]
    pub(super) fn new() -> Self {
        Self { p: PhantomData }
    }
}

impl<T> IndexStorageEditor<T> for PassThroughEditor<T>
where
    T: Clone,
    u64: From<T>,
{
    #[inline]
    fn insert_items(&mut self, items: &[T]) -> Result<StorageInsertionResult> {
        if items.is_empty() {
            return Err(Error::UnsupportedOperation);
        }
        let ids = items.iter().map(|i| i.clone().into()).collect();
        Ok(StorageInsertionResult::Ids(ids))
    }
}
