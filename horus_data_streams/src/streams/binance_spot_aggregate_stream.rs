use std::{sync::atomic::AtomicBool, cell::RefCell, borrow::Borrow};

use horus_finance::aggregate::Aggregate;
use binance::{websockets::*, market::Market, api::Binance};

use super::data_stream::DataStream;

pub struct BinanceSpotAggregateStream<'a> {
    market: Market,
    symbol: String,
    interval: String,
    middleware: RefCell<Vec<&'a dyn Fn(&str, Aggregate)>>
}

impl<'a> BinanceSpotAggregateStream<'a> {
    pub fn new(symbol: String, interval: String) -> BinanceSpotAggregateStream<'a> {
        BinanceSpotAggregateStream {
            market: Binance::new(None, None),
            symbol,
            interval,
            middleware: RefCell::new(Vec::new())
        }
    }
}

impl<'a> DataStream<'a, Aggregate> for BinanceSpotAggregateStream<'a> {
    fn start_listening(&self, hot_path: &dyn Fn(&str, Aggregate)) {
            let keep_running = AtomicBool::new(true);
            let mut web_socket: WebSockets = WebSockets::new(|event: WebsocketEvent| {
                if let WebsocketEvent::Kline(kline_event) = event {
                    let new_aggregate = Aggregate {
                        open: kline_event.kline.open.parse::<f32>().unwrap(),
                        close: kline_event.kline.close.parse::<f32>().unwrap()
                    };
                    let middleware_ref = self.middleware.borrow();
                    for mw in  middleware_ref.iter() {
                        mw("binance", new_aggregate);
                    }
                    hot_path("binance", new_aggregate);
                }
                Ok(())
            });

        let kline: String = "btceur@kline_1m".to_string();
        web_socket.connect(&kline).unwrap();
        web_socket.event_loop(&keep_running).unwrap();
    }

    fn add_middleware(&self, on_data: &'a dyn Fn(&str, Aggregate) -> ()) {
        let mut listeners_ref = self.middleware.borrow_mut();
        listeners_ref.push(on_data);
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