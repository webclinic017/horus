use std::rc::Rc;

use horus_data_streams::streams::{data_stream::DataStream, mock_data_stream::MockDataStream};
use horus_finance::time_series_element::TimeSeriesElement;
use horus_strategies::strategies::strategy::Strategy;
use backtest_simulation::BacktestSimulation;
use test_market_adapter::TestMarketAdapter;

pub struct BacktestResult {
    pub profit_loss_rel: f32,
    pub profit_loss_abs: f32,
    pub alpha: f32
}

impl PartialEq for BacktestResult {
    fn eq(&self, other: &Self) -> bool {
        self.profit_loss_rel == other.profit_loss_rel && self.profit_loss_abs == other.profit_loss_abs && self.alpha == other.alpha
    }
}

impl PartialOrd for BacktestResult {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.profit_loss_rel.partial_cmp(&other.profit_loss_rel) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        match self.profit_loss_abs.partial_cmp(&other.profit_loss_abs) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        self.alpha.partial_cmp(&other.alpha)
    }
}

pub fn run_backtest<STRATEGY: Strategy>(strategy: &STRATEGY, simulation: &mut BacktestSimulation, streams: &Vec<MockDataStream<Box<dyn TimeSeriesElement>, 2>>, adapter: &TestMarketAdapter) -> BacktestResult {

    // SETUP
    adapter.set_initial_state();

    // for stream in streams {
    //     // let hot_path_ref = Rc::new(||{strategy.run_hot_path()});
    //     // let gg = Rc::downgrade(&hot_path_ref);
    //     adapter.
    //     stream.set_on_data(hot_path_ref);
    // }

    // RUN TESTS
    simulation.run();

    // END
    adapter.finalize_positions();

    let buy_and_hold_rel: f32 = adapter.get_buy_and_hold_relative();
    let strategy_rel: f32 = adapter.get_strategy_relative();
    let strategy_abs: f32 = adapter.get_strategy_absolute();

    let alpha: f32 = strategy_rel / buy_and_hold_rel - 1.;

    BacktestResult { 
        profit_loss_rel: strategy_rel, 
        profit_loss_abs: strategy_abs,
        alpha
    }
}

pub mod test_market_adapter;
pub mod simulation_element;
pub mod backtest_simulation;