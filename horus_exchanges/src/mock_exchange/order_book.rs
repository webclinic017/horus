use crate::{order_queue::OrderQueue, order::Order, order_side::OrderSide};

pub struct Orderbook {
    pub sell_orders: Vec<OrderQueue>,
    pub buy_orders: Vec<OrderQueue>
}

impl Orderbook {
    fn new() -> Orderbook {
        Orderbook {
            sell_orders: Vec::new(), 
            buy_orders: Vec::new()
        }
    }

    fn perform_make_order(&self, order: Order) {
        match order.side {
            OrderSide::BUY => {

            }
            OrderSide::SELL => {
                
            }
        }
    }

    fn perform_take_order(&self, order: Order) -> OrderFilledResult {

        let mut lot_to_be_filled = order.quantity;
        let mut spent = 0.;

        &self.perform_liquidity_check(&order);

        match order.side {
            OrderSide::BUY => {
                while lot_to_be_filled > 0 {

                    if self.sell_orders.len() == 0 { // is sell side is empty?
                        break;
                    }

                    loop {
                        let result = self.sell_orders[0].take_order();
                        if !result.filled {
                            break;
                        }
                        lot_to_be_filled -= 1;
                        spent += result.price;

                        if lot_to_be_filled == 0 {
                            break;
                        }
                    }

                    self.sell_orders.remove(0);
                }
            }
            OrderSide::SELL => {
                while lot_to_be_filled > 0 {

                    if self.buy_orders.len() == 0 { // is buy side is empty?
                        break;
                    }

                    while true {
                        let result = self.buy_orders[0].take_order();
                        if !result.filled {
                            break;
                        }
                        lot_to_be_filled -= 1;
                        spent += filled.price;

                        if lot_to_be_filled == 0 {
                            break;
                        }
                    }

                    self.sell_orders.remove(0);
                }
            }
        }
    }
}

impl Default for Orderbook {
    fn default() -> Self {
        Self::new()
    }
}