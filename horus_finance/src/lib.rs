#[derive(Debug)]

/// An aggregated time span of financial market data.
pub struct Aggregate {
    pub open: f32,
    pub close: f32
}

impl Clone for Aggregate {
    fn clone(&self) -> Self {
        Self { open: self.open.clone(), close: self.close.clone() }
    }
}

#[derive(Debug)]
/// The type of the order.
pub enum OrderSide {
    SELL,
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