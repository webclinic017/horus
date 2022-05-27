use horus_finance::{Aggregate, AggregatedMarketData};

use super::data_receiver::DataReceiver;

pub struct MockMarketDataReceiver {}

impl DataReceiver for MockMarketDataReceiver {
    fn start_listening(&self, _on_data_receive: &dyn Fn()) {
        todo!()
    }

    fn get_historical_data(&self, start: chrono::DateTime<chrono::Utc>, end: chrono::DateTime<chrono::Utc>) -> horus_finance::AggregatedMarketData {
        let mut mock_data = Vec::<Aggregate>::new();

        mock_data.push(Aggregate {
            open: 1.,
            close: 2.
        });
        mock_data.push(Aggregate {
            open: 2.,
            close: 3.
        });

        AggregatedMarketData {
            aggregates: mock_data,
            aggregation_length: String::from("undefined"),
            exchange_name: String::from("MOCK_EXCHANGE"),
            market_name: String::from("MOCK_MARKET"),
            start_time: start,
            end_time: end
        }
    }
}