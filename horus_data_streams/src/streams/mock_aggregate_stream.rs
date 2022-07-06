use horus_finance::aggregate::Aggregate;

use crate::models::time_series_element::TimeSeriesElement;

use super::data_stream::DataStream;

pub struct MockAggregateStream<'a> {
    on_data: &'a mut dyn FnMut(Aggregate)
}

impl<'a> MockAggregateStream<'a> {

    pub fn new(on_data: &'a mut dyn FnMut(Aggregate)) -> MockAggregateStream<'a> {
        MockAggregateStream {
            on_data
        }
    }

    pub fn inject(&mut self, datum: TimeSeriesElement<Aggregate>) {
        (self.on_data)(datum.datum);
    }
}

impl<'a> DataStream<Aggregate> for MockAggregateStream<'a> {
    fn start_listening(&mut self) {
        panic!("This method is not available for this mocking class")
    }

    fn get_historical_data(&self, _start: chrono::DateTime<chrono::Utc>, _end: chrono::DateTime<chrono::Utc>) -> Vec<TimeSeriesElement<Aggregate>> {
        panic!("This method is not available for this mocking class")
    }
}