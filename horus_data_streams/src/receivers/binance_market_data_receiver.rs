use std::sync::atomic::AtomicBool;

use horus_finance::Aggregate;
use binance::{websockets::*, market::Market, model::KlineSummary, api::Binance};

use super::data_receiver::DataReceiver;

pub struct BinanceMarketDataReceiver {
    market: Market,
    symbol: String,
    interval: String
}

    
// impl BTCEURBinanceMarketDataReceiver {

//     pub fn start_listening<'a>(&self, on_data_receive: &'a dyn Fn(Aggregate)) {
        
//         let keep_running = AtomicBool::new(true);
//         let mut web_socket: WebSockets = WebSockets::new(|event: WebsocketEvent| {
//             match event {
//                 WebsocketEvent::Kline(kline_event) => {
//                     let new_aggregate = Aggregate {
//                         open: kline_event.kline.open.parse::<f32>().unwrap(),
//                         close: kline_event.kline.close.parse::<f32>().unwrap()
//                     };
//                     println!("Received binance data");
//                     on_data_receive(new_aggregate);
//                 },
//                 _ => (),
//             };
//             Ok(())
//         });

//         let kline: String = format!("{}", "btceur@kline_1m");
//         web_socket.connect(&kline).unwrap();
//         web_socket.event_loop(&keep_running).unwrap();
//     }
// }

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
        todo!()
    }

    fn get_historical_data(&self, start: chrono::DateTime<chrono::Utc>, end: chrono::DateTime<chrono::Utc>) -> Vec<Aggregate> {

        match self.market.get_klines(&self.symbol, &self.interval, 800, None, None) {
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
            Err(e) => panic!("Unable to receive market data from binance"),
        }
    }
}