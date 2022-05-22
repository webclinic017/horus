use crate::exchange_connector::ExchangeConnector;

pub struct MockExchangeConnector {
}

impl MockExchangeConnector {
    pub fn new() -> MockExchangeConnector {
        MockExchangeConnector {  }
    }
}

impl ExchangeConnector for MockExchangeConnector {

    fn route_order(&self, order: &horus_finance::Order) -> bool {
        println!("Pretending to route an {:?} order", order.side);
        true
    }
}