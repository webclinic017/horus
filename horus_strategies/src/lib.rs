pub mod golden_cross;

use horus_finance::{OrderSide, Aggregate};

pub trait Strategy {
    fn get_name(&self) -> &'static str;
    fn next(&self, aggregate: &Aggregate) -> OrderSide;
    fn print_values_to_console(&self);
}