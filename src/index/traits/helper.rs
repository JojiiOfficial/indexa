use crate::index::dict::{EditableIndexDictionary, IndexDictionary};
use crate::index::posting::{EditableIndexPosting, IndexPosting};
use crate::index::storage::{EditableIndexStorage, IndexStorage};
use bytestore::traits::initiable::Initiable;

// Dict
pub trait DictImpl<B, T>: IndexDictionary<T> + Initiable<B> {}

impl<B, T, U> DictImpl<B, T> for U where U: IndexDictionary<T> + Initiable<B> {}

// Storage
pub trait StorageImpl<B, S>: IndexStorage<S> + Initiable<B> {}

impl<B, S, U> StorageImpl<B, S> for U where U: IndexStorage<S> + Initiable<B> {}

// Postings
pub trait PostingImpl<B>: IndexPosting + Initiable<B> {}

impl<B, U> PostingImpl<B> for U where U: IndexPosting + Initiable<B> {}

//
// ------------------------------------ Editable ------------------------------------
// Dict
pub trait EditableDictImpl<B, T>:
    IndexDictionary<T> + Initiable<B> + EditableIndexDictionary<T>
{
}

impl<B, T, U> EditableDictImpl<B, T> for U where
    U: IndexDictionary<T> + Initiable<B> + EditableIndexDictionary<T>
{
}

// Storage
pub trait EditableStorageImpl<B, S>: Initiable<B> + EditableIndexStorage<S> {}

impl<B, S, U> EditableStorageImpl<B, S> for U where U: Initiable<B> + EditableIndexStorage<S> {}

// Postings
pub trait EditablePostingImpl<B>: IndexPosting + Initiable<B> + EditableIndexPosting {}

impl<B, U> EditablePostingImpl<B> for U where U: IndexPosting + Initiable<B> + EditableIndexPosting {}
