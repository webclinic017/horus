use horus_finance::aggregate::Aggregate;
use horus_strategies::strategies::strategy::Strategy;
use test_market_adapter::TestMarketAdapter;

// pub struct MarketSimulation {
//     index: usize,
//     pub market_data: Vec<Aggregate>
// }

// impl MarketSimulation {

//     fn get_current_bid(&self) -> f32 {
//         self.market_data[self.index].close
//     }

//     fn get_current_ask(&self) -> f32 {
//         self.market_data[self.index].close
//     }
// }

// impl Iterator for MarketSimulation {
//     type Item = Aggregate;

//     fn next(&mut self) -> Option<Self::Item> {

//         if self.market_data.len() > self.index {
//             self.index += 1;
//             Some(self.market_data[self.index - 1])
//         } else {
//             None
//         }
//     }
// }

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

// impl Clone for BacktestResult {
//     fn clone(&self) -> Self {
//         Self { profit_loss_rel: self.profit_loss_rel, profit_loss_abs: self.profit_loss_abs, alpha: self.alpha }
//     }
// }

// impl Copy for BacktestResult { }

/// The mock market has to be connected to the strategy
pub fn run_backtest_on_aggregates<STRATEGY: Strategy>(strategy: &STRATEGY, test_simulation: Vec<Aggregate>, adapter: &TestMarketAdapter) -> BacktestResult {

    // SETUP
    adapter.set_initial_state();

    let strategy_handle = strategy.run();

    // RUN TESTS
    for aggregate in test {
        adapter.inject_aggregate(*aggregate);
    }

    strategy_handle.join().unwrap();

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