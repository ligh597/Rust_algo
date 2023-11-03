pub use crate::sorts::SortTrait;

pub struct SelectionSort;

impl<T: Ord> SortTrait<T> for SelectionSort {
    fn sort(&self, data: &mut [T]) {
        for i in 0..data.len() {
            let mut min_index = i;
            for j in i+1..data.len() {
                if data[j] < data[min_index] {
                    min_index = j;
                }
            }
            data.swap(i, min_index);
        }
    }
}