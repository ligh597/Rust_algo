pub(crate) struct ArrayGenerator{}

impl ArrayGenerator {
    pub(crate) fn generate_ordered_array(n: i32) -> Vec<i32> {
        let mut arr = Vec::with_capacity(n as usize);
        for i in 0..n {
            arr.push(i);
        }
        arr
    }
}