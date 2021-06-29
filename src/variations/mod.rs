//! Variations of binary search

mod exponential_search;
mod interpolation_search;
mod uniform;

pub use exponential_search::exponential_search;
pub use interpolation_search::{interpolation_search, linear_interpolation_search};
pub use uniform::UniformBinarySearch;
