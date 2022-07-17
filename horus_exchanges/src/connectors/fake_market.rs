use core::panic;

use super::market_connector::MarketConnector;

pub struct FakeMarket {}

impl FakeMarket {
    pub fn new() -> FakeMarket {
        FakeMarket {  }
    }
}

impl MarketConnector for FakeMarket {
    fn route_make_order(&self, _order: &horus_finance::order::Order) -> bool {
        true
    }

    fn route_take_order(&self, _order: &horus_finance::order::Order) -> bool {
        true
    }

    fn get_historical_snapshots(&self, _start: chrono::DateTime<chrono::Utc>, _end: chrono::DateTime<chrono::Utc>) -> Vec<horus_finance::market_snapshot::MarketSnapshot> {
        panic!("This method is not available at this market")
    }

    fn get_historical_aggregates(&self, _start: chrono::DateTime<chrono::Utc>, _end: chrono::DateTime<chrono::Utc>) -> Vec<horus_finance::aggregate::Aggregate> {
        panic!("This method is not available at this market")
    }
}