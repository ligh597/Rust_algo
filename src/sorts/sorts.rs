pub use crate::sorts::SortTrait;

pub struct SelectionSort;
pub struct InsertionSort;

impl<T: Ord + Copy> SortTrait<T> for SelectionSort {
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

impl<T: Ord + Copy> SortTrait<T> for InsertionSort {
    fn sort(&self, data: &mut [T], is_reverse: bool) {
        for i in 0..data.len(){

            let mut j = i;
            let temp = data[i];
            while j > 0 && temp < data[j-1] {
                data[j] = data[j-1];
                j -= 1;
            }
            data[j] = temp; // 优化，替换swap
        }
        if is_reverse {
            data.reverse();
        }
    }
}
