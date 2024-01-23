use crate::index::posting::IndexPosting;
use crate::retrieve::options::RetrieveOptions;
use crate::retrieve::retriever::RetrieveAlgo;
use std::cmp::Ordering;

pub struct AllTermRetriever<'a, P>
where
    P: IndexPosting + 'a,
{
    iters: Vec<
        Vec<(
            <P::PostingRetriever<'a> as IntoIterator>::IntoIter,
            Option<u64>,
        )>,
    >,
}

impl<'a, P> RetrieveAlgo<'a, P> for AllTermRetriever<'a, P>
where
    P: IndexPosting + 'a,
{
    #[inline]
    fn new(postings: &'a P, options: RetrieveOptions) -> Self {
        let mut iters = vec![];
        for term_id in options.term_ids.iter().copied() {
            let mut term_iter = vec![];
            for posting_id in options.posting_lists.iter().copied() {
                let Some(iter) = postings.posting_retriever(posting_id as usize, term_id) else {
                    continue;
                };
                let mut iter = iter.into_iter();
                let next = iter.next();
                if next.is_none() {
                    continue;
                }

                term_iter.push((iter, next));
            }
            if !term_iter.is_empty() {
                iters.push(term_iter);
            }
        }
        Self { iters }
    }
}

impl<'a, P> AllTermRetriever<'a, P>
where
    P: IndexPosting + 'a,
{
    pub fn min_item_at(&mut self, iter_idx: usize) -> Option<u64> {
        let term_iters = self.iters.get_mut(iter_idx)?;

        let mut min = u64::MAX;
        let mut min_pos = 0;
        let mut found = false;
        for (p, (_, val)) in term_iters.iter_mut().enumerate() {
            if val.is_none() {
                continue;
            }
            let val = val.unwrap();
            if val < min {
                min = val;
                min_pos = p;
                found = true;
            }
        }

        if !found {
            None
        } else {
            let (iter, v) = term_iters.get_mut(min_pos).unwrap();
            *v = iter.next();
            Some(min)
        }
    }
}

impl<'a, P> Iterator for AllTermRetriever<'a, P>
where
    P: IndexPosting + 'a,
{
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        let iterlen = self.iters.len();

        let mut i = 1;
        let mut max_i = 0;
        let mut max_val = self.min_item_at(0)?;

        while i != max_i {
            if iterlen >= i {
                return Some(max_val);
            }

            loop {
                let curr = self.min_item_at(i)?;

                match curr.cmp(&max_val) {
                    Ordering::Less => continue,
                    Ordering::Greater => {
                        max_val = curr;
                        max_i = i;
                    }
                    _ => {}
                }

                break;
            }

            i = (i + 1) % iterlen;
        }

        Some(max_val)
    }
}
