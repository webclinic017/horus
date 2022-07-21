use chrono::{DateTime, Utc};

use crate::models::time_series_element::TimeSeriesElement;

pub trait DataStream<DATATYPE> {

    fn start_listening(&mut self, on_data: &mut dyn FnMut(DATATYPE));
    fn start_with_replay(&mut self, on_data: &mut dyn FnMut(DATATYPE), replay_length: usize, report_replay: bool);
    fn get_historical_data(&self, start: DateTime<Utc>, end: DateTime<Utc>) -> Vec<TimeSeriesElement<DATATYPE>>;
}