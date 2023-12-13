use crate::index::traits::helper::{EditableDictImpl, EditablePostingImpl, EditableStorageImpl};
use bytestore::components::multi_file::entry_mut::MFileEntryMut;

pub trait EditableInvertedIndex<B, T, S> {
    type DictImpl<'a>: EditableDictImpl<MFileEntryMut<'a, B>, T>
    where
        B: 'a,
        T: 'a,
        Self: 'a;

    type StorageImpl<'a>: EditableStorageImpl<MFileEntryMut<'a, B>, S>
    where
        B: 'a,
        S: 'a,
        Self: 'a;

    type PostingsImpl<'a>: EditablePostingImpl<MFileEntryMut<'a, B>>
    where
        B: 'a,
        Self: 'a;

    fn get_dict_mut(&mut self) -> Self::DictImpl<'_>;

    fn get_storage_mut(&mut self) -> Self::StorageImpl<'_>;

    fn get_postings_mut(&mut self) -> Self::PostingsImpl<'_>;
}
