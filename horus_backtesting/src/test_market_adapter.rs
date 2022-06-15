use std::collections::HashMap;

use horus_exchanges::mock_exchange::mock_market_connector::MockMarketConnector;
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

    pub fn add_market_connector(&mut self, market_key: &str, market: &'a MockMarketConnector) {
        self.markets.insert(market_key.to_string(), market);
    }

    pub fn inject_aggregate(&self, market_key: &str, aggregate: Aggregate) {
        let market = self.markets.get(market_key).unwrap();
        market.inject_aggregate(aggregate);
    }

    pub fn inject_snapshot(&self, market_key: &str, snapshot: MarketSnapshot) {
        let market = self.markets.get(market_key).unwrap();
        market.inject_snapshot(snapshot);
    }

    pub fn set_initial_state(&self) {
        todo!()
    }

    pub fn finalize_positions(&self) {
        todo!()
    }

    pub fn get_buy_and_hold_relative(&self) {
        todo!()
    }

    pub fn get_strategy_relative(&self) {
        todo!()
    }

    pub fn get_strategy_absolute(&self) {
        todo!()
    }
}

