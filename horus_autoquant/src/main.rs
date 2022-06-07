use chrono::DateTime;

fn test_inter_market_arbitrage_strategy() {

    //1. Describe Strategy
    let binance_spot = BinanceSpotOrderBookReceiver::new("BTCEUR");
    let munich_exchange = MunichExchangeOrderBookReceiver::new("BTCETF");
    let fake_exchange_01 = MockExchangeConnector::new();
    let fake_exchange_02 = MockExchangeConnector::new();
    let strategy = InterMarketArbitrageStrategy<MockExchangeConnector, MockExchangeConnector>::new(&fake_exchange_01, &fake_exchange_02);

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
    let reporter = BacktestEventReporter::new();
    let result_reporter = BacktestResultReporter::new();

    //4. Run
    strategy.run<BacktestEventReporter>().join().unwrap();

    while !simulate_next_tick() {}

    let exchange_01_balance = fake_exchange_01.get_balance();
    let exchange_02_balance = fake_exchange_02.get_balance();

    let result = build_backtest_result(&exchange_01_balance, &exchange_02_balance);

    reporter.report(result);
}

fn main() {

}
