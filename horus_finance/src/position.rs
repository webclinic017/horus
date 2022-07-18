use smartstring::alias::String;

#[derive(Clone, Copy)]
pub struct Position {
    pub exchange: &'static String,
    pub market: &'static String,
    pub quantity: usize,
    pub buy_price: f32,
    pub sell_price: Option<f32>
}