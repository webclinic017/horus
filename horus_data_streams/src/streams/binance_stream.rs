use std::sync::{Arc, Mutex};
use horus_finance::Aggregate;
use crate::{receivers::binance_market_data_receiver::BinanceMarketDataReceiver, sequences::sequence::Sequence};

pub struct BinanceStream<'a> {
    pub data_sequence: &'a Arc<Mutex<Sequence<Aggregate>>>,
    pub receiver: BinanceMarketDataReceiver,
    pub strategy: Option<&'a dyn Fn()>,
}

impl<'a> BinanceStream<'a> {

    pub fn connect(&self) {
        // let on_data_receive = |nd: Aggregate| {
        //     {
        //         let mut data_write = self.data_sequence.lock().unwrap();
                
        //         data_write.enqueue(nd);
        //     }

        //     match self.strategy {
        //         Some(s) => s(),
        //         None => {},
        //     }
        // };
        // self.receiver.start_listening(&on_data_receive);
        todo!();
    }
}