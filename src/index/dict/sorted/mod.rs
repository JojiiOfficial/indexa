pub mod editor;
pub mod item;

use crate::index::dict::sorted::editor::DictEditor;
use crate::index::dict::sorted::item::DictItem;
use crate::index::dict::{EditIndexDictionary, IndexDictionary};
use crate::traits::deser::Deser;
use bytestore::backend::growable::GrowableBackend;
use bytestore::backend::Backend;
use bytestore::components::indexed_file::IndexedFile;
use bytestore::traits::creatable::Creatable;
use bytestore::traits::initiable::Initiable;
use std::cmp::Ordering;
use std::marker::PhantomData;

/// A general sorted index dictionary implementation.
pub struct SortedDict<B, T> {
    backend: IndexedFile<B>,
    _p1: PhantomData<T>,
}

impl<B, T> Creatable<B> for SortedDict<B, T>
where
    B: GrowableBackend,
{
    fn with_capacity(backend: B, capacity: usize) -> bytestore::Result<Self> {
        Ok(Self {
            backend: IndexedFile::with_capacity(backend, capacity)?,
            _p1: PhantomData,
        })
    }
}

impl<B, T> Initiable<B> for SortedDict<B, T>
where
    B: Backend,
{
    #[inline]
    fn init(backend: B) -> bytestore::Result<Self> {
        Ok(Self {
            backend: IndexedFile::init(backend)?,
            _p1: PhantomData,
        })
    }
}

impl<B, T> SortedDict<B, T>
where
    B: GrowableBackend,
{
    /// Creates a new `DictEditor` that can be used to insert new terms into this dictionary. This
    /// can and should be reused as it shares its allocation across insertion calls.
    #[inline]
    pub fn dict_editor(&mut self) -> DictEditor<B, T> {
        DictEditor::new(self)
    }
}

impl<B, T> SortedDict<B, T>
where
    B: Backend,
    T: Deser,
{
    /// Returns an Dictionary Item at the given position (not its ID!)
    #[inline]
    fn dict_item_at(&self, pos: usize) -> Option<DictItem<T>> {
        let data = self.backend.get(pos).ok()?;
        bitcode::deserialize(data).ok()
    }

    fn binary_search_by<F>(&self, mut f: F) -> Result<usize, usize>
    where
        F: FnMut(&T) -> Ordering,
    {
        let mut size = self.backend.count();
        let mut left = 0;
        let mut right = size;
        while left < right {
            let mid = left + size / 2;

            let cmp = f(self.dict_item_at(mid).unwrap().item());

            if cmp == Ordering::Less {
                left = mid + 1;
            } else if cmp == Ordering::Greater {
                right = mid;
            } else {
                return Ok(mid);
            }

            size = right - left;
        }

        Err(left)
    }
}

impl<B, T> SortedDict<B, T>
where
    B: Backend,
    T: Deser + Ord,
{
    #[inline]
    fn binary_search(&self, item: &T) -> Result<usize, usize> {
        self.binary_search_by(|i| i.cmp(item))
    }
}

impl<B, T> IndexDictionary<T> for SortedDict<B, T>
where
    B: Backend,
    T: Deser + Ord + Eq,
{
    fn term_id(&self, term: &T) -> Option<u32> {
        let item_pos = self.binary_search_by(|i| i.cmp(term)).ok()?;
        let ditem = self.dict_item_at(item_pos).unwrap();
        Some(ditem.id())
    }

    #[inline]
    fn len(&self) -> usize {
        self.backend.count()
    }
}
