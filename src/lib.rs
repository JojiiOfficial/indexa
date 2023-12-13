pub mod edit;
pub mod error;
pub mod index;
pub mod retrieve;

pub type Result<T> = std::result::Result<T, error::Error>;
