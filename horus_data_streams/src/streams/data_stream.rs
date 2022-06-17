use chrono::{DateTime, Utc};

pub trait DataStream<'a, DATATYPE> {

    fn start_listening(&self, hot_path: &dyn Fn(&str, DATATYPE));
    fn add_middleware(&self, on_data: &'a dyn Fn(&str, DATATYPE) -> ());
    fn get_historical_data(&self, start: DateTime<Utc>, end: DateTime<Utc>) -> Vec<DATATYPE>;
}