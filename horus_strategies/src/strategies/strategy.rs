use std::thread::JoinHandle;
// use horus_data_streams::receivers::data_receiver::DataReceiver;
use horus_finance::Order;

pub trait Strategy {
    fn run<HANDLER: FnMut(Vec<Order>)>(&self, order_handler: HANDLER) -> JoinHandle<()>;
    fn get_name() -> &'static str;
}