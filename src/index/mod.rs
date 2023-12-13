pub mod dict;
pub mod posting;
pub mod preset;
pub mod storage;
pub mod traits;

pub(crate) const DICT_INDEX: usize = 0;
pub(crate) const STORAGE_INDEX: usize = 1;
pub(crate) const POSTINGS_INDEX: usize = 2;

#[macro_export]
macro_rules! implement_index_dict_trait {
    ($dict_impl:ident, $t:tt) => {
        type DictImpl<'a> = $dict_impl<GeneralSubBackend<'a>, $t> where
                                                                                           B: 'a,
                                                                                           $t: 'a,
                                                                                           Self: 'a,
                                                                                   ;

        #[inline]
        fn get_dict(&self) -> Self::DictImpl<'_> {
            self.dict()
        }
    };
}

#[macro_export]
macro_rules! implement_index_trait {
    ($dict_impl:ident, $storage_impl:ident, $postings_impl:ident, $t:tt, $s:tt) => {
        type StorageImpl<'a> = $storage_impl<GeneralSubBackend<'a>, $s>
                                                                                       where
                                                                                           B: 'a,
                                                                                           $s: 'a,
                                                                                           Self: 'a,
                                                                                   ;

        type PostingsImpl<'a>  = $postings_impl<GeneralSubBackend<'a>>
                                                                                       where
                                                                                           B: 'a,
                                                                                           Self: 'a,
                                                                                   ;


        #[inline]
        fn get_storage(&self) -> Self::StorageImpl<'_> {
            self.storage()
        }

        #[inline]
        fn get_postings(&self) -> Self::PostingsImpl<'_> {
            self.postings()
        }
    };
}

#[macro_export]
macro_rules! implement_editable_index {
    ($dict_impl:ident, $storage_impl:ident, $postings_impl:ident, $t:tt, $s:tt) => {
        type DictImpl<'a> = $dict_impl<MFileEntryMut<'a, B>, $t>
                                                                                       where
                                                                                           B: 'a,
                                                                                           $t: 'a,
                                                                                           Self: 'a,
                                                                                   ;

        type StorageImpl<'a> = $storage_impl<MFileEntryMut<'a, B>, $s>
                                                                                       where
                                                                                           B: 'a,
                                                                                           $s: 'a,
                                                                                           Self: 'a,
                                                                                   ;

        type PostingsImpl<'a>  = $postings_impl<MFileEntryMut<'a, B>>
                                                                                       where
                                                                                           B: 'a,
                                                                                           Self: 'a,
                                                                                   ;

        #[inline]
        fn get_dict_mut(&mut self) -> Self::DictImpl<'_> {
            self.dict_mut()
        }

        #[inline]
        fn get_storage_mut(&mut self) -> Self::StorageImpl<'_> {
            self.storage_mut()
        }

        #[inline]
        fn get_postings_mut(&mut self) -> Self::PostingsImpl<'_> {
            self.postings_mut()
        }
    };
}

#[macro_export]
macro_rules! index_functions {
    ($dict_impl:ident, $storage_impl:ident, $postings_impl:ident, $t:tt, $s:tt) => {
        #[inline]
        pub fn editor(&mut self) -> IndexEditor<Self, B, $t, $s> {
            IndexEditor::new(self)
        }

        #[inline]
        pub fn dict(&self) -> $dict_impl<GeneralSubBackend, $t> {
            self.backend.get_backend(DICT_INDEX).unwrap()
        }

        #[inline]
        pub fn storage(&self) -> $storage_impl<GeneralSubBackend, $s> {
            self.backend.get_backend(STORAGE_INDEX).unwrap()
        }

        #[inline]
        pub fn postings(&self) -> $postings_impl<GeneralSubBackend> {
            self.backend.get_backend(POSTINGS_INDEX).unwrap()
        }

        #[inline]
        pub fn flush(&mut self) -> crate::Result<()> {
            self.backend.flush()?;
            Ok(())
        }
    };
}

#[macro_export]
macro_rules! index_mut_functions {
    ($dict_impl:ident, $storage_impl:ident, $postings_impl:ident, $t:tt, $s:tt) => {
        pub fn create(backend: B) -> crate::Result<Self>
        where
            B: GrowableBackend,
        {
            let mut mf = MultiFile::create(backend)?;
            // IMPORTANT: These must be in the same order as the index constants!
            mf.insert_new_backend::<$dict_impl<_, $t>>()?;
            mf.insert_new_backend::<$storage_impl<_, $s>>()?;
            mf.insert_new_backend::<$postings_impl<_>>()?;
            Ok(Self {
                backend: mf,
                p: std::marker::PhantomData,
            })
        }

        #[inline]
        pub fn load(backend: B) -> crate::Result<Self>
        where
            B: Backend,
        {
            Ok(Self {
                backend: MultiFile::init(backend)?,
                p: std::marker::PhantomData,
            })
        }

        #[inline]
        pub fn dict_mut(&mut self) -> $dict_impl<MFileEntryMut<'_, B>, $t>
        where
            B: GrowableBackend,
        {
            let entry = self.backend.entry_mut(DICT_INDEX).unwrap();
            $dict_impl::init(entry).unwrap()
        }

        #[inline]
        pub fn storage_mut(&mut self) -> $storage_impl<MFileEntryMut<'_, B>, $s>
        where
            B: GrowableBackend,
        {
            let entry = self.backend.entry_mut(STORAGE_INDEX).unwrap();
            $storage_impl::init(entry).unwrap()
        }

        #[inline]
        pub fn postings_mut(&mut self) -> $postings_impl<MFileEntryMut<'_, B>>
        where
            B: GrowableBackend,
        {
            let entry = self.backend.entry_mut(POSTINGS_INDEX).unwrap();
            $postings_impl::init(entry).unwrap()
        }
    };
}

#[macro_export]
macro_rules! ngram_index_functions {
    ($storage_impl:ident, $postings_impl:ident, $n:tt, $s:tt) => {
        #[inline]
        pub fn editor(&mut self) -> IndexEditor<Self, B, Ngram<$n>, $s> {
            IndexEditor::new(self)
        }

        #[inline]
        pub fn dict(&self) -> NGramDict<GeneralSubBackend, $n> {
            self.backend.get_backend(DICT_INDEX).unwrap()
        }

        #[inline]
        pub fn storage(&self) -> $storage_impl<GeneralSubBackend, $s> {
            self.backend.get_backend(STORAGE_INDEX).unwrap()
        }

        #[inline]
        pub fn postings(&self) -> $postings_impl<GeneralSubBackend> {
            self.backend.get_backend(POSTINGS_INDEX).unwrap()
        }

        #[inline]
        pub fn flush(&mut self) -> crate::Result<()> {
            self.backend.flush()?;
            Ok(())
        }
    };
}

#[macro_export]
macro_rules! ngram_index_mut_functions {
    ($storage_impl:ident, $postings_impl:ident, $n:tt, $s:tt) => {
        pub fn create(backend: B) -> crate::Result<Self>
        where
            B: GrowableBackend,
        {
            let mut mf = MultiFile::create(backend)?;
            // IMPORTANT: These must be in the same order as the index constants!
            mf.insert_new_backend::<NGramDict<_, $n>>()?;
            mf.insert_new_backend::<$storage_impl<_, $s>>()?;
            mf.insert_new_backend::<$postings_impl<_>>()?;
            Ok(Self {
                backend: mf,
                p: std::marker::PhantomData,
            })
        }

        pub fn load(backend: B) -> crate::Result<Self>
        where
            B: Backend,
        {
            Ok(Self {
                backend: MultiFile::init(backend)?,
                p: std::marker::PhantomData,
            })
        }

        #[inline]
        pub fn dict_mut(&mut self) -> NGramDict<MFileEntryMut<'_, B>, $n>
        where
            B: GrowableBackend,
        {
            let entry = self.backend.entry_mut(DICT_INDEX).unwrap();
            NGramDict::init(entry).unwrap()
        }

        #[inline]
        pub fn storage_mut(&mut self) -> $storage_impl<MFileEntryMut<'_, B>, $s>
        where
            B: GrowableBackend,
        {
            let entry = self.backend.entry_mut(STORAGE_INDEX).unwrap();
            $storage_impl::init(entry).unwrap()
        }

        #[inline]
        pub fn postings_mut(&mut self) -> $postings_impl<MFileEntryMut<'_, B>>
        where
            B: GrowableBackend,
        {
            let entry = self.backend.entry_mut(POSTINGS_INDEX).unwrap();
            $postings_impl::init(entry).unwrap()
        }
    };
}

#[macro_export]
macro_rules! implement_editable_ngindex_trait {
    ($storage_impl:ident, $postings_impl:ident, $n:tt, $s:tt) => {
        type DictImpl<'a> = NGramDict<MFileEntryMut<'a, B>, $n>
                                                                                       where
                                                                                           B: 'a,
                                                                                           Self: 'a;

        type StorageImpl<'a> = $storage_impl<MFileEntryMut<'a, B>, $s>
                                                                                       where
                                                                                           B: 'a,
                                                                                           Self: 'a,
                                                                                   ;

        type PostingsImpl<'a>  = $postings_impl<MFileEntryMut<'a, B>>
                                                                                       where
                                                                                           B: 'a,
                                                                                           Self: 'a,
                                                                                   ;

        #[inline]
        fn get_dict_mut(&mut self) -> Self::DictImpl<'_> {
            self.dict_mut()
        }

        #[inline]
        fn get_storage_mut(&mut self) -> Self::StorageImpl<'_> {
            self.storage_mut()
        }

        #[inline]
        fn get_postings_mut(&mut self) -> Self::PostingsImpl<'_> {
            self.postings_mut()
        }
    };
}

#[macro_export]
macro_rules! implement_ngindex_dict_trait {
    ($n:tt) => {
        type DictImpl<'a> = NGramDict<GeneralSubBackend<'a>, $n>
                                                                                        where
                                                                                        B: 'a,
                                                                                        Self: 'a,
                                                                                        ;
        #[inline]
        fn get_dict(&self) -> Self::DictImpl<'_> {
            self.dict()
        }
    };
}

#[macro_export]
macro_rules! implement_ngindex_trait {
    ($storage_impl:ident, $postings_impl:ident, $n:tt, $s:tt) => {
        type StorageImpl<'a> = $storage_impl<GeneralSubBackend<'a>, $s>
                                                                                       where
                                                                                           B: 'a,
                                                                                           $s: 'a,
                                                                                           Self: 'a,
                                                                                   ;

        type PostingsImpl<'a>  = $postings_impl<GeneralSubBackend<'a>>
                                                                                       where
                                                                                           B: 'a,
                                                                                           Self: 'a,
                                                                                   ;




        #[inline]
        fn get_storage(&self) -> Self::StorageImpl<'_> {
            self.storage()
        }

        #[inline]
        fn get_postings(&self) -> Self::PostingsImpl<'_> {
            self.postings()
        }
    };
}

#[cfg(test)]
pub mod test {
    use super::*;
    use crate::edit::NewItem;
    use crate::index::preset::DefaultIndex;
    use bytestore::backend::memory::{MemoryBackend, MemoryData};
    use bytestore::traits::creatable::Creatable;
    use std::collections::HashMap;

    pub fn index_test_data() -> (Vec<(Vec<String>, String)>, HashMap<String, Vec<String>>) {
        let text = r#"
        Jotoba is a multilingual Japanese dictionary built upon lots of free resources from the internet.
        It provides a lot of handy tools to lookup words, kanji, example sentences, Japanese names and much more.
        On this page we want to say thank you to all those creating such wonderful resources that make something like Jotoba even possible.
        If you're interested in the development of Jotoba itself, check out our Trello Board and see what we are currently working on and what features will come in future releases!
        Joto-kun was created by a good friend of ours who is truly a wizard when it comes down to design!
        Take a quick tour to get to know about the most relevant features of Jotoba!
        These shortcuts can be used anywhere when the input is not in focus.
        Use Kanji, Kana or Romaji to search for radicals.
        Pick a radical to start searching for kanji.
        "#;

        let sentences: Vec<_> = text
            .split(['.', '!'])
            .filter(|i| !i.is_empty())
            .map(|i| i.trim().to_string())
            .collect();

        let mut out = vec![];

        let mut real_index: HashMap<String, Vec<String>> = HashMap::new();

        for sentence in sentences {
            let terms: Vec<_> = sentence
                .split(['.', ' '])
                .map(|i| i.trim().replace([',', '.', '!'], "").to_lowercase())
                .filter(|i| !i.is_empty())
                .collect();

            for i in terms.iter() {
                real_index
                    .entry(i.clone())
                    .or_default()
                    .push(sentence.clone());
            }

            out.push((terms, sentence));
        }

        (out, real_index)
    }

    pub fn make_index<B: GrowableBackend>(
        backend: B,
        data: &[(Vec<String>, String)],
        sorted: bool,
    ) -> DefaultIndex<B, String, String> {
        let mut simple_index: DefaultIndex<_, String, String> =
            DefaultIndex::create(backend).unwrap();

        let mut editor = simple_index.editor();

        if sorted {
            editor = simple_index.editor().with_sorted_postings()
        }

        for (p, (terms, storage_item)) in data.iter().enumerate() {
            let post_id = p as u16 % 2;
            let insert_item = NewItem::new(terms.clone(), storage_item.clone());
            editor.insert_in_postings(insert_item, &[post_id]).unwrap();
        }

        editor.commit().unwrap();
        editor.finish().unwrap();

        simple_index
    }

    pub fn sorted_test_index() -> DefaultIndex<MemoryBackend, String, String> {
        let backend = MemoryBackend::create(MemoryData::new(vec![0u8; 20])).unwrap();
        make_index(backend, &index_test_data().0, true)
    }
}
