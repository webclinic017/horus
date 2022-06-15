use chrono::TimeZone;
use horus_backtesting::run_backtest_on_aggregates;
use horus_data_streams::receivers::{binance_market_data_receiver::BinanceSpotAggregateReceiver, data_receiver::DataReceiver};
use horus_exchanges::mock_exchange::mock_market_connector::MockMarketConnector;
use horus_strategies::strategies::{inter_market_arbitrage::InterMarketArbitrageStrategy, buy_low_sell_high::BuyLowSellHighStrategy};

// fn test_inter_market_arbitrage_strategy() {

//     // //1. Describe Strategy
//     let binance_spot = BinanceSpotAggregateReceiver::new("BTCEUR".to_string(), "5m".to_string(), &|_a| {});
//     let fake_market_01 = MockMarketConnector::new(1000.);
//     let fake_market_02 = MockMarketConnector::new(1000.);
//     let strategy = InterMarketArbitrageStrategy::<MockMarketConnector, MockMarketConnector>::new(&fake_market_01, &fake_market_02);

//     //2. Describe Simulation
//     let start_date = chrono::Utc.ymd(2015, 1, 1).and_hms(0, 0, 0);
//     let end_date = chrono::Utc::now();
//     let historical = binance_spot.get_historical_data(start_date, end_date);
//     let _result = run_backtest_on_aggregates(&strategy, &historical, &fake_market);

//     //3. Report
//     todo!()
// }

fn test_blsh_strategy() {

    //1. Describe Strategy
    let binance_spot = BinanceSpotAggregateReceiver::new("BTCEUR".to_string(), "5m".to_string(), &|_a| {});
    let fake_market = MockMarketConnector::new(1000.);
    let strategy = BuyLowSellHighStrategy::<MockMarketConnector>::new(&fake_market);

    //2. Describe Simulation
    let start_date = chrono::Utc.ymd(2015, 1, 1).and_hms(0, 0, 0);
    let end_date = chrono::Utc::now();
    let historical = binance_spot.get_historical_data(start_date, end_date);
    let _result = run_backtest_on_aggregates(&strategy, &historical, &fake_market);

    //3. Report
    todo!()
}

fn main() {
    test_blsh_strategy();
    test_inter_market_arbitrage_strategy();
}