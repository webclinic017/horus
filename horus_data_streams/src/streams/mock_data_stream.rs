use std::{rc::{Weak, Rc}, cell::RefCell};

use crate::sequences::sequence::Sequence;

use super::data_stream::DataStream;

pub struct MockDataStream<DATATYPE, const BUFFER_SIZE: usize> {
    sequence: Sequence<DATATYPE, BUFFER_SIZE>,
    pre_publish: RefCell<Weak<dyn Fn()>>,
    on_data: RefCell<Weak<dyn Fn()>>
}

impl<DATATYPE, const BUFFER_SIZE: usize> MockDataStream<DATATYPE, BUFFER_SIZE> {
    pub fn inject(&self, datum: DATATYPE) {
        self.sequence.enqueue(datum);
        let mut_ref = self.pre_publish.borrow_mut();
        match mut_ref.upgrade() {
            Some(action) => action(),
            None => {}
        }
        let mut_ref = self.on_data.borrow_mut();
        match mut_ref.upgrade() {
            Some(action) => action(),
            None => {}
        }
    }

    pub fn set_pre_publish(&self, _pre_publish: Rc<dyn Fn()>) {
        let mut mut_ref = self.pre_publish.borrow_mut();
        *mut_ref = Rc::downgrade(&_pre_publish);
    }
}

impl<DATATYPE, const BUFFER_SIZE: usize> DataStream<DATATYPE> for MockDataStream<DATATYPE, BUFFER_SIZE> {
    fn start_listening(&self) {
        panic!("This method is not available for this mocking class")
    }

    fn set_on_data(&self, _on_data: Rc<dyn Fn()>) {
        let mut mut_ref = self.on_data.borrow_mut();
        *mut_ref = Rc::downgrade(&_on_data);
    }

    fn get_historical_data(&self, _start: chrono::DateTime<chrono::Utc>, _end: chrono::DateTime<chrono::Utc>) -> Vec<DATATYPE> {
        panic!("This method is not available for this mocking class")
    }
}