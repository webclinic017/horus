#[derive(Clone, Copy)]
pub struct Position {
    pub quantity: usize,
    pub buy_price: f32,
    pub sell_price: Option<f32>
}