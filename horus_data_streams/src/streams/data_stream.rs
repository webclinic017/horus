use chrono::{DateTime, Utc};

use crate::models::time_series_element::TimeSeriesElement;

pub trait DataStream<DATATYPE> {

    fn start_listening(&mut self);
    fn get_historical_data(&self, start: DateTime<Utc>, end: DateTime<Utc>) -> Vec<TimeSeriesElement<DATATYPE>>;
}