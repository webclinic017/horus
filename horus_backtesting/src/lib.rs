use std::rc::Rc;

use horus_finance::{AggregatedMarketData, OrderSide};
use horus_strategies::Strategy;

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
        Self { profit_loss_rel: self.profit_loss_rel.clone(), profit_loss_abs: self.profit_loss_abs.clone(), alpha: self.alpha.clone() }
    }
}

impl Copy for BacktestResult {
    
}

pub fn run_backtest(market_data: &AggregatedMarketData, strategy: &Cell<dyn Strategy>) -> BacktestResult {

    let mut amount_asset: f32 = 0.;
    let mut amount_quote: f32 = market_data.aggregates[0].open;
    let mut current_side: OrderSide = OrderSide::HOLD;

    for aggregate in &market_data.aggregates {
        let orderside = strategy.next(&aggregate);
        match orderside {
            Some(side) => {
                if side != current_side {
                    match side {
                        OrderSide::SELL => {
                            amount_quote = amount_asset * aggregate.close;
                            amount_asset = 0.;
                            current_side = OrderSide::SELL;
                        }
                        OrderSide::HOLD => {
                            current_side = OrderSide::HOLD;
                         }
                        OrderSide::BUY => {
                            amount_asset = amount_quote * aggregate.close;
                            amount_quote = 0.;
                            current_side = OrderSide::BUY
                        }
                    }
                }
            }
            _ => {}
        } 
    }

    let buy_and_hold_rel: f32 = market_data.aggregates[market_data.aggregates.len() - 1].close / market_data.aggregates[0].open - 1.;
    let strategy_rel: f32 = amount_quote / market_data.aggregates[0].open - 1.;

    let alpha: f32 = strategy_rel / buy_and_hold_rel - 1.;

    BacktestResult { 
        profit_loss_rel: strategy_rel, 
        profit_loss_abs: amount_quote - market_data.aggregates[0].open,
        alpha
    }
}
