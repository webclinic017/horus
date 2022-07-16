use horus_finance::order::Order;

pub trait Reporter {
    fn report(order: &Order);
}