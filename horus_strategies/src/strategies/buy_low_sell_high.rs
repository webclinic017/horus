use horus_data_streams::receivers::data_receiver::DataReceiver;
use horus_exchanges::connectors::market_connector::MarketConnector;
use horus_finance::aggregate::Aggregate;

use super::strategy::Strategy;

pub struct BuyLowSellHighStrategy<'a, Market: MarketConnector> {
    market_connector: &'a Market
}

impl<'a, Market: MarketConnector> BuyLowSellHighStrategy<'a, Market> {
    pub fn new(market: &'a Market) -> BuyLowSellHighStrategy<Market> {
        BuyLowSellHighStrategy {
            market_connector: market
        }
    }
}

impl<'a, Market: MarketConnector> Strategy for BuyLowSellHighStrategy<'a, Market> {
    fn run(&self) -> std::thread::JoinHandle<()> {
        todo!()
    }
    
    fn get_name() -> &'static str {
        return "Buy Low Sell High";
    }
}