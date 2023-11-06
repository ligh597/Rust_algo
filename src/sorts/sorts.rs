pub use crate::sorts::SortTrait;

pub struct SelectionSort;
pub struct InsertionSort;

impl<T: Ord> SortTrait<T> for SelectionSort {
    fn sort(&self, data: &mut [T], is_reverse: bool) {
        for i in 0..data.len() {
            let mut min = i;
            for j in (i + 1)..data.len() {
                if data[j] < data[min] {
                    min = j;
                }
            }
            if min != i {
                data.swap(i, min);
            }
        }
        if is_reverse {
            data.reverse();
        }
    }
}

impl<T: Ord> SortTrait<T> for InsertionSort {
    fn sort(&self, data: &mut [T], is_reverse: bool) {
        for i in 1..data.len() {
            let mut j = i;
            while j > 0 && data[j - 1] > data[j] {
                data.swap(j - 1, j);
                j -= 1;
            }
        }
        if is_reverse {
            data.reverse()
        }
    }
}
