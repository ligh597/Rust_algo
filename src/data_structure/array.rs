#![allow(dead_code)]
use std::fmt;
use std::fmt::Formatter;

use std::ops::Index;


pub(crate) struct Array<T> {
    data: Vec<T>,
    size: usize,
}

impl<T:Copy> Array<T>{
    pub fn new(capacity:Option<usize>) -> Self{
        match capacity {
            Some(capacity) => {
                Array {
                    data: Vec::with_capacity(capacity),
                    size: 0,
                }
            },
            None => {
                Array {
                    data: Vec::with_capacity(16),
                    size: 0,
                }
            }
        }
    }
    pub fn get_size(&self) -> usize{
        self.size
    }
    pub fn get_capacity(&self) -> usize{
        self.data.capacity()
    }
    pub fn is_empty(&self) -> bool{
        self.size == 0
    }
    pub fn add_first(&mut self, e: T){
        self.data.insert(0, e);
        self.size += 1;
    }
    pub fn add_last(&mut self, e: T){
        self.data.push(e);
        self.size += 1;
    }
    pub fn add(&mut self, index: usize, e: T){
        if index > self.size{
            panic!("Index is illegal.");
        }
        if self.size == self.data.capacity(){
            self.resize(self.data.capacity() * 2);
        }
        self.data.insert(index, e);
        self.size += 1;
    }
    pub fn resize(&mut self, new_capacity: usize){
        let mut new_data: Vec<T> = Vec::with_capacity(new_capacity);
        for i in 0..self.size{
            new_data.push(self.data[i]);
        }
        self.data = new_data;
    }
}

impl<T> Index<usize> for Array<T>{
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        if index >= self.size{
            panic!("Index is illegal.");
        }
        &self.data[index]
    }
}

impl<T: fmt::Display> fmt::Display for Array<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Array: size = {}, capacity = {}\n", self.size, self.data.capacity())?;
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
