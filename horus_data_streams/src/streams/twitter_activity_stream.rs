use std::{rc::{Rc, Weak}, cell::RefCell};

use crate::models::time_series_element::TimeSeriesElement;

use super::data_stream::DataStream;

pub struct TwitterActivityStream {
    username: String,
    on_data: Option<RefCell<Weak<dyn Fn()>>>
}

impl DataStream::<()> for TwitterActivityStream {
    fn start_listening(&mut self) {
        todo!()
    }

    fn get_historical_data(&self, start: chrono::DateTime<chrono::Utc>, end: chrono::DateTime<chrono::Utc>) -> Vec<TimeSeriesElement<()>> {
        todo!()
    }
}