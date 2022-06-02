// mod reporters;

// use chrono::{Duration, Utc};

// use horus_backtesting::{run_backtest, BacktestResult, MarketSimulation};
// use horus_data_streams::receivers::{data_receiver::DataReceiver, mock_market_data_receiver::MockMarketDataReceiver};
// use horus_finance::AggregatedMarketData;
// use horus_strategies::strategies::strategy::Strategy;

// use reporters::report_to_console;

// fn get_strategy_collections() -> Vec<&'static dyn Fn() -> Vec<Box<dyn Strategy>>> {
//     let mut strategy_collections: Vec<&'static dyn Fn() -> Vec<Box<dyn Strategy>>> = Vec::<&'static dyn Fn() -> Vec<Box<dyn Strategy>>>::new();
//     let golden_cross_collection: &'static dyn Fn() -> Vec<Box<dyn Strategy>> = &golden_cross::generate_strategy_matrix;
//     strategy_collections.push(golden_cross_collection);
//     strategy_collections
// }

// fn find_best_strategy(market_data: &MarketSimulation) -> (Box<dyn Strategy>, BacktestResult) {

//     let mut best_result: Option<BacktestResult> = None;
//     let mut best_strategy: Option<Box<dyn Strategy>> = None;
//     let get_collection_methods = get_strategy_collections();

//     for get_collection_method in get_collection_methods {
//         let strategy_matrix: Vec<Box<dyn Strategy>> = get_collection_method();
//         for strategy in strategy_matrix {
//             let result = run_backtest<STRATEGY>(market_data, &strategy);
//             match best_result {
//                 Some(br) => {
//                     if result > br {
//                         best_result = Some(result);
//                         best_strategy = Some(strategy);
//                     }
//                 }
//                 None => {
//                     best_result = Some(result);
//                     best_strategy = Some(strategy);
//                 }
//             }
            
//         }
//     }

//     let best_strategy = best_strategy.unwrap();
//     let best_result = best_result.unwrap();
//     (best_strategy, best_result)
// }

fn main() {
    
    //1. Describe Market
    // let receiver = BinanceMarketDataReceiver::new(String::from("BNBETH"), String::from("5m"));
    // let receiver = MockMarketDataReceiver {

    // };
    // let end = Utc::now();
    // let start = end.checked_sub_signed(Duration::days(1)).unwrap();
    // let market_data: MarketSimulation = MarketSimulation {
    //     index: 0,
    //     market_data: receiver.get_historical_data(start, end)
    // };

    // //2. Describe Reporters
    // let reporters: Vec<&dyn Fn(&AggregatedMarketData, &Box<dyn Strategy>, &BacktestResult)> = vec!(&report_to_console);

    // //3. Run
    // let (best_strategy, best_result) = find_best_strategy(&market_data);
    // for r in reporters {
    //     r(&market_data, &best_strategy, &best_result);
    // }
}
