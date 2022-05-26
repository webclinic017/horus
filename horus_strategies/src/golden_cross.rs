use std::cell::RefCell;

use crate::Strategy;
use horus_data_streams::sequences::sequence::Sequence;
use horus_finance::{Aggregate, OrderSide};

struct GoldenCrossStrategyState {
    short_below_long: RefCell<Option<bool>>
}

pub struct GoldenCrossStrategy {
    pub first_sma: u16,
    pub second_sma: u16,
    first_sma_seq: Sequence<Aggregate>,
    second_sma_seq: Sequence<Aggregate>,
    state: GoldenCrossStrategyState
}

impl GoldenCrossStrategy {
    fn new(shorter: u16, longer: u16) -> GoldenCrossStrategy {
        GoldenCrossStrategy { 
            first_sma: shorter, 
            second_sma: longer,
            first_sma_seq: Sequence::<Aggregate>::new(),
            second_sma_seq: Sequence::<Aggregate>::new(),
            state: GoldenCrossStrategyState { short_below_long: RefCell::new(None) }
        }
    }
}

impl Strategy for GoldenCrossStrategy {
    fn next(&self, aggregate: &Aggregate) -> Option<OrderSide> {
        self.first_sma_seq.enqueue(aggregate);
        self.second_sma_seq.enqueue(aggregate);

        let first_sma_value = self.first_sma_seq.get_sma(&self.first_sma);
        let second_sma_value = self.first_sma_seq.get_sma(&self.second_sma);

        match first_sma_value {
            Some(f_val) => {
                match second_sma_value {
                    Some(s_val) => {
                        let next_short_below_long: bool = f_val < s_val;
                        let mut side_to_return: Option<OrderSide> = None;
                        {
                            let tmp_ref = self.state.short_below_long.borrow();
                            match *tmp_ref {
                                Some(s) => { 
                                    let last_short_below_long = s;
                                    if next_short_below_long && !last_short_below_long {
                                        side_to_return = Some(OrderSide::SELL);
                                    } 
                                    if !next_short_below_long && last_short_below_long {
                                        side_to_return = Some(OrderSide::BUY);
                                    }
                                }
                                None => {
                                    side_to_return = None;
                                }
                            }
                        }
                        let mut borrowed_ref = self.state.short_below_long.borrow_mut();
                        *borrowed_ref = Some(next_short_below_long);
                        side_to_return
                    }
                    None => {
                        None
                    }
                }
            }
            None => {
                None
            }
        }
    }
    fn get_name(&self) -> &'static str {
        "GOLDEN_CROSS"
    }
    fn print_values_to_console(&self) {
        println!("First SMA: {}", self.first_sma);
        println!("Second SMA: {}", self.second_sma);
    }
}

pub fn generate_strategy_matrix() -> Vec<Box<dyn Strategy>> {
    
    let strategy_01 = GoldenCrossStrategy::new(50, 200);
    let strategy_02 = GoldenCrossStrategy::new(20, 60);
    let mut matrix: Vec<Box<dyn Strategy>> = Vec::<Box<dyn Strategy>>::new();
    matrix.push(Box::new(strategy_01));
    matrix.push(Box::new(strategy_02));
    matrix
}