use horus_data_streams::streams::data_stream::DataStream;
use horus_exchanges::connectors::market_connector::MarketConnector;
use horus_finance::aggregate::Aggregate;

use super::strategy::Strategy;

pub struct BuyLowSellHighStrategy<'a, Market: MarketConnector> {
    market: &'a Market
}

impl<'a, Market: MarketConnector> BuyLowSellHighStrategy<'a, Market> {
    pub fn new(market: &'a Market) -> BuyLowSellHighStrategy<Market> {
        BuyLowSellHighStrategy {
            market: market
        }
    }
}

impl<'a, Market: MarketConnector> Strategy for BuyLowSellHighStrategy<'a, Market> {
    fn run_hot_path(&self) {
        todo!()
    }
    
    fn get_name() -> &'static str {
        return "Buy Low Sell High";
    }
}