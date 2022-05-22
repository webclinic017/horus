use chrono::{DateTime, Utc};
use horus_finance::Aggregate;

pub trait DataReceiver {

    fn start_listening(&self, on_data_receive: &dyn Fn());
    fn get_historical_data(&self, start: DateTime<Utc>, end: DateTime<Utc>) -> Vec<Aggregate>;
}