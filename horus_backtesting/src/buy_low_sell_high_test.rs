use horus_data_streams::{streams::mock_aggregate_stream::MockAggregateStream, models::time_series_element::TimeSeriesElement};
use horus_exchanges::mock_exchange::mock_market_connector::MockMarketConnector;
use horus_finance::aggregate::Aggregate;
use horus_strategies::strategies::buy_low_sell_high::BuyLowSellHighStrategy;

use crate::backtest_report::BacktestReport;

pub fn test_blsh_strategy(historical_data: Vec<TimeSeriesElement<Aggregate>>, initial_cash_balance: f32) -> BacktestReport {

    //1. Setup
    let mock_market = MockMarketConnector::new(initial_cash_balance);
    let mut strategy = BuyLowSellHighStrategy::<MockMarketConnector>::new(&mock_market);
    let mut on_data = |aggregate: Aggregate| {
        //update the mock exchange
        mock_market.set_price(aggregate.close, aggregate.close);

        //run the strategy
        strategy.next(aggregate);
    };
    let mut mock_stream = MockAggregateStream::new(&mut on_data);

    //2. Run
    for datum in historical_data {
        mock_stream.inject(datum);
    }

    //3. Report
    BacktestReport {
        alpha: (mock_market.get_cash_balance() / initial_cash_balance - 1.) * 100.
    }
}