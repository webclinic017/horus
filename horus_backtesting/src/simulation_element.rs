use horus_data_streams::streams::mock_data_stream::MockDataStream;

pub struct SimulationElement<'a, DATATYPE> {
    pub datum: DATATYPE,
    pub stream: &'a MockDataStream<DATATYPE>
}