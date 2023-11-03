pub mod selection_sort;
pub trait SortTrait<T> {
    fn sort(&self, data: &mut [T]);
}