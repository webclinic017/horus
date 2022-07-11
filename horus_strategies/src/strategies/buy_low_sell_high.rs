use horus_exchanges::connectors::market_connector::MarketConnector;
use horus_finance::aggregate::Aggregate;

use crate::signals::golden_cross::{GoldenCrossSignal, GoldenCrossSignalType};

use super::strategy::Strategy;

pub struct BuyLowSellHighStrategy<'a, Market: MarketConnector> {
    market: &'a Market,
    golden_cross_signal: GoldenCrossSignal<20, 200>
}

impl<'a, Market: MarketConnector> BuyLowSellHighStrategy<'a, Market> {
    pub fn new(market: &'a Market) -> BuyLowSellHighStrategy<'a, Market> {
        BuyLowSellHighStrategy {
            market,
            golden_cross_signal: GoldenCrossSignal::<20, 200>::new()
        }
    }

    fn run_hot_path(&self) {
        todo!();
    }

    pub fn next(&mut self, aggregate: Aggregate) {
        let result = self.golden_cross_signal.next(aggregate);
        if result == Some(GoldenCrossSignalType::LongOvertakes) {
            println!("SELL!");
        }
        if result == Some(GoldenCrossSignalType::ShortOvertakes) {
            println!("BUY!");
        }
    }
}

impl<'a, Market: MarketConnector> Strategy for BuyLowSellHighStrategy<'a, Market> {
    
    fn get_name() -> &'static str {
        return "Buy Low Sell High";
    }
}