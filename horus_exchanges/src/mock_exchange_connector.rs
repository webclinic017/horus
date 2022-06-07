use crate::market_connector::ExchangeConnector;

pub struct MockExchangeConnector {
}

impl MockExchangeConnector {
    pub fn new() -> MockExchangeConnector {
        MockExchangeConnector {  }
    }
}

impl Default for MockExchangeConnector {
    fn default() -> Self {
        Self::new()
    }
}

impl ExchangeConnector for MockExchangeConnector {

    fn route_order(&self, order: &horus_finance::Order) -> bool {
        println!("Pretending to route an {:?} order", order.side);
        true
    }
}