use std::rc::Weak;

use chrono::{DateTime, Utc};

pub trait DataStream<DATATYPE> {

    fn start_listening(&self);
    fn add_middleware(&self, on_data: &dyn Fn());
    fn get_historical_data(&self, start: DateTime<Utc>, end: DateTime<Utc>) -> Vec<DATATYPE>;
}