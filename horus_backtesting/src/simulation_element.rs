use horus_data_streams::streams::mock_data_stream::MockDataStream;
use horus_finance::time_series_element::TimeSeriesElement;

pub struct SimulationElement<'a, DATATYPE> {
    pub datum: DATATYPE,
    pub stream: &'a MockDataStream<Box<dyn TimeSeriesElement>, 2>
}