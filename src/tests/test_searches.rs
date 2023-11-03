use crate::searches::{LinearSearch, SearchTrait};

#[derive(PartialEq)]
struct Student {
    name: String,
    age: u8,
    grade: u8,
}

impl Student {
    fn new(name: &str, age: u8, grade: u8) -> Self {
        Self {
            name: name.to_string(),
            age,
            grade,
        }
    }
}

pub(crate) fn test() {
    let students = vec![
        Student::new("John", 16, 11),
        Student::new("Jane", 15, 10),
        Student::new("Bob", 17, 12),
    ];
    let query = Student::new("Jane", 15, 10);
    let search = LinearSearch;
    let result = search.search(&students, &query);
    match result {
        Some(idx) => println!("Found {} at index {}", query.name, idx),
        None => println!("{} not found", query.name),
    }
}