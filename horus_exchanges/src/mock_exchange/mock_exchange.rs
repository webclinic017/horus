use horus_finance::order_book::Orderbook;

use crate::market_connector::MarketConnector;

pub struct MockExchange {
    pub order_book: Orderbook
}

impl MockMarketConnector {
    pub fn new() -> MockMarketConnector {
        MockMarketConnector {
            order_book: Orderbook::new()
        }
    }

    pub fn inject(&self, order: Order) {
        todo!();
    }

    pub fn inject(&self, order: Aggregate) {
        todo!();
    }
}

impl Default for MockMarketConnector {
    fn default() -> Self {
        Self::new()
    }
}

impl MarketConnector for MockMarketConnector {

    fn make_order(&self, order: MakeOrder) -> OrderPlacedResult {
        todo!()
    }

    fn take_order(&self, order: TakeOrder) -> OrderFilledResult {
        todo!()
    }
}