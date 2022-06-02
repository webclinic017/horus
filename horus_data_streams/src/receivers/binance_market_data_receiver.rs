use std::sync::atomic::AtomicBool;

use horus_finance::Aggregate;
use binance::{websockets::*, market::Market, api::Binance};

use super::data_receiver::DataReceiver;

pub struct BinanceMarketDataReceiver<'a, ONDATARECEIVE: Fn(Aggregate)> {
    market: Market,
    symbol: String,
    interval: String,
    on_data_receive: &'a ONDATARECEIVE
}

impl<'a, ONDATARECEIVE: Fn(Aggregate)> BinanceMarketDataReceiver<'a, ONDATARECEIVE> {
    pub fn new(symbol: String, interval: String, on_data_receive: &'a ONDATARECEIVE) -> BinanceMarketDataReceiver<ONDATARECEIVE> {
        BinanceMarketDataReceiver {
            market: Binance::new(None, None),
            symbol,
            interval,
            on_data_receive
        }
    }
}

impl<'a, ONDATARECEIVE: Fn(Aggregate)> DataReceiver<Aggregate> for BinanceMarketDataReceiver<'a, ONDATARECEIVE> {
    fn start_listening(&self) {
            let keep_running = AtomicBool::new(true);
            let mut web_socket: WebSockets = WebSockets::new(|event: WebsocketEvent| {
                match event {
                    WebsocketEvent::Kline(kline_event) => {
                        let new_aggregate = Aggregate {
                            open: kline_event.kline.open.parse::<f32>().unwrap(),
                            close: kline_event.kline.close.parse::<f32>().unwrap()
                        };
                        println!("Received binance data");
                        let _ = &(self.on_data_receive)(new_aggregate);
                    },
                    _ => (),
                };
                Ok(())
            });

        let kline: String = format!("{}", "btceur@kline_1m");
        web_socket.connect(&kline).unwrap();
        web_socket.event_loop(&keep_running).unwrap();
    }

    fn inject(&self, aggregate: Aggregate) {
        let _ = &(self.on_data_receive)(aggregate);
    }

    fn get_historical_data(&self, start: chrono::DateTime<chrono::Utc>, end: chrono::DateTime<chrono::Utc>) -> Vec<Aggregate> {

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
                        formatted
                    }
                }
            },
            _ => panic!("Unable to receive market data from binance"),
        }
    }
}