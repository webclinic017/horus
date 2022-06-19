use std::collections::HashMap;

use chrono::{DateTime, Utc};
use horus_exchanges::{mock_exchange::mock_market_connector::MockMarketConnector, connectors::market_connector::MarketConnector};
use horus_finance::{aggregate::Aggregate, market_snapshot::MarketSnapshot};

pub struct TestMarketAdapter<'a> {
    markets: HashMap<String, &'a MockMarketConnector>
}

impl<'a> TestMarketAdapter<'a> {
    pub fn new() -> TestMarketAdapter<'a> {
        TestMarketAdapter {
            markets: HashMap::new()
        }
    }

    pub fn add_market_connector(&mut self, key: &str, market: &'a MockMarketConnector) {
        self.markets.insert(key.to_string(), market);
    }

    pub fn set_initial_state(&self) {
        todo!()
    }

    pub fn finalize_positions(&self) {
        todo!()
    }

    pub fn get_buy_and_hold_relative(&self) -> f32 {
        todo!()
    }

    pub fn get_strategy_relative(&self) -> f32 {
        todo!()
    }

    pub fn get_strategy_absolute(&self) -> f32 {
        todo!()
    }
}

impl MarketConnector for TestMarketAdapter<'_> {
    fn route_make_order(&self, order: &horus_finance::order::Order) -> bool {
        todo!()
    }

    fn route_take_order(&self, order: &horus_finance::order::Order) -> bool {
        todo!()
    }

    fn get_historical_snapshots(&self, _start: DateTime<Utc>, _end: DateTime<Utc>) -> Vec<MarketSnapshot> {
        panic!("The test adapter does not provide historical data")
    }

    fn get_historical_aggregates(&self, _start: DateTime<Utc>, _end: DateTime<Utc>) -> Vec<Aggregate> {
        panic!("The test adapter does not provide historical data")
    }
}