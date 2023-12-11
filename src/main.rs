pub mod edit;
pub mod error;
pub mod index;
pub mod retrieve;
pub mod traits;
mod utils;

pub type Result<T> = std::result::Result<T, error::Error>;

fn main() {
    println!("Hello, world!");
}
