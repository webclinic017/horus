use horus_finance::AggregatedMarketData;

use crate::StrategyPerformance;
// use postgres::{self, Client, NoTls};

pub fn report_to_console(market_data: &AggregatedMarketData, strategy_performance: &StrategyPerformance) {
    println!("Analyzed market {} ({} interval) on exchange {}", market_data.market_name, market_data.aggregation_length, market_data.exchange_name);
    println!("Ran against {} aggregates ranging from {} to {}", market_data.aggregates.len(), market_data.start_time, market_data.end_time);
    println!("Best Strategy: {}", strategy_performance.strategy.get_name());
}

// pub fn report_to_database(market_data: &AggregatedMarketData, strategy_performance: &StrategyPerformance) {
//     todo!();
// }

// fn connect_to_db() -> Result<Client, postgres::Error> {
//     let host = "localhost";
//     let port = "5432";
//     let db_name = "Strategies";
//     let user = "root";
//     let password = "root";
//     let connection_string = format!("host={host} port={port} dbname={db_name} user={user} password={password}");
//     Client::connect(&connection_string, NoTls)
// }

// fn write_to_db(self, client: &Client) {
//     let insert_statement = "INSERT INTO strategy.name (created_on, exchange, market, first_sma, second_sma) VALUES ($1, $2, $3, $4, $5)";
//     let params;
//     let strategy_params = self.strategy.get_params();
//     params.add_range(strategy_params);
//     let insert_statement: &str = self.build_insert_statement(strategy_params);
//     client.execute(insert_statement, params).unwrap();
// }

// fn save_performance(strategy: &StrategyPerformance){
//     let mut client = connect_to_db().unwrap();
//     // client.batch_execute("
//     //     CREATE TABLE IF NOT EXISTS strategy.name (
//     //         id          SERIAL PRIMARY KEY,
//     //         created_on  TIMESTAMP NOT NULL,
//     //         exchange    TEXT NOT NULL,
//     //         market      TEXT NOT NULL,
//     //         first_sma   SMALLINT NOT NULL,
//     //         second_sma  SMALLINT NOT NULL
//     //     );
//     //     // ALTER TABLE strategies 
//     //     // ADD CONSTRAINT is_past_check 
//     //     // CHECK (
//     // 	//        created_on <= NOW
//     //     // ADD CONSTRAINT valid_sma_check 
//     //     // CHECK (
//     // 	//        AND first_sma >= 1
//     // 	//        AND second_sma >= 1
//     //     // );
//     // ").unwrap();
//     // client.execute(
//     //     "INSERT INTO strategy.name (created_on, exchange, market, first_sma, second_sma) VALUES ($1, $2, $3, $4, $5)",
//     //     &[&strategy.time_of_creation, &strategy.exchange_name, &strategy.market_name, &strategy.first_sma, &strategy.second_sma],
//     // ).unwrap();
//     //strategy.write_to_db(&client);
//     client.close().unwrap();
// }