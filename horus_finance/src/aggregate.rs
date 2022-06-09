/// An aggregated time span of financial market data.
#[derive(Debug)]

pub struct Aggregate {
    pub open: f32,
    pub close: f32
}

impl Clone for Aggregate {
    fn clone(&self) -> Self {
        Self { open: self.open, close: self.close }
    }
}

impl Copy for Aggregate {
    
}