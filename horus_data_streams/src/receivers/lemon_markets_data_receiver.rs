use core::time;
use std::thread;

use horus_finance::Aggregate;

pub struct LemonMarketsDataReceiver {
    pub exchange: String,
    pub instrument_isin: String
}

impl LemonMarketsDataReceiver {

    pub fn new(exchange: &str, instrument_isin: &str) -> LemonMarketsDataReceiver {
        LemonMarketsDataReceiver {
            exchange: String::from(exchange),
            instrument_isin: String::from(instrument_isin)
        }
    } 

    pub fn start_listening<'a>(&self, on_data_receive: &'a dyn Fn(Aggregate)) {
        
        loop {
            let new_aggregate = Aggregate {
                open: 1330.,
                close: 1337.
            };
            println!("Simulate exchange tick");
            on_data_receive(new_aggregate);

            thread::sleep(time::Duration::from_millis(200));
        }
    }
}