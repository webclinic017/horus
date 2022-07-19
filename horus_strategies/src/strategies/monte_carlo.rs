use horus_exchanges::connectors::market_connector::MarketConnector;
use horus_finance::{aggregate::Aggregate, order::Order, market_position::MarketPosition, order_side::OrderSide};
use horus_reporters::reporter::Reporter;
use rand::Rng;

use super::strategy::Strategy;

pub struct MonteCarloAggregates<Market: MarketConnector, Rep: Reporter> {
    pub market: Market,
    pub reporter: Rep,
    current_side: MarketPosition
 }

impl<Market: MarketConnector, Rep: Reporter> MonteCarloAggregates<Market, Rep> {
    pub fn new(market: Market, reporter: Rep) -> MonteCarloAggregates<Market, Rep> {
        MonteCarloAggregates {
            market,
            reporter,
            current_side: MarketPosition::NEUTRAL
        }
    }

    fn take_action(&self) -> bool {
        let ran = rand::thread_rng().gen_range(0..100);

        50 > ran
    }

    fn generate_fake_order(&self) -> Order {

        let lot = 10;

        if self.current_side == MarketPosition::LONG {
            Order {
                side: OrderSide::SELL,
                quantity: lot,
                price: None
            }
        } else {
            Order {
                side: OrderSide::BUY,
                quantity: lot,
                price: None
            }
        }
    }

    pub fn next(&mut self, _aggregate: Aggregate) {
        let take_action = self.take_action();

        if take_action {

            let order = self.generate_fake_order();

            let result = self.market.route_take_order(&order);

            if let Ok(position) = result {
                self.reporter.add_position(&position);
            } 
        }
    }
}

impl<Market: MarketConnector, Rep: Reporter> Strategy for MonteCarloAggregates<Market, Rep> {
    fn get_name() -> &'static str {
        "Monte Carlo Aggregates"
    }
}