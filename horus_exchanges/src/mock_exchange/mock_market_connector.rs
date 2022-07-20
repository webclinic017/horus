use std::cell::RefCell;

use horus_finance::{aggregate::Aggregate, order::Order, order_side::OrderSide, market_snapshot::MarketSnapshot, position::Position};

use crate::connectors::market_connector::MarketConnector;

static INIT_EXCHANGE_NAME: &str = "MOCK_EXCHANGE";
static INIT_MARKET_NAME: &str = "MOCK_MARKET";
static INIT_CURRENCY_SYMBOL: &str = "$";

pub struct MockMarketConnector {
    pub current_ask: RefCell<f32>,
    pub current_bid: RefCell<f32>,
    pub cash_balance: RefCell<f32>,
    exchange_name_to_fake: String,
    market_name_to_fake: String,
    currency_symbol_to_fake: String,
    current_position: RefCell<Option<Position>>
}

impl MockMarketConnector {
    pub fn new(initial_cash_balance: f32) -> MockMarketConnector {
        MockMarketConnector {
            current_ask: RefCell::new(0.),
            current_bid: RefCell::new(0.),
            cash_balance: RefCell::new(initial_cash_balance),
            exchange_name_to_fake: INIT_EXCHANGE_NAME.to_string(),
            market_name_to_fake: INIT_MARKET_NAME.to_string(),
            currency_symbol_to_fake: INIT_CURRENCY_SYMBOL.to_string(),
            current_position: RefCell::new(None)
        }
    }

    // pub fn inject_snapshot(&self, snapshot: MarketSnapshot) {

    //     let mut mut_ref_ask = self.current_ask.borrow_mut();
    //     let mut mut_ref_bid = self.current_bid.borrow_mut();
    //     *mut_ref_ask = snapshot.current_ask;
    //     *mut_ref_bid = snapshot.current_bid;
    // }

    pub fn set_fake_exchange_name(&mut self, name: &str) {
        self.exchange_name_to_fake = name.to_string();
    }

    pub fn set_fake_market_name(&mut self, name: &str) {
        self.market_name_to_fake = name.to_string();
    }

    pub fn set_fake_currency_symbol(&mut self, symbol: &str) {
        self.currency_symbol_to_fake = symbol.to_string();
    }

    pub fn set_price(&self, bid: f32, ask: f32) {
        let mut mut_ref_bid = self.current_bid.borrow_mut();
        let mut mut_ref_ask = self.current_ask.borrow_mut();
        *mut_ref_bid = bid;
        *mut_ref_ask = ask;
    }

    pub fn get_current_ask(&self) -> f32 {
        *self.current_ask.borrow()
    }

    pub fn get_current_bid(&self) -> f32 {
        *self.current_bid.borrow()
    }

    pub fn get_cash_balance(&self) -> f32 {
        *self.cash_balance.borrow()
    }
}

impl MarketConnector for MockMarketConnector {

    fn route_make_order(&self, order: &Order) -> Result<Position, String> {
        
        let mut cash_balance_ref = self.cash_balance.borrow_mut();
        let mut pos_ref = self.current_position.borrow_mut();

        let price;
        let cash_to_transfer;
        match order.side {
            OrderSide::BUY => {
                price = order.price.unwrap();
                cash_to_transfer = order.quantity as f32 * price;

                if cash_to_transfer > *cash_balance_ref {
                    return Err("Unable to proceed order due to insufficient cash balance".to_string())
                }

                *cash_balance_ref -= cash_to_transfer;
                
                let position = Position {
                    quantity: order.quantity,
                    buy_price: price,
                    sell_price: None
                };

                *pos_ref = Some(position);
                
                Ok(position)
            },
            OrderSide::SELL => {

                if pos_ref.is_none() {
                    return Err("Unable to close non existing position".to_string())
                }

                let current_pos = pos_ref.unwrap();

                if order.quantity != current_pos.quantity {
                    return Err("Partial fills are currently not supported".to_string())
                }

                price = order.price.unwrap();
                cash_to_transfer = order.quantity as f32 * price;

                *cash_balance_ref += cash_to_transfer;

                let position = Position {
                    quantity: order.quantity,
                    buy_price: current_pos.buy_price,
                    sell_price: Some(price)
                };

                *pos_ref = None;

                Ok(position)
            }
        }
    }

    fn route_take_order(&self, order: &Order) -> Result<Position, String> {
        
        let mut cash_balance_ref = self.cash_balance.borrow_mut();
        let mut pos_ref = self.current_position.borrow_mut();

        let price;
        let cash_to_transfer;
        match order.side {
            OrderSide::BUY => {
                price = *self.current_ask.borrow();
                cash_to_transfer = order.quantity as f32 * price;

                if cash_to_transfer > *cash_balance_ref {
                    return Err("Unable to proceed order due to insufficient cash balance".to_string())
                }

                *cash_balance_ref -= cash_to_transfer;

                let position = Position {
                    quantity: order.quantity,
                    buy_price: price,
                    sell_price: None
                };

                *pos_ref = Some(position);

                Ok(position)
                
            },
            OrderSide::SELL => {

                if pos_ref.is_none() {
                    return Err("Unable to close non existing position".to_string())
                }

                let current_pos = pos_ref.unwrap();

                if order.quantity != current_pos.quantity {
                    return Err("Partial fills are currently not supported".to_string())
                }

                price = *self.current_bid.borrow();
                cash_to_transfer = order.quantity as f32 * price;

                *cash_balance_ref += cash_to_transfer;

                let position = Position {
                    quantity: order.quantity,
                    buy_price: current_pos.buy_price,
                    sell_price: Some(price)
                };

                *pos_ref = None;

                Ok(position)
            }
        }
    }

    fn get_historical_snapshots(&self, _start: chrono::DateTime<chrono::Utc>, _end: chrono::DateTime<chrono::Utc>) -> Vec<MarketSnapshot> {
        panic!("This connector is only used for backtesting and can not provide historical data")
    }

    fn get_historical_aggregates(&self, _start: chrono::DateTime<chrono::Utc>, _end: chrono::DateTime<chrono::Utc>) -> Vec<Aggregate> {
        panic!("This connector is only used for backtesting and can not provide historical data")
    }
    
    fn get_exchange_name(&self) -> String {
        (&self.exchange_name_to_fake).to_string()
    }

    fn get_market_name(&self) -> String {
        (&self.market_name_to_fake).to_string()
    }

    fn get_currency_symbol(&self) -> String {
        (&self.currency_symbol_to_fake).to_string()
    }
}