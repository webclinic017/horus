use chrono::{Duration, Utc};

use horus_data_streams::receivers::{binance_market_data_receiver::BinanceMarketDataReceiver, data_receiver::DataReceiver};
use horus_finance::Aggregate;
use horus_strategies::{Strategy, GoldenCrossStrategy};
use postgres::{self, Client, NoTls};

struct StrategyPerformance<'a> {
    strategy: &'a dyn Strategy,
    time_of_creation: String,
    exchange_name: String,
    market_name: String
}

impl<'a> StrategyPerformance<'a> {
    // fn new(strategy: &'a dyn Strategy, time_of_creation: String, exchange_name: String, market_name: String) -> StrategyPerformance<'a> { 
    //     StrategyPerformance {
    //         strategy,
    //         time_of_creation,
    //         exchange_name, 
    //         market_name
    //     }
    // }
    fn build_params(self) {}
    fn write_to_db(self, client: &Client) {
        let insert_statement = "INSERT INTO strategy.name (created_on, exchange, market, first_sma, second_sma) VALUES ($1, $2, $3, $4, $5)";
        // let params;
        // let strategy_params = self.strategy.get_params();
        //params.add_range(strategy_params);
        //let insert_statement: &str = self.build_insert_statement(strategy_params);
        //client.execute(insert_statement, params).unwrap();
    }
}

fn find_and_store_best_strategy(time_series: &Vec<Aggregate>) {
    let strategy = GoldenCrossStrategy { 
        first_sma: 20, 
        second_sma: 50 
    };

    let best_strategy = StrategyPerformance { strategy: &strategy, time_of_creation: "2022-05-18".to_string(), exchange_name: "EUREX".to_string(), market_name: "FDAX".to_string() };
    save_performance(&best_strategy);
}

fn connect_to_db() -> Result<Client, postgres::Error> {
    let host = "localhost";
    let port = "5432";
    let db_name = "Strategies";
    let user = "root";
    let password = "root";
    let connection_string = format!("host={host} port={port} dbname={db_name} user={user} password={password}");
    Client::connect(&connection_string, NoTls)
}

fn save_performance(strategy: &StrategyPerformance){
    let mut client = connect_to_db().unwrap();
    // client.batch_execute("
    //     CREATE TABLE IF NOT EXISTS strategy.name (
    //         id          SERIAL PRIMARY KEY,
    //         created_on  TIMESTAMP NOT NULL,
    //         exchange    TEXT NOT NULL,
    //         market      TEXT NOT NULL,
    //         first_sma   SMALLINT NOT NULL,
    //         second_sma  SMALLINT NOT NULL
    //     );
    //     // ALTER TABLE strategies 
    //     // ADD CONSTRAINT is_past_check 
    //     // CHECK (
    // 	//        created_on <= NOW
    //     // ADD CONSTRAINT valid_sma_check 
    //     // CHECK (
    // 	//        AND first_sma >= 1
    // 	//        AND second_sma >= 1
    //     // );
    // ").unwrap();
    // client.execute(
    //     "INSERT INTO strategy.name (created_on, exchange, market, first_sma, second_sma) VALUES ($1, $2, $3, $4, $5)",
    //     &[&strategy.time_of_creation, &strategy.exchange_name, &strategy.market_name, &strategy.first_sma, &strategy.second_sma],
    // ).unwrap();
    //strategy.write_to_db(&client);
    client.close().unwrap();
}

fn main() {
    
    let receiver = BinanceMarketDataReceiver::new(String::from("BNBETH"), String::from("5m"));
    let end = Utc::now();
    let start = end.checked_add_signed(Duration::days(1)).unwrap();
    let time_series = receiver.get_historical_data(start, end);
    find_and_store_best_strategy(&time_series);
}
