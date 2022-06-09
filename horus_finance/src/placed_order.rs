/// An order that has been placed in the order book
pub struct PlacedOrder {
    pub quantity: usize,
    pub price: f32,
    pub queue_position: usize
}