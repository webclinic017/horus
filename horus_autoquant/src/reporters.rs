// use horus_backtesting::BacktestResult;
// use horus_finance::AggregatedMarketData;
// use horus_strategies::strategies::strategy::Strategy;

// pub fn report_to_console(market_data: &AggregatedMarketData, strategy: &dyn Strategy, _backtest_result: &BacktestResult) {
//     println!("Analyzed market {} ({} interval) on exchange {}", market_data.market_name, market_data.aggregation_length, market_data.exchange_name);
//     println!("Ran against {} aggregates ranging from {} to {}", market_data.aggregates.len(), market_data.start_time, market_data.end_time);
//     println!("Best Strategy: {}", strategy.get_name());
//     strategy.print_values_to_console();
// }