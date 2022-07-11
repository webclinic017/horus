use crate::order_side::OrderSide;

pub struct Order {
    pub side: OrderSide,
    pub quantity: usize,
    pub price: Option<f32>,
    pub expiration_date: Option<String>
}