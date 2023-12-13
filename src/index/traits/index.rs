use crate::index::traits::helper::{DictImpl, PostingImpl, StorageImpl};
use bytestore::backend::base::sub::GeneralSubBackend;

pub trait InvertedIndex<T, S>: InvertedIndexDict<T> {
    type StorageImpl<'a>: StorageImpl<GeneralSubBackend<'a>, S>
    where
        S: 'a,
        Self: 'a;

    type PostingsImpl<'a>: PostingImpl<GeneralSubBackend<'a>>
    where
        Self: 'a;

    fn get_storage(&self) -> Self::StorageImpl<'_>;

    fn get_postings(&self) -> Self::PostingsImpl<'_>;
}

pub trait InvertedIndexDict<T> {
    type DictImpl<'a>: DictImpl<GeneralSubBackend<'a>, T>
    where
        T: 'a,
        Self: 'a;

    fn get_dict(&self) -> Self::DictImpl<'_>;
}
