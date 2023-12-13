use crate::index::posting::IndexPosting;
use crate::retrieve::options::RetrieveOptions;
use crate::retrieve::retriever::RetrieveAlgo;
use std::collections::HashSet;

/// A simple retriever that can be used for unsorted postings or any other simple retrieving. It returns _all_ storage IDs
/// which at least contained _one_ of the terms.
/// If all posting lists are sorted, [`DefaultSortedRetriever`] is much faster and memory efficient, especially for unique
/// retrieving!
pub struct DefaultRetriever<'a, P>
where
    P: IndexPosting + 'a,
{
    postings: &'a P,
    options: RetrieveOptions,

    iter: Option<<P::PostingRetriever<'a> as IntoIterator>::IntoIter>,

    curr_posting: usize,
    seen: HashSet<u64>,
}

impl<'a, P> RetrieveAlgo<'a, P> for DefaultRetriever<'a, P>
where
    P: IndexPosting + 'a,
{
    #[inline]
    fn new(postings: &'a P, options: RetrieveOptions) -> Self {
        Self {
            postings,
            options,
            iter: None,
            curr_posting: 0,
            seen: HashSet::new(),
        }
    }
}

impl<'a, P> DefaultRetriever<'a, P>
where
    P: IndexPosting + 'a,
{
    #[inline]
    fn curr_term_id(&self) -> Option<u64> {
        self.options.term_ids.last().copied()
    }
}

impl<'a, P> Iterator for DefaultRetriever<'a, P>
where
    P: IndexPosting + 'a,
    Self: 'a,
{
    type Item = u64;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(v) = self
                .iter
                .as_mut()
                .and_then(|i| i.find(|j| !self.options.unique || !self.seen.contains(j)))
            {
                return Some(v);
            }

            if self.curr_posting >= self.options.posting_lists.len() {
                self.curr_posting = 0;
                self.options.term_ids.pop();
            }

            let curr_term = self.curr_term_id()?;

            let iter = self
                .postings
                .posting_retriever(self.curr_posting, curr_term);

            self.curr_posting += 1;

            if let Some(iter) = iter {
                let iter = iter.into_iter();
                self.iter = Some(iter);
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::index::dict::IndexDictionary;
    use crate::index::storage::IndexStorage;
    use crate::index::test::{index_test_data, sorted_test_index};
    use crate::retrieve::build::RetrieverBuilder;
    use itertools::Itertools;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_retrieve_single_term() {
        let index = sorted_test_index();
        let exp_index = index_test_data().1;

        for (term, term_id) in index.dict().map.iter() {
            let mut retrieve_builder = RetrieverBuilder::new(&index);
            retrieve_builder.add_term_id(term_id as u64);
            retrieve_builder.in_all_postings();
            let retriever: DefaultRetriever<_> = retrieve_builder.retriever();

            let mut res: Vec<_> = retriever
                .map(|i| index.storage().get_item(i as usize).unwrap())
                .collect();
            res.sort_unstable();
            res.dedup();

            let mut exp = exp_index.get(&term).unwrap().clone();
            exp.sort_unstable();
            exp.dedup();
            assert_eq!(res, *exp);
        }
    }

    #[test]
    fn test_retrieve_multi_term() {
        let index = sorted_test_index();
        let exp_index = index_test_data().1;

        let dic_len = index.dict().len();

        for i in (1..(dic_len - 1)).step_by(13) {
            for chunk in &index.dict().map.iter().chunks(i) {
                let chunk = chunk.collect_vec();

                let mut retrieve_builder = RetrieverBuilder::new(&index);
                for (_, term_id) in chunk.iter() {
                    retrieve_builder.add_term_id(*term_id as u64);
                }
                retrieve_builder.in_all_postings();
                let retriever: DefaultRetriever<_> = retrieve_builder.retriever();

                let mut res: Vec<_> = retriever
                    .map(|i| index.storage().get_item(i as usize).unwrap())
                    .collect();
                res.sort_unstable();
                res.dedup();

                let mut exp = vec![];
                for (term, _) in chunk.iter() {
                    let e = exp_index.get(term).unwrap();
                    exp.extend_from_slice(e);
                }

                exp.sort_unstable();
                exp.dedup();
                assert_eq!(res, *exp);
            }
        }
    }
}
