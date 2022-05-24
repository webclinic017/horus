use crate::Strategy;
use horus_finance::{Aggregate, OrderSide};

pub struct GoldenCrossStrategy {
    pub first_sma: u32,
    pub second_sma: u32,
}

impl Strategy for GoldenCrossStrategy {
    fn next(&self, _aggregate: &Aggregate) -> OrderSide {
        todo!();
    }
    fn get_name(&self) -> &'static str {
        "GOLDEN_CROSS"
    }
    fn print_values_to_console(&self) {
        println!("First SMA: {}", self.first_sma);
        println!("Second SMA: {}", self.second_sma);
    }
}

pub fn generate_strategy_matrix() -> Vec<&'static dyn Strategy> {
    
    let strategy_01: &'static GoldenCrossStrategy = &GoldenCrossStrategy { 
        first_sma: 20, 
        second_sma: 50 
    };
    let mut matrix: Vec<&'static dyn Strategy> = Vec::<&'static dyn Strategy>::new();
    matrix.push(strategy_01);
    matrix
}