use horus_data_streams::receivers::data_receiver::DataReceiver;
use horus_exchanges::connectors::market_connector::MarketConnector;
use horus_finance::aggregate::Aggregate;

use super::strategy::Strategy;

struct InterMarketArbitrage<'a, ExOne: MarketConnector, ExTwo: MarketConnector> {
    exchange_connector_01: &'a ExOne,
    exchange_connector_02: &'a ExTwo
}

impl<'a, ExOne: MarketConnector, ExTwo: MarketConnector> Strategy for InterMarketArbitrage<'a, ExOne, ExTwo> {
    fn run(&self) -> std::thread::JoinHandle<()> {
        todo!()
    }

    fn get_market_connectors(&self) -> &Vec<&dyn MarketConnector> {
        todo!()
    }

    fn get_name() -> &'static str {
        return "Inter Exchange Arbitrage";
    }
}