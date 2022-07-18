use horus_data_streams::streams::{binance_futures_aggregate_stream::BinanceFuturesAggregateStream, data_stream::DataStream};
use horus_finance::aggregate::Aggregate;
use horus_reporters::console_reporter::ConsoleReporter;
use horus_strategies::strategies::monte_carlo::MonteCarloAggregates;

fn main() {
    let market_connector = MockMarketConnector::new();
    let reporter = ConsoleReporter::new();
    let mut strategy = MonteCarloAggregates::new(market_connector, reporter);
    let mut on_data = |aggregate: Aggregate| { 
        strategy.next(aggregate); 
    };
    let mut binance = BinanceFuturesAggregateStream::new(&mut on_data, "BTCEUR".to_string(), "1m".to_string());
    binance.start_listening();
}