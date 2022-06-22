use std::rc::Rc;

use chrono::{DateTime, Utc};

pub trait DataStream<DATATYPE> {

    fn start_listening(&self);
    fn set_on_data(&self, _on_data: Rc<dyn Fn()>);
    fn get_historical_data(&self, start: DateTime<Utc>, end: DateTime<Utc>) -> Vec<DATATYPE>;
}