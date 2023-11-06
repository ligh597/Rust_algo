use crate::sorts::sorts;
use crate::sorts::SortTrait;
pub(crate) fn test() {
    let mut data = vec![2,3,6,5,3,1,7];

    sorts::InsertionSort.sort(&mut data,false);
    println!("Sorted data: {:?}", data)
}