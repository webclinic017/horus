use chrono::{DateTime, Utc};

pub trait DataReceiver<DATATYPE> {

    fn start_listening(&self);
    fn inject(&self, datum: DATATYPE);
    fn get_historical_data(&self, start: DateTime<Utc>, end: DateTime<Utc>) -> Vec<DATATYPE>;
}