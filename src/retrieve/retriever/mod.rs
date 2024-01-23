pub mod all_terms;
pub mod default;
pub mod ngram;
mod sorted;

use crate::retrieve::options::RetrieveOptions;

pub trait RetrieveAlgo<'a, P>: Iterator<Item = u64> {
    fn new(postings: &'a P, options: RetrieveOptions) -> Self;
}
