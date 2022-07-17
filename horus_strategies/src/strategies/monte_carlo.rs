use horus_finance::aggregate::Aggregate;
use rand::Rng;

use super::strategy::Strategy;

pub struct MonteCarloAggregates { }

impl MonteCarloAggregates {
    pub fn next(&mut self, aggregate: Aggregate) {
        let num = rand::thread_rng().gen_range(0..100);
    }
}

impl Strategy for MonteCarloAggregates {
    fn get_name() -> &'static str {
        "Monte Carlo Aggregates"
    }
}