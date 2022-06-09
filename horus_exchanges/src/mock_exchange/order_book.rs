use horus_finance::{order::Order, order_side::OrderSide};

pub struct Orderbook {
    pub sell: f32,
    pub buy: f32
}

impl Orderbook {
    fn new() -> Orderbook {
        Orderbook {
            sell: 0., 
            buy: 0.
        }
    }

    fn perform_make_order(&self, order: Order) {
        todo!();
    }

    fn perform_take_order(&self, order: Order) {

        match order.side {
            OrderSide::SELL => {
                order.quantity * &self.sell;
            }
            OrderSide::BUY => {
                order.quantity * &self.buy;
            }
        }
        
    }
}

impl Default for Orderbook {
    fn default() -> Self {
        Self::new()
    }
}