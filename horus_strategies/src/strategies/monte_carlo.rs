use horus_exchanges::connectors::market_connector::MarketConnector;
use horus_finance::{aggregate::Aggregate, order_side::OrderSide, position::Position};
use horus_reporters::reporter::Reporter;
use rand::Rng;
use smartstring::alias::String;

use super::strategy::Strategy;

pub struct MonteCarloAggregates<Market: MarketConnector, Rep: Reporter> {
    pub market: Market,
    pub reporter: Rep
 }

impl<Market: MarketConnector, Rep: Reporter> MonteCarloAggregates<Market, Rep> {
    pub fn new(market: Market, reporter: Rep) -> MonteCarloAggregates<Market, Rep> {
        MonteCarloAggregates {
            market,
            reporter
        }
    }
    pub fn next(&mut self, aggregate: Aggregate) {
        let num = rand::thread_rng().gen_range(0..100);
        let side;

        if num >= 50 {
            side = OrderSide::BUY;
        } else {
            side = OrderSide::SELL;
        }

        let position = Position {
            exchange: self.market.get_exchange_name(),
            market: self.market.get_market_name(),
            quantity: 100 - num,
            buy_price: aggregate.close,
            sell_price: None
        };

        self.reporter.add_position(&position);
    }
}

impl<Market: MarketConnector, Rep: Reporter> Strategy for MonteCarloAggregates<Market, Rep> {
    fn get_name() -> &'static str {
        "Monte Carlo Aggregates"
    }
}