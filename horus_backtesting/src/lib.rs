use horus_data_streams::{receivers::data_receiver::DataReceiver};
use horus_finance::{Order, MarketPosition, Aggregate};
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
            return Some(self.market_data[self.index - 1]);
        } else {
            return None;
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
        Self { profit_loss_rel: self.profit_loss_rel.clone(), profit_loss_abs: self.profit_loss_abs.clone(), alpha: self.alpha.clone() }
    }
}

impl Copy for BacktestResult {
    
}

fn validate_order(order: &Order) {
    match order.price {
        Some(_) => panic!("Backtesting is currently only available for market order"),
        _ => {}
    }
    match order.expiration_date {
        Some(_) => panic!("Backtesting is currently only available for market order"),
        _ => {}
    }
}

pub fn run_backtest<STRATEGY: Strategy, MARKET: DataReceiver<Aggregate>>(strategy: &STRATEGY, markets: &mut Vec<MarketSimulation>, simulated_market: &MARKET) -> BacktestResult {

    if markets.len() != 1 {
        panic!("Backtesting is currently only available for single markets");
    }

    let market = &mut markets[0];

    let initial_ask: f32 = market.get_current_ask();
    let mut amount_asset: f32 = 0.;
    let mut amount_quote: f32 = 1000.;
    let mut current_side: MarketPosition = MarketPosition::NEUTRAL;

    let backtest_handler = |orders: Vec<Order>| {

        let amount_orders = orders.len();
        
        match amount_orders {
            0 => {},
            1 => {
                let order = &orders[0];
                validate_order(&order);
                if order.side != current_side {
                    match order.side {
                        MarketPosition::SHORT => {
                            amount_quote = amount_asset * market.get_current_bid();
                            amount_asset = 0.;
                            current_side = MarketPosition::SHORT;
                        }
                        MarketPosition::NEUTRAL => {
                            current_side = MarketPosition::NEUTRAL;
                        }
                        MarketPosition::LONG => {
                            amount_asset = amount_quote * market.get_current_ask();
                            amount_quote = 0.;
                            current_side = MarketPosition::LONG
                        }
                    }
                }
            },
            _ => panic!("Backtesting is currently only available for single markets")
        }
    };

    let strategy_handle = strategy.run(backtest_handler);

    for aggregate in market.into_iter() {
        simulated_market.inject(aggregate);
    }

    strategy_handle.join().unwrap();

    let buy_and_hold_rel: f32 = market.get_current_bid() / initial_ask;
    let strategy_rel: f32 = amount_quote / initial_ask;

    let alpha: f32 = strategy_rel / buy_and_hold_rel - 1.;

    BacktestResult { 
        profit_loss_rel: strategy_rel, 
        profit_loss_abs: amount_quote - initial_ask,
        alpha
    }
}
