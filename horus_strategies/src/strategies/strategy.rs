use std::thread::JoinHandle;

use horus_exchanges::connectors::market_connector::MarketConnector;

pub trait Strategy {
    fn run(&self) -> JoinHandle<()>;
    fn get_name() -> &'static str;
}