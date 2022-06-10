use core::time;
use std::thread;

use horus_finance::aggregate::Aggregate;

pub struct LemonMarketsDataReceiver<'a, ONDATARECEIVE: Fn(Aggregate)> {
    pub exchange: String,
    pub instrument_isin: String,
    on_data_receive: &'a ONDATARECEIVE
}

impl<'a, ONDATARECEIVE: Fn(Aggregate)> LemonMarketsDataReceiver<'a, ONDATARECEIVE> {

    pub fn new(exchange: &str, instrument_isin: &str, on_data_receive: &'a ONDATARECEIVE) -> LemonMarketsDataReceiver<'a, ONDATARECEIVE> {
        LemonMarketsDataReceiver {
            exchange: String::from(exchange),
            instrument_isin: String::from(instrument_isin),
            on_data_receive
        }
    } 

    pub fn start_listening(&self) {
        
        loop {
            let new_aggregate = Aggregate {
                open: 1330.,
                close: 1337.
            };
            println!("Simulate exchange tick");
            (self.on_data_receive)(new_aggregate);

            thread::sleep(time::Duration::from_millis(200));
        }
    }
}