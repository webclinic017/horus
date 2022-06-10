use horus_finance::{aggregate::Aggregate, market_snapshot::MarketSnapshot, order::Order};

use crate::connectors::market_connector::MarketConnector;

pub struct MockMarketConnector {
    pub current_ask: f32,
    pub current_bid: f32,
    pub asset_balance: usize,
    pub cash_balance: f32
}

impl MockMarketConnector {
    pub fn new(initial_cash_balance: f32) -> MockMarketConnector {
        MockMarketConnector {
            current_ask: 0.,
            current_bid: 0.,
            asset_balance: 0,
            cash_balance: initial_cash_balance
        }
    }

    pub fn inject_snapshot(&mut self, snapshot: MarketSnapshot) {
        self.current_ask = snapshot.current_ask;
        self.current_bid = snapshot.current_bid;
    }

    pub fn inject_aggregate(&mut self, order: Aggregate) {
        todo!();
    }

    pub fn get_asset_balance(&self) -> usize {
        self.asset_balance
    }

    pub fn get_cash_balance(&self) -> f32 {
        self.cash_balance
    }
}

impl MarketConnector for MockMarketConnector {

    fn route_make_order(&self, order: &Order) -> bool {
        todo!()
    }

    fn route_take_order(&self, order: &Order) -> bool {
        todo!()
    }

    fn get_historical_snapshots(&self, start: chrono::DateTime<chrono::Utc>, end: chrono::DateTime<chrono::Utc>) -> Vec<MarketSnapshot> {
        panic!("This connector is only used for backtesting and can not provide historical data")
    }

    fn get_historical_aggregates(&self, start: chrono::DateTime<chrono::Utc>, end: chrono::DateTime<chrono::Utc>) -> Vec<Aggregate> {
        panic!("This connector is only used for backtesting and can not provide historical data")
    }
}