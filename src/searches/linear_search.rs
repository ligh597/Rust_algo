use crate::searches::SearchTrait;

pub struct LinearSearch;

impl<T: PartialEq> SearchTrait<T> for LinearSearch {
    fn search(&self, data: &[T], query: &T) -> Option<usize> {
        // for (idx, item) in data.iter().enumerate() {
        //     if item == query {
        //         return Some(idx);
        //     }
        // }
        // None
        data.iter().position(|x| x == query)
    }
}