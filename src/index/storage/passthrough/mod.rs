mod editor;

use crate::index::storage::passthrough::editor::PassThroughEditor;
use crate::index::storage::{EditableIndexStorage, IndexStorage};
use bytestore::traits::creatable::Creatable;
use bytestore::traits::initiable::Initiable;
use std::marker::PhantomData;

/// A dummy storage storing nothing but returning all requested ids as existing storage id. This can be used where it
/// is not important to check whether a given storage item exists and the actual items are stored externally.
pub struct PassThroughStorage<B, S> {
    p: PhantomData<(B, S)>,
}

impl<B, S> PassThroughStorage<B, S> {
    #[inline]
    pub fn new() -> Self {
        Self { p: PhantomData }
    }
}

impl<B, S> IndexStorage<S> for PassThroughStorage<B, S>
where
    S: From<u64>,
{
    #[inline]
    fn get_item(&self, id: usize) -> crate::Result<S> {
        Ok((id as u64).into())
    }

    #[inline]
    fn len(&self) -> usize {
        0
    }
}

impl<B, S> Creatable<B> for PassThroughStorage<B, S> {
    #[inline]
    fn with_capacity(_: B, _: usize) -> bytestore::Result<Self> {
        Ok(Self::new())
    }
}

impl<B, S> Initiable<B> for PassThroughStorage<B, S> {
    #[inline]
    fn init(_: B) -> bytestore::Result<Self> {
        Ok(Self::new())
    }
}

impl<B, S> EditableIndexStorage<S> for PassThroughStorage<B, S>
where
    S: From<u64> + Clone,
    u64: From<S>,
{
    type Editor<'a> = PassThroughEditor<S> where Self: 'a, S: 'a;

    #[inline]
    fn editor(&mut self) -> Self::Editor<'_> {
        PassThroughEditor::new()
    }
}
