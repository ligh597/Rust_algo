pub mod linear_search;


pub trait SearchTrait<T> {
    fn search(&self, data: &[T], query: &T) -> Option<usize>;
}