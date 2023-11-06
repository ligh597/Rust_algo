pub mod sorts;

pub trait SortTrait<T> {
    fn sort(&self, data: &mut [T],is_reverse:bool);
}