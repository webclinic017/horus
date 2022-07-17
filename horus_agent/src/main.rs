use horus_data_streams::streams::{binance_futures_aggregate_stream::BinanceFuturesAggregateStream, data_stream::DataStream};
use horus_exchanges::connectors::fake_market::FakeMarket;
use horus_finance::aggregate::Aggregate;
use horus_reporters::console_reporter::ConsoleReporter;
use horus_strategies::strategies::buy_low_sell_high::BuyLowSellHighStrategy;

fn main() {
    let market_connector = FakeMarket::new();
    let reporter = ConsoleReporter::new();
    let mut strategy = BuyLowSellHighStrategy::new(&market_connector, reporter);
    let mut on_data = |aggregate: Aggregate| { 
        strategy.next(aggregate); 
    };
    let mut binance = BinanceFuturesAggregateStream::new(&mut on_data, "BTCEUR".to_string(), "1m".to_string());
    binance.start_listening();
}