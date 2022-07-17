use crate::reporter::Reporter;

pub struct VoidReporter {}

impl Reporter for VoidReporter {
    fn report(&self, _order: &horus_finance::order::Order) {
        
    }
}