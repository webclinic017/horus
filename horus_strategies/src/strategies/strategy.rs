use std::thread::JoinHandle;

pub trait Strategy {
    fn run(&self) -> JoinHandle<()>;
    fn get_name() -> &'static str;
}