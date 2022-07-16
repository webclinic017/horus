use chrono::TimeZone;
use horus_backtesting::buy_low_sell_high_test::test_blsh_strategy;
use horus_data_streams::streams::{binance_futures_aggregate_stream::BinanceFuturesAggregateStream, data_stream::DataStream};

fn main() {
    let binance_spot = BinanceFuturesAggregateStream::new(&|_|{}, "BTCEUR".to_string(), "5m".to_string());
    let start = chrono::Utc.ymd(2015, 1, 1).and_hms(0, 0, 0);
    let end = chrono::Utc::now();
    let historical_data = binance_spot.get_historical_data(start, end);

    let report = test_blsh_strategy(historical_data, 1000.);

    println!("Alpha: {}%", report.alpha);
}
