use horus_data_streams::{receivers::data_receiver::DataReceiver};
use horus_exchanges::{connectors::market_connector::MarketConnector, mock_exchange::mock_market_connector::MockMarketConnector};
use horus_finance::{order::Order, market_position::MarketPosition, aggregate::Aggregate, order_side::OrderSide};
use horus_strategies::strategies::strategy::Strategy;

pub struct MarketSimulation {
    index: usize,
    pub market_data: Vec<Aggregate>
}

impl MarketSimulation {

    fn get_current_bid(&self) -> f32 {
        self.market_data[self.index].close
    }

    fn get_current_ask(&self) -> f32 {
        self.market_data[self.index].close
    }
}

impl Iterator for MarketSimulation {
    type Item = Aggregate;

    fn next(&mut self) -> Option<Self::Item> {

        if self.market_data.len() > self.index {
            self.index += 1;
            Some(self.market_data[self.index - 1])
        } else {
            None
        }
    }
}

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

impl Clone for BacktestResult {
    fn clone(&self) -> Self {
        Self { profit_loss_rel: self.profit_loss_rel, profit_loss_abs: self.profit_loss_abs, alpha: self.alpha }
    }
}

impl Copy for BacktestResult {
    
}

fn validate_order(order: &Order) {
    if order.price.is_some() { panic!("Backtesting is currently only available for market order") }
    if order.expiration_date.is_some() { panic!("Backtesting is currently only available for market order") }
}

pub fn run_backtest<STRATEGY: Strategy>(strategy: &STRATEGY, test: &Vec<Aggregate>, mock_market: &mut MockMarketConnector) -> BacktestResult {

    // let markets = strategy.get_market_connectors();

    // if markets.len() != 1 {
    //     panic!("Backtesting is currently only available for single markets");
    // }

    // let market = markets[0];

    let initial_ask: f32 = test[0].close;
    let mut amount_quote: f32 = 1000.;
    let mut current_side: MarketPosition = MarketPosition::NEUTRAL;

    let strategy_handle = strategy.run();

    for aggregate in test {
        mock_market.inject_aggregate(*aggregate);
    }

    strategy_handle.join().unwrap();

    let buy_and_hold_rel: f32 = test[test.len() - 1].close / initial_ask;
    let strategy_rel: f32 = amount_quote / initial_ask;

    let alpha: f32 = strategy_rel / buy_and_hold_rel - 1.;

    BacktestResult { 
        profit_loss_rel: strategy_rel, 
        profit_loss_abs: amount_quote - initial_ask,
        alpha
    }
}

// pub fn run<STRATEGY: Strategy>(strategy: &STRATEGY) {

//     let sequences
// }