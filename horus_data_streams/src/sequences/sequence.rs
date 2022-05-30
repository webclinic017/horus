use std::cell::RefCell;

use heapless::spsc::Queue;

pub struct Sequence<T, const SIZE: usize> {
    pub data: RefCell<Queue<T, SIZE>>
}

impl<T, const SIZE: usize> Sequence<T, SIZE> {
    pub fn get_length(&self) -> usize {
        self.data.borrow().capacity()
    }
}