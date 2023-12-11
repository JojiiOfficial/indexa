mod editor;
mod item;

use crate::index::dict::simple::editor::DictEditor;
use crate::index::dict::simple::item::DictItem;
use crate::index::dict::{EditIndexDictionary, IndexDictionary};
use crate::traits::deser::Deser;
use mapstore::backend::growable::GrowableBackend;
use mapstore::backend::Backend;
use mapstore::components::indexed_file::IndexedFile;
use mapstore::traits::creatable::Creatable;
use mapstore::traits::initiable::Initiable;
use serde::Serialize;
use std::cmp::Ordering;
use std::marker::PhantomData;

/// A general simple index dictionary implementation that fits most use cases.
pub struct SimpleDict<B, T> {
    backend: IndexedFile<B>,
    _p1: PhantomData<T>,
}

impl<B, T> Creatable<B> for SimpleDict<B, T>
where
    B: GrowableBackend,
{
    fn with_capacity(backend: B, capacity: usize) -> mapstore::Result<Self> {
        Ok(Self {
            backend: IndexedFile::with_capacity(backend, capacity)?,
            _p1: PhantomData,
        })
    }
}

impl<B, T> Initiable<B> for SimpleDict<B, T>
where
    B: Backend,
{
    #[inline]
    fn init(backend: B) -> mapstore::Result<Self> {
        Ok(Self {
            backend: IndexedFile::init(backend)?,
            _p1: PhantomData,
        })
    }
}

impl<B, T> SimpleDict<B, T>
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

impl<B, T> SimpleDict<B, T>
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

impl<B, T> SimpleDict<B, T>
where
    B: Backend,
    T: Deser + Ord,
{
    #[inline]
    fn binary_search(&self, item: &T) -> Result<usize, usize> {
        self.binary_search_by(|i| i.cmp(item))
    }
}

impl<B, T> IndexDictionary<T> for SimpleDict<B, T>
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

#[cfg(test)]
mod test {
    use super::*;
    use mapstore::backend::mmap::{MmapBackend, MmapFile};
    use mapstore::traits::creatable::MemCreatable;
    use std::collections::HashSet;

    #[test]
    fn simple_dict() {
        let backend =
            MmapBackend::create(MmapFile::create("./test", 1024 * 1024).unwrap()).unwrap();

        // let mut dict: SimpleDict<_, String> = SimpleDict::create_mem_with_capacity(10).unwrap();
        let mut dict: SimpleDict<_, String> = SimpleDict::create(backend).unwrap();

        let mut editor = dict.dict_editor();
        let ids = editor
            .insert_or_get(["".to_string(), "".to_string(), "".to_string()])
            .unwrap();
        assert_eq!(ids, vec![0, 0, 0]);

        let terms: Vec<_> = (0..100_000).map(|i| format!("{i}")).collect();
        editor.insert_or_get(terms).unwrap();

        let ids = editor
            .insert_or_get([
                "music".to_string(),
                "footprint".to_string(),
                "weird".to_string(),
            ])
            .unwrap();
        assert!(ids.iter().all(|i| *i > 0));
        assert_eq!(ids.iter().copied().collect::<HashSet<_>>().len(), ids.len());

        assert_eq!(dict.term_id(&"music".to_string()), Some(ids[0]));
        assert_eq!(dict.term_id(&"footprint".to_string()), Some(ids[1]));
        assert_eq!(dict.term_id(&"weird".to_string()), Some(ids[2]));
        assert_eq!(dict.term_id(&"not existing".to_string()), None);
    }
}
