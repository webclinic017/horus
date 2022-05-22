use core::time;

pub struct TwitterActivityReceiver {}

impl TwitterActivityReceiver {

    pub fn start_listening<'a>(&self, on_data_receive: &'a dyn Fn()) {

        loop {
            std::thread::sleep(time::Duration::from_secs(2));
            on_data_receive();
        }
    }
}
