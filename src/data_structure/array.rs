#![allow(dead_code)]
use std::fmt;
use std::fmt::Formatter;

use std::ops::Index;

pub(crate) struct Array<T> {
    data: Vec<T>,
    size: usize,
}

impl<T: Copy + PartialEq> Array<T> {
    pub fn new() -> Self {
        Array {
            data: Vec::new(),
            size: 0,
        }
    }
    pub fn get_size(&self) -> usize {
        self.size
    }
    pub fn is_empty(&self) -> bool {
        self.size == 0
    }
    pub fn add_first(&mut self, e: T) {
        self.data.insert(0, e);
        self.size += 1;
    }
    pub fn add_last(&mut self, e: T) {
        self.data.push(e);
        self.size += 1;
    }
    pub fn add(&mut self, index: usize, e: T) {
        if index > self.size {
            panic!("Index is illegal.");
        }
        self.data.insert(index, e);
        self.size += 1;
    }
    pub fn get(&self, index: usize) -> T {
        if index >= self.size {
            panic!("Index is illegal.");
        }
        self.data[index]
    }
    pub fn find(&self, e: T) -> Option<usize> {
        for i in 0..self.size {
            if self.data[i] == e {
                return Some(i);
            }
        }
        None
    }
    pub fn find_all(&self, e: T) -> Vec<usize> {
        let mut ret: Vec<usize> = Vec::new();
        for i in 0..self.size {
            if self.data[i] == e {
                ret.push(i);
            }
        }
        ret
    }
    pub fn remove(&mut self, index: usize) -> T {
        if index >= self.size {
            panic!("Index is illegal.");
        }
        let ret = self.data.remove(index);
        self.size -= 1;
        ret
    }
    pub fn remove_element(&mut self, e: T) {
        let index = self.find(e);
        match index {
            Some(index) => {
                self.remove(index);
                self.size -= 1;
            }
            None => {
                panic!("Element is not exist.");
            }
        }
    }
    pub fn remove_element_all(&mut self, e: T) {
        let index = self.find_all(e);
        let mut d = 0;
        for i in index {
            self.remove(i - d);
            d += 1;
        }
    }
}

impl<T> Index<usize> for Array<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        if index >= self.size {
            panic!("Index is illegal.");
        }
        &self.data[index]
    }
}

impl<T: fmt::Display> fmt::Display for Array<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Array: \nsize = {}\nitem = ", self.size,)?;
        write!(f, "[")?;
        for i in 0..self.size {
            write!(f, "{}", self.data[i])?;
            if i != self.size - 1 {
                write!(f, ", ")?;
            }
        }
        write!(f, "]")
    }
}
