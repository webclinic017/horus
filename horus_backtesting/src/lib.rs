use horus_data_streams::streams::{data_stream::DataStream, mock_data_stream::MockDataStream};
use horus_finance::{aggregate::Aggregate, time_series_element::TimeSeriesElement};
use horus_strategies::strategies::strategy::Strategy;
use market_events::MarketNewAggregateEvent;
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

pub fn run_backtest_on_snapshots<STRATEGY: Strategy>(strategy: &STRATEGY, simulation: BacktestSimulation, streams: &Vec<MockDataStream<Box<dyn TimeSeriesElement>>>, adapter: &TestMarketAdapter) -> BacktestResult {

    // SETUP
    adapter.set_initial_state();

    for stream in streams {
        stream.add_middleware(&||{ strategy.run_hot_path() });
    }

    // RUN TESTS
    for event in simulation {
        // adapter.inject_snapshot(&event.target_key, event.snapshot);
        event.stream.inject(event.datum);
    }

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

// pub fn run_backtest_on_aggregates<STRATEGY: Strategy>(strategy: &STRATEGY, test_simulation: MarketsAggregateEventSimulation, adapter: &TestMarketAdapter) -> BacktestResult {

//     // SETUP
//     adapter.set_initial_state();

//     let strategy_handle = strategy.run();

//     // RUN TESTS
//     for event in test_simulation {
//         adapter.inject_aggregate(&event.market_key, event.aggregate);
//     }

//     strategy_handle.join().unwrap();

//     // END
//     adapter.finalize_positions();

//     let buy_and_hold_rel: f32 = adapter.get_buy_and_hold_relative();
//     let strategy_rel: f32 = adapter.get_strategy_relative();
//     let strategy_abs: f32 = adapter.get_strategy_absolute();

//     let alpha: f32 = strategy_rel / buy_and_hold_rel - 1.;

//     BacktestResult { 
//         profit_loss_rel: strategy_rel, 
//         profit_loss_abs: strategy_abs,
//         alpha
//     }
// }

mod market_events;
pub mod test_market_adapter;
pub mod simulation_element;
pub mod backtest_simulation;