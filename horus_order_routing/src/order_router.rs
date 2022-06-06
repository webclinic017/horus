use horus_finance::Order;

use crate::exchange_connector::ExchangeConnector;

fn publish_order(order: &Order) {
    println!("Publishing {:?} order to RabbitMQ", order.side);
}

pub fn route_order<E>(order: &Order, exchange_connector: &E) where E: ExchangeConnector {

    exchange_connector.route_order(order);

    publish_order(order);
}