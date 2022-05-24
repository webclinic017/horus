use horus_finance::AggregatedMarketData;
use horus_strategies::Strategy;

pub struct BacktestResult {
    pub score: usize
}

impl PartialEq for BacktestResult {
    fn eq(&self, other: &Self) -> bool {
        self.score == other.score
    }
}

impl PartialOrd for BacktestResult {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.score.partial_cmp(&other.score)
    }
}

impl Clone for BacktestResult {
    fn clone(&self) -> Self {
        Self { score: self.score.clone() }
    }
}

impl Copy for BacktestResult {
    
}

pub fn run_backtest(_market_data: &AggregatedMarketData, _strategy: &dyn Strategy) -> BacktestResult {
    BacktestResult {
        score: 1
    }
}
