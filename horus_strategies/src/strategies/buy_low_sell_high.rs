use horus_exchanges::connectors::market_connector::MarketConnector;
use horus_finance::aggregate::Aggregate;

use crate::signals::golden_cross::GoldenCrossSignal;

use super::strategy::Strategy;

pub struct BuyLowSellHighStrategy<'a, Market: MarketConnector> {
    market: &'a Market,
    golden_cross_signal: &'a GoldenCrossSignal<20, 200>
}

impl<'a, Market: MarketConnector> BuyLowSellHighStrategy<'a, Market> {
    pub fn new(market: &'a Market, signal: &'a GoldenCrossSignal<20, 200>) -> BuyLowSellHighStrategy<'a, Market> {
        BuyLowSellHighStrategy {
            market,
            golden_cross_signal: signal
        }
    }

    fn run_hot_path(&self) {
        todo!();
    }

    pub fn next(&self, aggregate: Aggregate) {
        self.golden_cross_signal.next(aggregate);
        self.run_hot_path();
    }
}

impl<'a, Market: MarketConnector> Strategy for BuyLowSellHighStrategy<'a, Market> {
    
    fn get_name() -> &'static str {
        return "Buy Low Sell High";
    }
}