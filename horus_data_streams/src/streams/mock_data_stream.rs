use std::{rc::{Weak, Rc}, cell::RefCell};

use horus_finance::aggregate::Aggregate;

use crate::sequences::aggregate_sequence::AggregateSequence;

use super::data_stream::DataStream;

pub struct MockAggregateStream<const BUFFER_SIZE: usize> {
    sequence: AggregateSequence<BUFFER_SIZE>,
    pre_publish: RefCell<Weak<dyn Fn()>>,
    on_data: RefCell<Weak<dyn Fn()>>
}

impl<const BUFFER_SIZE: usize> MockAggregateStream<BUFFER_SIZE> {
    pub fn inject(&mut self, datum: Aggregate) {
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

impl<const BUFFER_SIZE: usize> DataStream<Aggregate> for MockAggregateStream<BUFFER_SIZE> {
    fn start_listening(&mut self) {
        panic!("This method is not available for this mocking class")
    }

    fn set_on_data(&self, _on_data: Rc<dyn Fn()>) {
        let mut mut_ref = self.on_data.borrow_mut();
        *mut_ref = Rc::downgrade(&_on_data);
    }

    fn get_historical_data(&self, _start: chrono::DateTime<chrono::Utc>, _end: chrono::DateTime<chrono::Utc>) -> Vec<Aggregate> {
        panic!("This method is not available for this mocking class")
    }
}