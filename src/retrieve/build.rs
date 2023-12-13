use crate::index::dict::IndexDictionary;
use crate::index::posting::IndexPosting;
use crate::index::traits::index::{InvertedIndex, InvertedIndexDict};
use crate::retrieve::options::RetrieveOptions;
use crate::retrieve::retriever::RetrieveAlgo;
use std::marker::PhantomData;

pub struct RetrieverBuilder<'a, P, I> {
    index: &'a I,
    postings: P,

    options: RetrieveOptions,

    p: PhantomData<&'a ()>,
}

impl<'a, P, I> RetrieverBuilder<'a, P, I> {
    #[inline]
    pub fn new<T, S>(index: &'a I) -> Self
    where
        I: InvertedIndex<T, S, PostingsImpl<'a> = P> + 'a,
    {
        let postings = index.get_postings();
        Self {
            index,
            postings,
            options: RetrieveOptions::default(),
            p: PhantomData,
        }
    }

    pub fn add_term_id(&mut self, term: u64) {
        self.options.term_ids.push(term);
    }

    pub fn unique(mut self) -> Self {
        self.options.unique = true;
        self
    }

    pub fn with_term_ids(mut self, ids: &[u64]) -> Self {
        self.options.term_ids = ids.to_vec();
        self
    }

    pub fn in_posting_lists(mut self, post_lists: &[u16]) -> Self {
        self.options.posting_lists = post_lists.to_vec();
        self
    }
}

impl<'a, P, I> RetrieverBuilder<'a, P, I> {
    pub fn add_terms<'t, T: 't, U>(&mut self, iter: U) -> Option<()>
    where
        U: IntoIterator<Item = &'t T>,
        I: InvertedIndexDict<T>,
    {
        let dict = self.index.get_dict();
        for term in iter {
            if let Some(termid) = dict.term_id(&term) {
                self.add_term_id(termid as u64)
            }
        }
        Some(())
    }

    pub fn add_term<T>(&mut self, term: &T) -> Option<()>
    where
        I: InvertedIndexDict<T>,
    {
        let dict = self.index.get_dict();
        let tid = dict.term_id(term)? as u64;
        self.add_term_id(tid);
        Some(())
    }

    pub fn in_all_postings(&mut self)
    where
        P: IndexPosting,
    {
        let len: u16 = self
            .postings
            .len()
            .try_into()
            .expect("Too many posting lists!");
        self.options.posting_lists = (0..len).collect();
    }
}

impl<'a, P, I> RetrieverBuilder<'a, P, I>
where
    P: 'a,
{
    #[inline]
    pub fn retriever<A>(&'a self) -> A
    where
        A: RetrieveAlgo<'a, P>,
    {
        A::new(&self.postings, self.options.clone())
    }
}
