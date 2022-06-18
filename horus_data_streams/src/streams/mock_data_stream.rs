use std::rc::Weak;

use crate::sequences::sequence::Sequence;

use super::data_stream::DataStream;

pub struct MockDataStream<DATATYPE> {
    pub sequence: Sequence<DATATYPE, 2>, //TODO
    pub on_data: Weak<dyn Fn()>
}

impl<DATATYPE> MockDataStream<DATATYPE> {
    pub fn inject(&self, datum: DATATYPE) {
        match self.on_data.upgrade() {
            Some(action) => action(),
            None => {}
        }
    }
}

impl<DATATYPE> DataStream<DATATYPE> for MockDataStream<DATATYPE> {
    fn start_listening(&self) {
        panic!("This method is not available for this mocking class")
    }

    fn add_middleware(&self, on_data: &dyn Fn()) {
        todo!()
    }

    fn get_historical_data(&self, start: chrono::DateTime<chrono::Utc>, end: chrono::DateTime<chrono::Utc>) -> Vec<DATATYPE> {
        panic!("This method is not available for this mocking class")
    }
}