use crate::Strategy;
use horus_data_streams::sequences::sequence::Sequence;
use horus_finance::{Aggregate, OrderSide};

struct GoldenCrossStrategyState {
    short_below_long: Option<bool>
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
            state: GoldenCrossStrategyState { short_below_long: None }
        }
    }
}

impl Strategy for GoldenCrossStrategy {
    fn next(& mut self, aggregate: &Aggregate) -> Option<OrderSide> {
        self.first_sma_seq.enqueue(aggregate);
        self.second_sma_seq.enqueue(aggregate);

        let first_sma_value = self.first_sma_seq.get_sma(&self.first_sma);
        let second_sma_value = self.first_sma_seq.get_sma(&self.second_sma);

        match first_sma_value {
            Some(f_val) => {
                match second_sma_value {
                    Some(s_val) => {
                        match self.state.short_below_long {
                            Some(state) => {
                                if f_val > s_val {
                                    self.state.short_below_long = Some(false);
                                    if state {
                                        Some(OrderSide::SELL)
                                    } else {
                                        Some(OrderSide::HOLD)
                                    }
                                    
                                } else {
                                    self.state.short_below_long = Some(true);
                                    if state {
                                        Some(OrderSide::HOLD)
                                    } else {
                                        Some(OrderSide::BUY)
                                    }
                                }
                            }
                            None => {
                                if f_val > s_val {
                                    self.state.short_below_long = Some(false);
                                } else {
                                    self.state.short_below_long = Some(true);
                                }
                                None
                            }
                        }
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
    matrix
}