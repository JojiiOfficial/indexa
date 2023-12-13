use crate::retrieve::options::RetrieveOptions;

pub mod default;
pub mod ngram;
mod sorted;

pub trait RetrieveAlgo<'a, P>: Iterator<Item = u64> {
    fn new(postings: &'a P, options: RetrieveOptions) -> Self;
}
