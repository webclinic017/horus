use chrono::TimeZone;
use horus_backtesting::{run_backtest_on_aggregates, markets_event_simulation::{MarketsSnapshotEventSimulation, MarketsAggregateEventSimulation}, test_market_adapter::TestMarketAdapter, run_backtest_on_snapshots};
use horus_data_streams::streams::{binance_spot_aggregate_stream::BinanceSpotAggregateReceiver, data_stream::DataReceiver};
use horus_exchanges::mock_exchange::mock_market_connector::MockMarketConnector;
use horus_finance::{aggregate::Aggregate, market_snapshot::MarketSnapshot};
use horus_strategies::strategies::{inter_market_arbitrage::InterMarketArbitrageStrategy, buy_low_sell_high::BuyLowSellHighStrategy};

// fn test_twitter_strategy() {
    
// }

// fn test_inter_market_arbitrage_strategy() {

//     //1. Describe Strategy
//     let binance_spot = BinanceSpotAggregateReceiver::new("BTCEUR".to_string(), "5m".to_string(), &|_a| {});
//     let bitfinex_spot = BitfinexSpotMarketConnector::new("BTCEUR".to_string(), "5m".to_string(), &|_a| {});
//     let mock_market_01 = MockMarketConnector::new(1000.);
//     let mock_market_02 = MockMarketConnector::new(0.);
//     let stream_01 = MockMarketSnapshotDataReceiver::new();
//     let stream_02 = MockMarketSnapshotDataReceiver::new();
//     stream_01.add_listener(|snapshot: MarketSnapshot| { mock_market_01.inject_snapshot(snapshot) });
//     let test_adapter = TestMarketAdapter::new();
    
//     test_adapter.add_receiver(stream_01);
//     test_adapter.add_receiver(stream_02);
//     let strategy = InterMarketArbitrageStrategy::<MockMarketReceiver, MockMarketConnector>::new(&mock_market_01, &mock_market_02);

//     //2. Describe Simulation
//     let start_date = chrono::Utc.ymd(2015, 1, 1).and_hms(0, 0, 0);
//     let end_date = chrono::Utc::now();
//     let historical = binance_spot.get_historical_snapshots(start_date, end_date);
//     let simulation = MarketsSnapshotEventSimulation::new();
//     simulation.insert("market_01", historical);
//     let _result = run_backtest_on_snapshots(&strategy, simulation, &test_adapter);

//     //3. Report
//     todo!()
// }

// fn test_blsh_strategy() {

//     //1. Describe Strategy
//     let binance_spot = BinanceSpotAggregateReceiver::new("BTCEUR".to_string(), "5m".to_string(), &|_a| {});
//     let mock_market = MockMarketConnector::new(1000.);
//     let test_adapter = TestMarketAdapter::new();
//     test_adapter.add_market_connector("market_01", &mock_market);
//     let strategy = BuyLowSellHighStrategy::<MockMarketConnector>::new(&mock_market);

//     //2. Describe Simulation
//     let start_date = chrono::Utc.ymd(2015, 1, 1).and_hms(0, 0, 0);
//     let end_date = chrono::Utc::now();
//     let historical = binance_spot.get_historical_data(start_date, end_date);
//     let simulation = MarketsAggregateEventSimulation::new();
//     simulation.insert("market_01", historical);
//     let _result = run_backtest_on_aggregates(&strategy, simulation, &test_adapter);

//     //3. Report
//     todo!()
// }

fn main() {
    // test_blsh_strategy();
    // test_inter_market_arbitrage_strategy();
}