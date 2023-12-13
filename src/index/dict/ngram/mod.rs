pub mod editor;

#[allow(clippy::module_inception)]
pub mod ngram;

use crate::index::dict::ngram::editor::DictEditor;
use crate::index::dict::ngram::ngram::Ngram;
use crate::index::dict::EditableIndexDictionary;
use crate::index::dict::IndexDictionary;
use bytestore::backend::growable::GrowableBackend;
use bytestore::backend::Backend;
use bytestore::components::map::FMap;
use bytestore::traits::creatable::Creatable;
use bytestore::traits::initiable::Initiable;

/// An index Dictionary with a fixed string length.
pub struct NGramDict<B, const N: usize> {
    backend: FMap<B, Ngram<N>, u32>,
}

impl<B, const N: usize> IndexDictionary<Ngram<N>> for NGramDict<B, N>
where
    B: Backend,
{
    #[inline]
    fn term_id(&self, term: &Ngram<N>) -> Option<u32> {
        self.backend.get(term)
    }

    #[inline]
    fn len(&self) -> usize {
        self.backend.len()
    }
}

impl<B, const N: usize> Creatable<B> for NGramDict<B, N>
where
    B: GrowableBackend,
{
    #[inline]
    fn with_capacity(backend: B, capacity: usize) -> bytestore::Result<Self> {
        let backend = FMap::with_capacity(backend, capacity)?;
        Ok(Self { backend })
    }
}

impl<B, const N: usize> Initiable<B> for NGramDict<B, N>
where
    B: Backend,
{
    #[inline]
    fn init(backend: B) -> bytestore::Result<Self> {
        let backend = FMap::init(backend)?;
        Ok(Self { backend })
    }
}

impl<B, const N: usize> EditableIndexDictionary<Ngram<N>> for NGramDict<B, N>
where
    B: GrowableBackend,
{
    type Editor<'a> = DictEditor<'a, B, N> where Self: 'a;

    #[inline]
    fn editor(&mut self) -> Self::Editor<'_> {
        DictEditor::new(self)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::index::dict::default::DefaultDict;
    use crate::index::dict::IndexDictionaryEditor;
    use bytestore::backend::memory::{MemoryBackend, MemoryData};

    #[test]
    fn ngram_dict() {
        let mut ngram_backend = MemoryBackend::create(MemoryData::new(vec![0u8; 8])).unwrap();
        let mut default_backend = MemoryBackend::create(MemoryData::new(vec![0u8; 8])).unwrap();

        let mut ngram_dict: NGramDict<_, 3> = NGramDict::create(&mut ngram_backend).unwrap();
        let mut default_dict: DefaultDict<_, String> =
            DefaultDict::create(&mut default_backend).unwrap();

        let mut ngram_editor = ngram_dict.editor();
        let mut default_editor = default_dict.editor();
        for i in 0..1000 {
            let term = format!("{i:0>3}");

            ngram_editor
                .insert_or_get_single(&Ngram::try_from(&term).unwrap())
                .unwrap();
            default_editor.insert_or_get_single(&term).unwrap();
        }

        let ngram_len = ngram_dict.backend.backend.get(2).unwrap().len();
        let dict_len = default_dict.map.backend.get(2).unwrap().len();
        assert!(ngram_len < dict_len);
    }
}
