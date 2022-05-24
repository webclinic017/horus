use chrono::{DateTime, Utc};

/// An aggregated time span of financial market data.
#[derive(Debug)]

pub struct Aggregate {
    pub open: f32,
    pub close: f32
}

impl Clone for Aggregate {
    fn clone(&self) -> Self {
        Self { open: self.open.clone(), close: self.close.clone() }
    }
}

/// A fixed amount of aggregated market data. All aggregates must have the same length
/// and must not contain any gaps.
#[derive(Debug)]
pub struct AggregatedMarketData {
    pub aggregates: Vec<Aggregate>,
    pub exchange_name: String,
    pub market_name: String,
    pub aggregation_length: String,
    pub start_time: DateTime<Utc>,
    pub end_time: DateTime<Utc>
}

/// The type of the order.
#[derive(Debug)]
#[derive(PartialEq)]
pub enum OrderSide {
    SELL,
    HOLD,
    BUY
}

pub struct Position {
    pub quantity: f32
}

pub struct Order {
    pub instrument_symbol: String,
    pub side: OrderSide,
    pub quantity: f32,
    pub price: f32,
    pub expiration_date: String
}