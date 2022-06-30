use std::{sync::atomic::AtomicBool, cell::RefCell, rc::{Weak, Rc}};

use horus_finance::aggregate::Aggregate;
use binance::{websockets::*, market::Market, api::Binance};

use crate::sequences::aggregate_sequence::AggregateSequence;

use super::data_stream::DataStream;

pub struct BinanceSpotAggregateStream<const SEQ_SIZE: usize> {
    market: Market,
    symbol: String,
    interval: String,
    on_data: Option<RefCell<Weak<dyn Fn()>>>,
    sequence: AggregateSequence<SEQ_SIZE>
}

impl<const SEQ_SIZE: usize> BinanceSpotAggregateStream<SEQ_SIZE> {
    pub fn new(symbol: String, interval: String) -> BinanceSpotAggregateStream<SEQ_SIZE> {
        BinanceSpotAggregateStream {
            market: Binance::new(None, None),
            symbol,
            interval,
            on_data: None,
            sequence: AggregateSequence::<SEQ_SIZE>::new()
        }
    }
}

impl<const SEQ_SIZE: usize> DataStream<Aggregate> for BinanceSpotAggregateStream<SEQ_SIZE> {
    fn start_listening(&mut self) {
            let keep_running = AtomicBool::new(true);
            let hot_path = self.on_data.as_ref().unwrap().borrow();
            let rc = hot_path.upgrade().unwrap();
            let mut web_socket: WebSockets = WebSockets::new(|event: WebsocketEvent| {
                if let WebsocketEvent::Kline(kline_event) = event {
                    let new_aggregate = Aggregate {
                        open: kline_event.kline.open.parse::<f32>().unwrap(),
                        close: kline_event.kline.close.parse::<f32>().unwrap()
                    };
                    self.sequence.enqueue(new_aggregate);
                    rc();
                }
                Ok(())
            });

        let kline: String = "btceur@kline_1m".to_string();
        web_socket.connect(&kline).unwrap();
        web_socket.event_loop(&keep_running).unwrap();
    }

    fn set_on_data(&self, _on_data: Rc<dyn Fn()>) {
        let mut mut_ref = self.on_data.as_ref().unwrap().borrow_mut();
        *mut_ref = Rc::downgrade(&_on_data);
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