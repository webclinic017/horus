use horus_finance::position::Position;

use crate::reporter::Reporter;

pub struct VoidReporter {}

impl Reporter for VoidReporter {
    fn add_position(&mut self, _order: &Position) {
        
    }
}