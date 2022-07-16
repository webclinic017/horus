use horus_exchanges::connectors::market_connector::MarketConnector;
use horus_finance::{aggregate::Aggregate, order::Order, order_side::OrderSide};

use crate::signals::golden_cross::{GoldenCrossSignal, GoldenCrossSignalType};

use super::strategy::Strategy;

pub struct BuyLowSellHighStrategy<'a, Market: MarketConnector, Reporter: Reporter> {
    market: &'a Market,
    reporter: &'a Reporter,
    golden_cross_signal: GoldenCrossSignal<20, 200>
}

impl<'a, Market: MarketConnector, Reporter: Reporter> BuyLowSellHighStrategy<'a, Market, Reporter> {
    pub fn new(market: &'a Market, reporter: &'a Reporter) -> BuyLowSellHighStrategy<'a, Market> {
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
                price: None, 
                expiration_date: None 
            };
            let success = self.market.route_take_order(&order);
        }
        if result == Some(GoldenCrossSignalType::ShortOvertakes) {
            let order = Order { 
                side: OrderSide::BUY, 
                quantity: 1, 
                price: None, 
                expiration_date: None 
            };
            self.market.route_take_order(&order);
        }
    }
}

impl<'a, Market: MarketConnector> Strategy for BuyLowSellHighStrategy<'a, Market> {
    
    fn get_name() -> &'static str {
        return "Buy Low Sell High";
    }
}