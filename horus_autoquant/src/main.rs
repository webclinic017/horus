mod reporters;

use chrono::{Duration, Utc};

use horus_data_streams::receivers::{binance_market_data_receiver::BinanceMarketDataReceiver, data_receiver::DataReceiver};
use horus_finance::AggregatedMarketData;
use horus_strategies::{Strategy, GoldenCrossStrategy};

use reporters::report_to_console;

pub struct StrategyPerformance<'a> {
    strategy: &'a dyn Strategy
}

fn find_best_strategy(market_data: &AggregatedMarketData, reporters: &Vec<&dyn Fn(&AggregatedMarketData, &StrategyPerformance) -> ()>) {
    let strategy = GoldenCrossStrategy { 
        first_sma: 20, 
        second_sma: 50 
    };

    let best_strategy = StrategyPerformance { strategy: &strategy };
    for r in reporters {
        r(&market_data, &best_strategy);
    }
}

fn main() {
    
    //1. Describe Market
    let receiver = BinanceMarketDataReceiver::new(String::from("BNBETH"), String::from("5m"));
    let end = Utc::now();
    let start = end.checked_sub_signed(Duration::days(1)).unwrap();
    let market_data: AggregatedMarketData = receiver.get_historical_data(start, end);

    //2. Describe Reporters
    let reporters: Vec<&dyn Fn(&AggregatedMarketData, &StrategyPerformance) -> ()> = vec!(&report_to_console);
    find_best_strategy(&market_data, &reporters);
}
