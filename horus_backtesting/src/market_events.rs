use horus_finance::{aggregate::Aggregate, market_snapshot::MarketSnapshot};

pub struct MarketNewAggregateEvent {
    pub market_key: String,
    pub aggregate: Aggregate
}

pub struct MarketNewSnapshotEvent {
    pub market_key: String,
    pub snapshot: MarketSnapshot
}