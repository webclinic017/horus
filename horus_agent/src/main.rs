use horus_data_streams::streams::{binance_futures_aggregate_stream::BinanceFuturesAggregateStream, data_stream::DataStream};
use horus_exchanges::{mock_exchange::mock_market_connector::MockMarketConnector, connectors::market_connector::MarketConnector};
use horus_finance::aggregate::Aggregate;
use horus_reporters::console_reporter::ConsoleReporter;
use horus_strategies::strategies::monte_carlo::MonteCarloAggregates;

fn main() {
    let mut market_connector = MockMarketConnector::new(1000.);
    market_connector.set_fake_exchange_name("BINANCE FUTURES");
    market_connector.set_fake_market_name("BTC/EUR");
    let reporter = ConsoleReporter::new(&market_connector.get_exchange_name(), &market_connector.get_market_name(), "€");
    let mut strategy = MonteCarloAggregates::new(market_connector, reporter);
    let mut on_data = |aggregate: Aggregate| { 
        strategy.next(aggregate); 
    };
    let mut binance = BinanceFuturesAggregateStream::new(&mut on_data, "BTCEUR".to_string(), "1m".to_string());
    binance.start_listening();
}