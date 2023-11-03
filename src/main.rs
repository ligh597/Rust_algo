mod searches;
use crate::searches::{LinearSearch, SearchTrait};

fn main() {
    let data = vec![1, 2, 3, 4, 5];
    let query = 3;
    let search = LinearSearch;
    let result = search.search(&data, &query);
    match result {
        Some(idx) => println!("Found {} at index {}", query, idx),
        None => println!("{} not found", query),
    }
}
