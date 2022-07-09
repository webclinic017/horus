use std::sync::atomic::AtomicBool;

use chrono::{NaiveDateTime, DateTime, Utc};
use horus_finance::aggregate::Aggregate;
use binance::{websockets::*, market::Market, api::Binance};

use crate::models::time_series_element::TimeSeriesElement;

use super::data_stream::DataStream;

pub struct BinanceFuturesAggregateStream<'a> {
    market: Market,
    symbol: String,
    interval: String,
    on_data: &'a dyn Fn(Aggregate)
}

impl<'a> BinanceFuturesAggregateStream<'a> {
    pub fn new(on_data: &'a dyn Fn(Aggregate), symbol: String, interval: String) -> BinanceFuturesAggregateStream<'a> {
        BinanceFuturesAggregateStream {
            market: Binance::new(None, None),
            symbol,
            interval,
            on_data
        }
    }
}

impl<'a> DataStream<Aggregate> for BinanceFuturesAggregateStream<'a> {
    fn start_listening(&mut self) {
            let keep_running = AtomicBool::new(true);
            let mut web_socket: WebSockets = WebSockets::new(|event: WebsocketEvent| {
                if let WebsocketEvent::Kline(kline_event) = event {
                    let new_aggregate = Aggregate {
                        open: kline_event.kline.open.parse::<f32>().unwrap(),
                        close: kline_event.kline.close.parse::<f32>().unwrap()
                    };
                    (self.on_data)(new_aggregate);
                }
                Ok(())
            });

        let kline: String = "btceur@kline_1m".to_string();
        web_socket.connect(&kline).unwrap();
        web_socket.event_loop(&keep_running).unwrap();
    }

    fn get_historical_data(&self, start: chrono::DateTime<chrono::Utc>, end: chrono::DateTime<chrono::Utc>) -> Vec<TimeSeriesElement<Aggregate>> {

        let start_time = u64::try_from(start.timestamp_millis()).unwrap();
        let end_time = u64::try_from(end.timestamp_millis()).unwrap();
        match self.market.get_klines(&self.symbol, &self.interval, None, start_time , end_time) {
            Ok(klines) => {
                match klines {
                    binance::model::KlineSummaries::AllKlineSummaries(klines) => {
                        let mut formatted = Vec::<TimeSeriesElement<Aggregate>>::new();
                        for kline in klines {
                            let naive_start = NaiveDateTime::from_timestamp(kline.open_time, 0);
                            let naive_end = NaiveDateTime::from_timestamp(kline.close_time, 0);
                            let element: TimeSeriesElement<Aggregate> = TimeSeriesElement::<Aggregate> {
                                start: DateTime::from_utc(naive_start, Utc),
                                end: DateTime::from_utc(naive_end, Utc),
                                datum: Aggregate { 
                                    open: kline.open.parse::<f32>().unwrap(),
                                    close: kline.close.parse::<f32>().unwrap()
                                }
                            };
                            formatted.push(element);
                        }
                        formatted
                    }
                }
            },
            _ => panic!("Unable to receive market data from binance"),
        }
    }
}