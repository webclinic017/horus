use horus_finance::Order;

pub trait MarketConnector {
    fn route_order(&self, order: &Order) -> bool;
    fn get_historical_snapshots(&self, start: DateTime<Utc>, end: DateTime<Utc>) -> Vec<MarketSnapshot>;
    fn get_historical_aggregates(&self, start: DateTime<Utc>, end: DateTime<Utc>) -> Vec<Aggregate>;
}