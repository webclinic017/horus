use chrono::TimeZone;
use horus_data_streams::{streams::{binance_futures_aggregate_stream::{BinanceFuturesAggregateStream}, data_stream::DataStream, mock_aggregate_stream::MockAggregateStream}, models::time_series_element::TimeSeriesElement};
use horus_exchanges::mock_exchange::mock_market_connector::MockMarketConnector;
use horus_finance::{aggregate::Aggregate, market_snapshot::MarketSnapshot};
use horus_strategies::strategies::{inter_market_arbitrage::InterMarketArbitrageStrategy, buy_low_sell_high::BuyLowSellHighStrategy};

// fn test_twitter_strategy() {
    
// }

fn test_inter_market_arbitrage_strategy() {

    //1. Describe Strategy
    // let binance_spot_snapshots = Arc::new(MarketSnapshotSequence::<16>::new());
    // let bitfinex_spot_snapshots = Arc::new(MarketSnapshotSequence::<16>::new());
    // let binance_spot = BinanceSpotMarketConnector::new("BTCEUR".to_string(), "5m".to_string(), &|_a| {});
    // let bitfinex_spot = BitfinexSpotMarketConnector::new("BTCEUR".to_string(), "5m".to_string(), &|_a| {});
    // let mock_market_01 = MockMarketConnector::new(1000.);
    // let mock_market_02 = MockMarketConnector::new(0.);
//     let historical_01 = get_historical_01();
//     let historical_02 = get_historical_02();
//     stream_01.add_listener(|snapshot: MarketSnapshot| { mock_market_01.inject_snapshot(snapshot) });
//     let test_adapter = TestMarketAdapter::new();
//     test_adapter.add_receiver(stream_01);
//     test_adapter.add_receiver(stream_02);
//     let simulation_elements_01 = build_simluation(&binance_spot, &mock_market_01, historical_01);
//     let simulation_elements_02 = build_simluation(&bitfinex_spot, &mock_market_02, historical_02);
//     let simulation = merge();
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
}

fn test_blsh_strategy(historical_data: Vec<TimeSeriesElement<Aggregate>>, initial_cash_balance: f32) {

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
    let alpha = (mock_market.get_cash_balance() / initial_cash_balance - 1.) * 100.;
    println!("Alpha: {}%", alpha);
}

fn main() {
    let binance_spot = BinanceFuturesAggregateStream::new(&|_|{}, "BTCEUR".to_string(), "5m".to_string());
    let start = chrono::Utc.ymd(2015, 1, 1).and_hms(0, 0, 0);
    let end = chrono::Utc::now();
    let historical_data = binance_spot.get_historical_data(start, end);

    test_blsh_strategy(historical_data, 1000.);

    // test_inter_market_arbitrage_strategy();
}
