pub struct SimulationElement<'a, DATATYPE> {
    datum: DATATYPE,
    stream: &'a dyn DataStream<DATATYPE>,
    market_connector: Option<&'a MockMarketConnector>
}