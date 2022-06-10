use horus_finance::aggregate::Aggregate;

use super::data_receiver::DataReceiver;

pub struct MockMarketDataReceiver<'a, ONDATARECEIVE: Fn(Aggregate)> {
    on_data_receive: &'a ONDATARECEIVE
}

impl<'a, ONDATARECEIVE: Fn(Aggregate)> DataReceiver<Aggregate> for MockMarketDataReceiver<'a, ONDATARECEIVE> {
    fn start_listening(&self) {
        todo!()
    }

    fn inject(&self, datum: Aggregate) {
        let _ = &(self.on_data_receive)(datum);
    }

    fn get_historical_data(&self, _start: chrono::DateTime<chrono::Utc>, _end: chrono::DateTime<chrono::Utc>) -> Vec<Aggregate> {
        let mock_data = vec![
            Aggregate {
                open: 1.,
                close: 2.
            },
            Aggregate {
                open: 2.,
                close: 3.
            }
        ];

        mock_data
    }
}