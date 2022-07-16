pub fn test_inter_market_arbitrage_strategy() {

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