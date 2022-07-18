use horus_exchanges::connectors::market_connector::MarketConnector;
use horus_finance::{aggregate::Aggregate, order::Order, order_side::OrderSide};
use horus_reporters::reporter::Reporter;

use crate::signals::golden_cross::{GoldenCrossSignal, GoldenCrossSignalType};

use super::strategy::Strategy;

pub struct BuyLowSellHighStrategy<'a, Market: MarketConnector, Rep: Reporter> {
    market: &'a Market,
    reporter: Rep,
    golden_cross_signal: GoldenCrossSignal<20, 200>
}

impl<'a, Market: MarketConnector, Rep: Reporter> BuyLowSellHighStrategy<'a, Market, Rep> {
    pub fn new(market: &'a Market, reporter: Rep) -> BuyLowSellHighStrategy<'a, Market, Rep> {
        BuyLowSellHighStrategy {
            market,
            reporter,
            golden_cross_signal: GoldenCrossSignal::<20, 200>::new()
        }
    }

    pub fn next(&mut self, aggregate: Aggregate) {
        let result = self.golden_cross_signal.next(aggregate);
        if result == Some(GoldenCrossSignalType::LongOvertakes) {
            let order = Order { 
                side: OrderSide::SELL, 
                quantity: 1, 
                price: None
            };
            let res = self.market.route_take_order(&order);
            if res == Some(position) {
                self.reporter.add_position(&position);
            }

        }
        if result == Some(GoldenCrossSignalType::ShortOvertakes) {
            let order = Order { 
                side: OrderSide::SELL, 
                quantity: 1, 
                price: None
            };
            let res = self.market.route_take_order(&order);
            if res == Some(position) {
                self.reporter.add_position(&position);
            }
        }
    }
}

impl<'a, Market: MarketConnector, Rep: Reporter> Strategy for BuyLowSellHighStrategy<'a, Market, Rep> {
    
    fn get_name() -> &'static str {
        return "Buy Low Sell High";
    }
}