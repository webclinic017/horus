fn run_inter_exchange_arbitrage_strategy() {
    
    //1. Describe Strategy
    let binance_spot = BinanceSpotExchange::new("BTCEUR");
    let munich_exchange = MunichExchange::new("BTCETF");
    let strategy = InterExchangeStrategy<BinanceSpotExchange, MunichExchange>::new(&binance_spot, &munich_exchange);

    //2. Describe Simulation
    let start_date: DateTime<UTC> = "";
    let end_date: DateTime<UTC> = "";
    let historical_01 = binance_spot.get_historical_data(start_date, end_date);
    let historical_02 = munich_exchange.get_historical_data(start_date, end_date);
    let simulate_next_tick = || {
        //big stuff todo
        false
    };

    //3. Describe Reporters
    let reporter = ConsoleReporter::new();

    //4. Run
    strategy.run<ConsoleReporter>(&console_reporter).join().unwrap();

    let result = strategy.get_backtest_result();
    for r in reporters {
        r(&market_data, &best_strategy, &best_result);
    }
}

fn main() {
}