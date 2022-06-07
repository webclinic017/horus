use horus_data_streams::receivers::data_receiver::DataReceiver;
use horus_finance::Aggregate;

use super::strategy::Strategy;

struct InterMarketArbitrage<'a, ExOne: DataReceiver<Aggregate>, ExTwo: DataReceiver<Aggregate>> {
    exchange_connector_01: &'a ExOne,
    exchange_connector_02: &'a ExTwo
}

impl<'a, ExOne: MarketConnector, ExTwo: MarketConnector> Strategy for InterMarketArbitrage<'a, ExOne, ExTwo> {
    fn run<REPORTER: Reporter>(&self, order_handler: Reporter) -> std::thread::JoinHandle<()> {
        todo!()
    }

    fn get_name() -> &'static str {
        return "Inter Exchange Arbitrage";
    }
}