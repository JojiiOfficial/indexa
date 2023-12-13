use super::{DICT_INDEX, POSTINGS_INDEX, STORAGE_INDEX};
use crate::edit::IndexEditor;
use crate::index::dict::default::DefaultDict;
use crate::index::dict::ngram::ngram::Ngram;
use crate::index::dict::ngram::NGramDict;
use crate::index::posting::compressed::CompressedPostings;
use crate::index::posting::default::DefaultPostings;
use crate::index::storage::default::DefaultStorage;
use crate::index::storage::passthrough::PassThroughStorage;
use crate::index::traits::editable_index::EditableInvertedIndex;
use crate::index::traits::index::{InvertedIndex, InvertedIndexDict};
use crate::{
    implement_editable_index, implement_editable_ngindex_trait, implement_index_dict_trait,
    implement_index_trait, implement_ngindex_dict_trait, implement_ngindex_trait, index_functions,
    index_mut_functions, ngram_index_functions, ngram_index_mut_functions,
};
use bytestore::backend::base::sub::GeneralSubBackend;
use bytestore::backend::growable::GrowableBackend;
use bytestore::backend::Backend;
use bytestore::components::map::hashing;
use bytestore::components::multi_file::entry_mut::MFileEntryMut;
use bytestore::components::multi_file::MultiFile;
use bytestore::traits::creatable::Creatable;
use bytestore::traits::deser::Deser;
use bytestore::traits::initiable::Initiable;
use std::marker::PhantomData;

/// A default implementation for an inverted index. Doesn't apply any compression or other mods.
pub struct DefaultIndex<B, T, S> {
    backend: MultiFile<B>,
    p: PhantomData<(T, S)>,
}

impl<B: Backend, T, S> DefaultIndex<B, T, S> {
    index_functions!(DefaultDict, DefaultStorage, DefaultPostings, T, S);
}

impl<B, T, S> DefaultIndex<B, T, S> {
    index_mut_functions!(DefaultDict, DefaultStorage, DefaultPostings, T, S);
}

impl<B, T, S> InvertedIndex<T, S> for DefaultIndex<B, T, S>
where
    B: Backend,
    T: Deser + hashing::Hash + Eq,
    S: Deser,
{
    implement_index_trait!(DefaultDict, DefaultStorage, DefaultPostings, T, S);
}

impl<B, T, S> InvertedIndexDict<T> for DefaultIndex<B, T, S>
where
    T: Deser + hashing::Hash + Eq,
    B: Backend,
{
    implement_index_dict_trait!(DefaultDict, T);
}

impl<B, T, S> EditableInvertedIndex<B, T, S> for DefaultIndex<B, T, S>
where
    B: GrowableBackend,
    T: Deser + Ord + Clone + hashing::Hash + Eq,
    S: Deser,
{
    implement_editable_index!(DefaultDict, DefaultStorage, DefaultPostings, T, S);
}

/// An inverted index that stores postings compressed.
pub struct CompressedIndex<B, T, S> {
    backend: MultiFile<B>,
    p: PhantomData<(T, S)>,
}

impl<B: Backend, T, S> CompressedIndex<B, T, S> {
    index_functions!(DefaultDict, DefaultStorage, CompressedPostings, T, S);
}

impl<B, T, S> CompressedIndex<B, T, S> {
    index_mut_functions!(DefaultDict, DefaultStorage, CompressedPostings, T, S);
}

impl<B, T, S> InvertedIndex<T, S> for CompressedIndex<B, T, S>
where
    B: Backend,
    T: Deser + hashing::Hash + Eq,
    S: Deser,
{
    implement_index_trait!(DefaultDict, DefaultStorage, CompressedPostings, T, S);
}

impl<B, T, S> InvertedIndexDict<T> for CompressedIndex<B, T, S>
where
    T: Deser + hashing::Hash + Eq,
    B: Backend,
{
    implement_index_dict_trait!(DefaultDict, T);
}

impl<B, T, S> EditableInvertedIndex<B, T, S> for CompressedIndex<B, T, S>
where
    B: GrowableBackend,
    T: Deser + Ord + Clone + hashing::Hash + Eq,
    S: Deser,
{
    implement_editable_index!(DefaultDict, DefaultStorage, CompressedPostings, T, S);
}

/// An inverted index that stores postings compressed.
pub struct CompressedIntIndex<B, T> {
    backend: MultiFile<B>,
    p: PhantomData<T>,
}

impl<B, T> CompressedIntIndex<B, T>
where
    B: Backend,
{
    index_functions!(DefaultDict, PassThroughStorage, CompressedPostings, T, u64);
}

impl<B, T> CompressedIntIndex<B, T> {
    index_mut_functions!(DefaultDict, PassThroughStorage, CompressedPostings, T, u64);
}

impl<B, T> InvertedIndex<T, u64> for CompressedIntIndex<B, T>
where
    B: Backend,
    T: Deser + hashing::Hash + Eq,
{
    implement_index_trait!(DefaultDict, PassThroughStorage, CompressedPostings, T, u64);
}

impl<B, T> InvertedIndexDict<T> for CompressedIntIndex<B, T>
where
    T: Deser + hashing::Hash + Eq,
    B: Backend,
{
    implement_index_dict_trait!(DefaultDict, T);
}

impl<B, T> EditableInvertedIndex<B, T, u64> for CompressedIntIndex<B, T>
where
    B: GrowableBackend,
    T: Deser + Ord + Clone + hashing::Hash + Eq,
{
    implement_editable_index!(DefaultDict, PassThroughStorage, CompressedPostings, T, u64);
}

//                                                        //
//                           NGram                        //
//                                                        //

/// An inverted index that is optimitzed (space and performance) for Ngram terms. Doesn't apply any compression or other mods.
pub struct DefaultNgramIndex<B, S, const N: usize> {
    backend: MultiFile<B>,
    p: PhantomData<S>,
}

impl<B, S, const N: usize> DefaultNgramIndex<B, S, N>
where
    B: Backend,
    S: Deser,
{
    ngram_index_functions!(DefaultStorage, DefaultPostings, N, u64);
}

impl<B, S, const N: usize> DefaultNgramIndex<B, S, N>
where
    S: Deser,
{
    ngram_index_mut_functions!(DefaultStorage, DefaultPostings, N, u64);
}

impl<B, S, const N: usize> InvertedIndex<Ngram<N>, u64> for DefaultNgramIndex<B, S, N>
where
    B: Backend,
    S: Deser,
{
    implement_ngindex_trait!(DefaultStorage, DefaultPostings, N, u64);
}

impl<B, S, const N: usize> InvertedIndexDict<Ngram<N>> for DefaultNgramIndex<B, S, N>
where
    S: Deser,
    B: Backend,
{
    implement_ngindex_dict_trait!(N);
}

impl<B, S, const N: usize> EditableInvertedIndex<B, Ngram<N>, u64> for DefaultNgramIndex<B, S, N>
where
    B: GrowableBackend,
    S: Deser,
{
    implement_editable_ngindex_trait!(DefaultStorage, DefaultPostings, N, u64);
}

/// A compressed inverted index that is optimitzed (space and performance) for Ngram terms.
pub struct CompressedNgramIndex<B, S, const N: usize> {
    backend: MultiFile<B>,
    p: PhantomData<S>,
}

impl<B, S, const N: usize> CompressedNgramIndex<B, S, N>
where
    B: Backend,
    S: Deser,
{
    ngram_index_functions!(DefaultStorage, CompressedPostings, N, u64);
}

impl<B, S, const N: usize> CompressedNgramIndex<B, S, N>
where
    S: Deser,
{
    ngram_index_mut_functions!(DefaultStorage, CompressedPostings, N, u64);
}

impl<B, S, const N: usize> InvertedIndex<Ngram<N>, u64> for CompressedNgramIndex<B, S, N>
where
    B: Backend,
    S: Deser,
{
    implement_ngindex_trait!(DefaultStorage, CompressedPostings, N, u64);
}

impl<B, S, const N: usize> InvertedIndexDict<Ngram<N>> for CompressedNgramIndex<B, S, N>
where
    S: Deser,
    B: Backend,
{
    implement_ngindex_dict_trait!(N);
}

impl<B, S, const N: usize> EditableInvertedIndex<B, Ngram<N>, u64> for CompressedNgramIndex<B, S, N>
where
    B: GrowableBackend,
    S: Deser,
{
    implement_editable_ngindex_trait!(DefaultStorage, CompressedPostings, N, u64);
}

/// A compressed inverted index that is optimitzed (space and performance) for Ngram as terms and ints as storage item.
pub struct CompressedIntNgramIndex<B, const N: usize> {
    backend: MultiFile<B>,
    p: PhantomData<()>,
}

impl<B, const N: usize> CompressedIntNgramIndex<B, N>
where
    B: Backend,
{
    ngram_index_functions!(PassThroughStorage, CompressedPostings, N, u64);
}

impl<B, const N: usize> CompressedIntNgramIndex<B, N> {
    ngram_index_mut_functions!(PassThroughStorage, CompressedPostings, N, u64);
}

impl<B, const N: usize> InvertedIndex<Ngram<N>, u64> for CompressedIntNgramIndex<B, N>
where
    B: Backend,
{
    implement_ngindex_trait!(PassThroughStorage, CompressedPostings, N, u64);
}

impl<B, const N: usize> InvertedIndexDict<Ngram<N>> for CompressedIntNgramIndex<B, N>
where
    B: Backend,
{
    implement_ngindex_dict_trait!(N);
}

impl<B, const N: usize> EditableInvertedIndex<B, Ngram<N>, u64> for CompressedIntNgramIndex<B, N>
where
    B: GrowableBackend,
{
    implement_editable_ngindex_trait!(PassThroughStorage, CompressedPostings, N, u64);
}
