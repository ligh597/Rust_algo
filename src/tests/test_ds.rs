use crate::data_structure::array::Array;

pub(crate) fn test() {
    let mut data:Array<i32>= Array::new();
    for i in 0..12 {
        data.add_last(i);
    }
    println!("{}",data);
}
