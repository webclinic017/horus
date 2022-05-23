use std::sync::atomic::AtomicBool;

use horus_finance::{Aggregate, AggregatedMarketData};
use binance::{websockets::*, market::Market, api::Binance};

use super::data_receiver::DataReceiver;

pub struct BinanceMarketDataReceiver {
    market: Market,
    symbol: String,
    interval: String
}

impl BinanceMarketDataReceiver {
    pub fn new(symbol: String, interval: String) -> BinanceMarketDataReceiver {
        BinanceMarketDataReceiver {
            market: Binance::new(None, None),
            symbol,
            interval
        }
    }
}

impl DataReceiver for BinanceMarketDataReceiver {
    fn start_listening(&self, on_data_receive: &dyn Fn()) {
            let keep_running = AtomicBool::new(true);
            let mut web_socket: WebSockets = WebSockets::new(|event: WebsocketEvent| {
                match event {
                    WebsocketEvent::Kline(_) => { //kline_event) => {
                        // let new_aggregate = Aggregate {
                        //     open: kline_event.kline.open.parse::<f32>().unwrap(),
                        //     close: kline_event.kline.close.parse::<f32>().unwrap()
                        // };
                        // println!("Received binance data");
                        on_data_receive();
                    },
                    _ => (),
                };
                Ok(())
            });

        let kline: String = format!("{}", "btceur@kline_1m");
        web_socket.connect(&kline).unwrap();
        web_socket.event_loop(&keep_running).unwrap();
    }

    fn get_historical_data(&self, start: chrono::DateTime<chrono::Utc>, end: chrono::DateTime<chrono::Utc>) -> AggregatedMarketData {

        let start_time = u64::try_from(start.timestamp_millis()).unwrap();
        let end_time = u64::try_from(end.timestamp_millis()).unwrap();
        match self.market.get_klines(&self.symbol, &self.interval, None, start_time , end_time) {
            Ok(klines) => {
                match klines {
                    binance::model::KlineSummaries::AllKlineSummaries(klines) => {
                        let mut formatted = Vec::<Aggregate>::new();
                        for kline in klines {
                            let aggregate = Aggregate {
                                open: kline.open.parse::<f32>().unwrap(),
                                close : kline.close.parse::<f32>().unwrap()
                            };
                            formatted.push(aggregate);
                        }
                        AggregatedMarketData {
                            aggregates: formatted,
                            exchange_name: String::from("BINANCE"),
                            market_name: String::from(&self.symbol),
                            aggregation_length: String::from(&self.interval),
                            start_time: start,
                            end_time: end
                        }
                    }
                }
            },
            _ => panic!("Unable to receive market data from binance"),
        }
    }
}