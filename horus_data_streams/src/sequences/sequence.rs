use std::cell::RefCell;

use heapless::spsc::Queue;

pub struct Sequence<T, const SIZE: usize> {
    pub data: RefCell<Queue<T, SIZE>>
}

impl<T, const SIZE: usize> Sequence<T, SIZE> {
    pub fn get_length(&self) -> usize {
        self.data.borrow().capacity()
    }

    pub fn enqueue(&self, item: T) -> Option<T> {

        let mut dequeued: Option<T> = None;
        if self.data.borrow().is_full() {
            dequeued = self.data.borrow_mut().dequeue();
        }

        let _ = self.data.borrow_mut().enqueue(item);

        dequeued
    }
}