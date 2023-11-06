use crate::data_structure::array::Array;

pub(crate) fn test() {
    let mut data: Array<i32> = Array::new(None);
    for i in 0..17 {
        data.add_last(i);
    }
    println!("{}", data);
}
