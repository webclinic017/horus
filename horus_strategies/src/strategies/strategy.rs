use std::thread::JoinHandle;

use horus_exchanges::connectors::market_connector::MarketConnector;

pub trait Strategy {
    fn run_hot_path(&self);
    fn get_name() -> &'static str;
}