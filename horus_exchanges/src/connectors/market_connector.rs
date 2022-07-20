use chrono::{DateTime, Utc};
use horus_finance::{order::Order, market_snapshot::MarketSnapshot, aggregate::Aggregate, position::Position};

pub trait MarketConnector {
    fn route_make_order(&self, order: &Order) -> Result<Position, String>;
    fn route_take_order(&self, order: &Order) -> Result<Position, String>;
    fn get_historical_snapshots(&self, start: DateTime<Utc>, end: DateTime<Utc>) -> Vec<MarketSnapshot>;
    fn get_historical_aggregates(&self, start: DateTime<Utc>, end: DateTime<Utc>) -> Vec<Aggregate>;
    fn get_exchange_name(&self) -> String;
    fn get_market_name(&self) -> String;
    fn get_currency_symbol(&self) -> String;
}