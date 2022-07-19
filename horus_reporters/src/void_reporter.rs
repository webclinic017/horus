use horus_finance::position::Position;

use crate::reporter::Reporter;

pub struct VoidReporter {}

impl Reporter for VoidReporter {
    fn update_cash_balance(&mut self, _new_cash_balance: f32) {}
    fn update_position(&mut self, _order: &Position) {}
}