mod linear_search;
pub use linear_search::LinearSearch;

pub trait SearchTrait<T> {
    fn search(&self, data: &[T], query: &T) -> Option<usize>;
}