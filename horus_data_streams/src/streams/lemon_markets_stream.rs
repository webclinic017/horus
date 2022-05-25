use std::sync::{Arc, Mutex};
use horus_finance::Aggregate;
use crate::{receivers::lemon_markets_data_receiver::LemonMarketsDataReceiver, sequences::sequence::Sequence};

pub struct LemonMarketsStream<'a> {
    pub data_sequence: &'a Arc<Mutex<Sequence<Aggregate>>>,
    pub receiver: LemonMarketsDataReceiver,
    pub strategy: Option<&'a dyn Fn()>,
}

impl<'a> LemonMarketsStream<'a> {

    pub fn connect(&self) {
        let on_data_receive = |nd: Aggregate| {
            {
                let mut data_write = self.data_sequence.lock().unwrap();
                
                data_write.enqueue(&nd);
            }

            match self.strategy {
                Some(s) => s(),
                None => {},
            }
        };
        self.receiver.start_listening(&on_data_receive);
    }
}