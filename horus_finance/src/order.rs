use crate::order_side::OrderSide;

pub struct Order {
    pub exchange_id: String,
    pub instrument_symbol: String,
    pub side: OrderSide,
    pub quantity: usize,
    pub price: Option<f32>,
    pub expiration_date: Option<String>
}