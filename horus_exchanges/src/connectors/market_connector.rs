use chrono::{DateTime, Utc};
use horus_finance::{order::Order, market_snapshot::MarketSnapshot, aggregate::Aggregate};

pub trait MarketConnector {
    fn route_make_order(&self, order: &Order) -> bool;
    fn route_take_order(&self, order: &Order) -> bool;
    fn get_historical_snapshots(&self, start: DateTime<Utc>, end: DateTime<Utc>) -> Vec<MarketSnapshot>;
    fn get_historical_aggregates(&self, start: DateTime<Utc>, end: DateTime<Utc>) -> Vec<Aggregate>;
}