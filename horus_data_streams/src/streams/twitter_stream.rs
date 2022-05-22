use crate::receivers::twitter_activity_receiver::TwitterActivityReceiver;

pub struct TwitterStream<'a> {
    pub receiver: TwitterActivityReceiver,
    pub strategy: Option<&'a dyn Fn()>
}

impl<'a> TwitterStream<'a> {

    pub fn connect(&self) {
        let on_data_receive = || {
            
            match self.strategy {
                Some(s) => s(),
                None => {},
            }
        };
        self.receiver.start_listening(&on_data_receive);
    }
}