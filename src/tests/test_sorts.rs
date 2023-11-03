use crate::sorts::selection_sort;
use crate::sorts::SortTrait;
pub(crate) fn test() {
    let mut data = vec![2,3,6,5,3,1,7];

    selection_sort::SelectionSort.sort(&mut data);
    println!("Sorted data: {:?}", data)
}