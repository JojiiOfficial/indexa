mod editor;

use crate::index::dict::{EditableIndexDictionary, IndexDictionary};
use editor::DictEditor;
use bytestore::backend::growable::GrowableBackend;
use bytestore::backend::Backend;
use bytestore::components::map::{hashing, FMap};
use bytestore::traits::creatable::Creatable;
use bytestore::traits::deser::Deser;
use bytestore::traits::initiable::Initiable;
use std::marker::PhantomData;

/// The default Dictionary implementation that can be used in most cases.
pub struct DefaultDict<B, T> {
    pub map: FMap<B, T, u32>,
    _p1: PhantomData<T>,
}

impl<B, T> Initiable<B> for DefaultDict<B, T>
where
    B: Backend,
{
    fn init(backend: B) -> bytestore::Result<Self> {
        let map = FMap::init(backend)?;
        Ok(Self {
            map,
            _p1: PhantomData,
        })
    }
}

impl<B, T> Creatable<B> for DefaultDict<B, T>
where
    B: GrowableBackend,
{
    fn with_capacity(backend: B, capacity: usize) -> bytestore::Result<Self> {
        let map = FMap::with_capacity(backend, capacity)?;
        Ok(Self {
            map,
            _p1: PhantomData,
        })
    }
}

impl<B, T> IndexDictionary<T> for DefaultDict<B, T>
where
    B: Backend,
    T: hashing::Hash + Eq + Deser,
{
    #[inline]
    fn term_id(&self, term: &T) -> Option<u32> {
        self.map.get(term)
    }

    #[inline]
    fn len(&self) -> usize {
        self.map.len()
    }
}

impl<B, T> EditableIndexDictionary<T> for DefaultDict<B, T>
where
    B: GrowableBackend,
    T: hashing::Hash + Eq + Deser + Clone + Ord,
{
    type Editor<'a> = DictEditor<'a, B, T> where T: 'a, B: 'a, Self: 'a;

    /// Returns an editor for the dictionary.
    #[inline]
    fn editor(&mut self) -> Self::Editor<'_> {
        DictEditor::new(self)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::index::dict::IndexDictionaryEditor;
    use bytestore::traits::creatable::MemCreatable;
    use std::time::Instant;

    #[test]
    #[allow(dead_code)]
    fn simple_dict() {
        let mut dict: DefaultDict<_, String> = DefaultDict::create_mem_with_capacity(10).unwrap();

        let mut editor = dict.editor();

        let terms: Vec<_> = (0..10_000).map(|i| format!("{i}")).collect();
        let start = Instant::now();
        let ids = editor.insert_or_get(&terms).unwrap();
        println!("inserted {} terms in {:?}", terms.len(), start.elapsed());

        for (term, id) in terms.iter().zip(ids.iter()) {
            assert_eq!(dict.term_id(term), Some(*id));
        }

        let start = Instant::now();
        dict.map.flush().unwrap();
        println!("flushed {} terms in {:?}", terms.len(), start.elapsed());
    }
}
