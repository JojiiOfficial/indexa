/*
use crate::index::posting::IndexPosting;
use crate::retrieve::options::RetrieveOptions;
use crate::retrieve::retriever::RetrieveAlgo;

pub struct DefaultSortedRetriever<'a, P>
where
    P: IndexPosting + 'a,
{
    iter: SortedDedupedMultiIter<<P::PostingRetriever<'a> as IntoIterator>::IntoIter, u64>,
    options: RetrieveOptions,
}

impl<'a, P> RetrieveAlgo<'a, P> for DefaultSortedRetriever<'a, P>
where
    P: IndexPosting + 'a,
{
    #[inline]
    fn new(postings: &'a P, options: RetrieveOptions) -> Self {
        let mut iters = vec![];
        for term_id in options.term_ids.iter().copied() {
            for posting_id in options.posting_lists.iter().copied() {
                let Some(iter) = postings.posting_retriever(posting_id as usize, term_id) else {
                    continue;
                };
                let mut iter = iter.into_iter();
                let Some(first) = iter.next() else {
                    continue;
                };
                iters.push((iter, first));
            }
        }
        Self {
            options,
            iter: SortedDedupedMultiIter::new(iters),
        }
    }
}

impl<'a, P> Iterator for DefaultSortedRetriever<'a, P>
where
    P: IndexPosting + 'a,
{
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}

pub struct SortedDedupedMultiIter<I, V> {
    all_iters: Vec<(I, V)>,
    curr_min_idx: usize,
    remove_buf: Vec<usize>,
}

impl<I, V> SortedDedupedMultiIter<I, V>
where
    V: Ord + Copy,
{
    #[inline]
    pub fn new(iterator: Vec<(I, V)>) -> Self {
        let mut this = Self {
            remove_buf: Vec::with_capacity(iterator.len()),
            all_iters: iterator,
            curr_min_idx: 0,
        };
        this.set_min_index();
        this
    }

    #[inline]
    fn set_min_index(&mut self) -> Option<()> {
        self.curr_min_idx = self.min_index()?;
        Some(())
    }

    #[inline]
    fn min_index(&self) -> Option<usize> {
        let mut curr_min_idx = 0;

        for (pos, val) in self.all_iters.iter().enumerate() {
            if val.1 < self.all_iters[curr_min_idx].1 {
                curr_min_idx = pos;
            }
        }

        Some(curr_min_idx)
    }

    #[inline]
    fn curr_smallest(&self) -> Option<V> {
        self.all_iters.get(self.curr_min_idx).map(|i| i.1)
    }
}

impl<I, V> SortedDedupedMultiIter<I, V>
where
    I: Iterator<Item = V>,
    V: Ord + Copy,
{
    pub fn from_vec<D>(iter: Vec<D>) -> Self
    where
        D: IntoIterator<Item = V, IntoIter = I>,
    {
        let iter: Vec<_> = iter
            .into_iter()
            .filter_map(|vecs| {
                let mut iter = vecs.into_iter();
                let next = iter.next()?;
                Some((iter, next))
            })
            .collect();

        Self::new(iter)
    }

    #[inline]
    fn enforce_invariant(&mut self) {
        let min = self.curr_smallest().unwrap();

        let mut has = false;
        let mut min_index = 0;
        let mut min_val = self.all_iters[min_index].1;

        for (pos, (iter, old_next)) in self.all_iters.iter_mut().enumerate() {
            if *old_next > min {
                if !has || *old_next < min_val {
                    min_index = pos;
                    min_val = *old_next;
                }
                has = true;
                continue;
            }

            loop {
                *old_next = match iter.next() {
                    Some(v) => v,
                    None => {
                        self.remove_buf.push(pos);
                        break;
                    }
                };

                if *old_next > min {
                    if !has || *old_next < min_val {
                        min_val = *old_next;
                        min_index = pos;
                    }
                    has = true;
                    break;
                }
            }
        }

        if !has {
            // We're done but still have one item to yield so just clear all iterator as we're done here.
            self.all_iters.clear();
            return;
        }

        let mut offset = 0;
        if !self.remove_buf.is_empty() {
            let mut of = 0;

            for i in self.remove_buf.drain(..) {
                self.all_iters.remove(i - of);
                of += 1;
                if of <= min_index {
                    offset = of;
                }
            }
        }

        self.curr_min_idx = min_index - offset;
    }
}

impl<I, V> Iterator for SortedDedupedMultiIter<I, V>
where
    I: Iterator<Item = V>,
    V: Ord + Copy,
{
    type Item = V;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        let next = self.curr_smallest()?;
        self.enforce_invariant();
        Some(next)
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
    use std::collections::HashSet;
    use std::fmt::Debug;

    #[test]
    fn test() {
        let data = vec![
            vec![0, 1, 2, 3, 4],
            vec![0, 1, 2, 3, 4],
            vec![0, 1, 2, 3, 4],
            vec![0, 1, 2, 3, 4],
        ];
        let exp = vec![0, 1, 2, 3, 4];
        sorted_deduped(data, exp);

        let data = vec![
            vec![0, 1, 3, 4, 9, 13],
            vec![2, 2, 3, 4],
            vec![3, 3, 5, 7, 15],
            vec![0, 1, 3, 4, 13, 14],
        ];
        let exp = vec![0, 1, 2, 3, 4, 5, 7, 9, 13, 14, 15];
        sorted_deduped(data, exp);

        let data = vec![vec![0, 0, 1, 1], vec![0, 0, 1, 1]];
        let exp = vec![0, 1];
        sorted_deduped(data, exp);

        let data = vec![vec![], vec![]];
        let exp: Vec<u32> = vec![];
        sorted_deduped(data, exp);

        let data = vec![vec![1], vec![]];
        let exp = vec![1];
        sorted_deduped(data, exp);
    }

    fn sorted_deduped<V: Ord + Copy + Debug>(data: Vec<Vec<V>>, exp: Vec<V>) {
        let mut deduped = SortedDedupedMultiIter::from_vec(data);
        for exp in exp {
            assert_eq!(Some(exp), deduped.next());
        }
        assert!(deduped.next().is_none());
    }

    #[test]
    fn test_retrieve_single_term() {
        let index = sorted_test_index();
        let exp_index = index_test_data().1;

        for (term, term_id) in index.dict().map.iter() {
            let mut retrieve_builder = RetrieverBuilder::new(&index);
            retrieve_builder.add_term_id(term_id as u64);
            retrieve_builder.in_all_postings();
            let retriever: DefaultSortedRetriever<_> = retrieve_builder.retriever();

            let mut res: Vec<_> = retriever
                .map(|i| index.storage().get_item(i as usize).unwrap())
                .collect();
            res.sort_unstable();

            let mut exp = exp_index.get(&term).unwrap().clone();
            exp.sort_unstable();
            exp.dedup();
            pretty_assertions::assert_eq!(res, *exp);
        }
    }

    #[test]
    fn test_retrieve_multi_term() {
        let index = sorted_test_index();
        let exp_index = index_test_data().1;

        for i in 1..=5 {
            for chunk in &index.dict().map.iter().chunks(i) {
                let chunk = chunk.collect_vec();
                println!("{chunk:#?}");

                let mut retrieve_builder = RetrieverBuilder::new(&index);
                for (_, term_id) in chunk.iter() {
                    retrieve_builder.add_term_id(*term_id as u64);
                }
                retrieve_builder.in_all_postings();
                let retriever: DefaultSortedRetriever<_> = retrieve_builder.retriever();

                let mut res: Vec<_> = retriever
                    .map(|i| index.storage().get_item(i as usize).unwrap())
                    .collect();
                res.sort_unstable();
                res.dedup();

                let mut exp = vec![];
                for (term, _) in chunk.iter() {
                    let e = exp_index.get(term).unwrap();
                    exp.extend(e.iter().cloned());
                }
                exp.sort_unstable();
                exp.dedup();

                pretty_assertions::assert_eq!(res, *exp);
            }
        }
    }
}
*/
