use std::cell::RefCell;

use crate::Strategy;
use horus_data_streams::sequences::sequence::Sequence;
use horus_finance::{Aggregate, OrderSide};

pub struct GoldenCrossStrategy<const SHORT_MA: usize, const LONG_MA: usize> {
    short_moving_average_seq: Sequence<Aggregate, SHORT_MA>,
    long_moving_average_seq: Sequence<Aggregate, LONG_MA>,
    short_moving_avg_sum: RefCell<f32>,
    long_moving_avg_sum: RefCell<f32>,
    short_below_long: RefCell<Option<bool>>
}

impl<const SHORT_MA: usize, const LONG_MA: usize> GoldenCrossStrategy<SHORT_MA, LONG_MA> {
    fn new() -> GoldenCrossStrategy<SHORT_MA, LONG_MA> {
        GoldenCrossStrategy { 
            short_moving_average_seq: Sequence::<Aggregate, SHORT_MA>::new(),
            long_moving_average_seq: Sequence::<Aggregate, LONG_MA>::new(),
            short_moving_avg_sum: RefCell::new(0.),
            long_moving_avg_sum: RefCell::new(0.),
            short_below_long: RefCell::new(None)
        }
    }
}

impl<const SHORT_MA: usize, const LONG_MA: usize> Strategy for GoldenCrossStrategy<SHORT_MA, LONG_MA> {
    fn next(&self, aggregate: &Aggregate) -> Option<OrderSide> {
        let mut short_sum_borrowed = self.short_moving_avg_sum.borrow_mut();
        let mut long_sum_borrowed = self.long_moving_avg_sum.borrow_mut();

        let first_moving_average_value = self.short_moving_average_seq.enqueue_for_moving_average(aggregate, &mut short_sum_borrowed);
        let second_moving_average_value = self.long_moving_average_seq.enqueue_for_moving_average(aggregate, &mut long_sum_borrowed);

        match first_moving_average_value {
            Some(f_val) => {
                match second_moving_average_value {
                    Some(s_val) => {
                        let next_short_below_long: bool = f_val < s_val;
                        let mut side_to_return: Option<OrderSide> = None;
                        {
                            let tmp_ref = self.short_below_long.borrow();
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
                        let mut borrowed_ref = self.short_below_long.borrow_mut();
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
        println!("Short MA: {}", self.short_moving_average_seq.get_length());
        println!("Long MA: {}", self.long_moving_average_seq.get_length());
    }
}

pub fn generate_strategy_matrix() -> Vec<Box<dyn Strategy>> {
    
    let strategy_01 = GoldenCrossStrategy::<50, 200>::new();
    let strategy_02 = GoldenCrossStrategy::<20, 60>::new();
    let mut matrix: Vec<Box<dyn Strategy>> = Vec::<Box<dyn Strategy>>::new();
    matrix.push(Box::new(strategy_01));
    matrix.push(Box::new(strategy_02));
    matrix
}