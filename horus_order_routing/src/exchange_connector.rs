use horus_finance::Order;

pub trait ExchangeConnector {
    fn route_order(&self, order: &Order) -> bool;
}