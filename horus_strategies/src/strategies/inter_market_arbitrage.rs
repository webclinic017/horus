use horus_data_streams::receivers::data_receiver::DataReceiver;
use horus_exchanges::connectors::market_connector::MarketConnector;
use horus_finance::aggregate::Aggregate;

use super::strategy::Strategy;

pub struct InterMarketArbitrageStrategy<'a, MarketOne: MarketConnector, MarketTwo: MarketConnector> {
    market_connector_01: &'a MarketOne,
    market_connector_02: &'a MarketTwo
}

impl<'a, MarketOne: MarketConnector, MarketTwo: MarketConnector> InterMarketArbitrageStrategy<'a, MarketOne, MarketTwo> {
    pub fn new(market_01: &'a MarketOne, market_02: &'a MarketTwo) -> InterMarketArbitrageStrategy<'a, MarketOne, MarketTwo> {
        InterMarketArbitrageStrategy {
            market_connector_01: market_01,
            market_connector_02: market_02
        }
    }
}

impl<'a, MarketOne: MarketConnector, MarketTwo: MarketConnector> Strategy for InterMarketArbitrageStrategy<'a, MarketOne, MarketTwo> {
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