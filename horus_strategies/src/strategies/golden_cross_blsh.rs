use crate::signals::golden_cross::GoldenCrossSignal;

use super::strategy::Strategy;

struct GoldenCrossBLSH<const SHORT_MA: usize, const LONG_MA: usize> {
    signal: GoldenCrossSignal<SHORT_MA, LONG_MA>
}

impl<const SHORT_MA: usize, const LONG_MA: usize> Strategy for GoldenCrossBLSH<SHORT_MA, LONG_MA> {
    fn run<HANDLER: FnMut(Vec<horus_finance::Order>)>(&self, order_handler: HANDLER) -> std::thread::JoinHandle<()> {
        todo!()
    }

    fn get_name() -> &'static str {
        return "GoldenCrossBLSH";
    }

    fn get_strategy_matrix() -> Vec<Self> where Self: Sized {
        todo!()
    }
}