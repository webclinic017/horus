use std::cell::RefCell;

use horus_finance::{aggregate::Aggregate, market_snapshot::MarketSnapshot, order::Order, order_side::OrderSide};

use crate::connectors::market_connector::MarketConnector;

pub struct MockMarketConnector {
    pub current_ask: RefCell<f32>,
    pub current_bid: RefCell<f32>,
    pub asset_balance: RefCell<usize>,
    pub cash_balance: RefCell<f32>
}

impl MockMarketConnector {
    pub fn new(initial_cash_balance: f32) -> MockMarketConnector {
        MockMarketConnector {
            current_ask: RefCell::new(0.),
            current_bid: RefCell::new(0.),
            asset_balance: RefCell::new(0),
            cash_balance: RefCell::new(initial_cash_balance)
        }
    }

    pub fn inject_snapshot(&self, snapshot: MarketSnapshot) {

        let mut mut_ref_ask = self.current_ask.borrow_mut();
        let mut mut_ref_bid = self.current_bid.borrow_mut();
        *mut_ref_ask = snapshot.current_ask;
        *mut_ref_bid = snapshot.current_bid;
    }

    pub fn inject_aggregate(&self, order: Aggregate) {
        let mut mut_ref_ask = self.current_ask.borrow_mut();
        *mut_ref_ask = order.close;
    }

    pub fn get_current_ask(&self) -> f32 {
        *self.current_ask.borrow()
    }

    pub fn get_current_bid(&self) -> f32 {
        *self.current_bid.borrow()
    }

    pub fn get_asset_balance(&self) -> usize {
        *self.asset_balance.borrow()
    }

    pub fn get_cash_balance(&self) -> f32 {
        *self.cash_balance.borrow()
    }
}

impl MarketConnector for MockMarketConnector {

    fn route_make_order(&self, order: &Order) -> bool {
        
        let mut cash_balance_ref = self.cash_balance.borrow_mut();
        let mut asset_balance_ref = self.asset_balance.borrow_mut();

        let price;
        let cash_to_transfer;
        let lot_to_transfer;
        match order.side {
            OrderSide::BUY => {
                price = order.price.unwrap();
                cash_to_transfer = order.quantity as f32 * price;

                if cash_to_transfer > *cash_balance_ref {
                    return false
                }

                lot_to_transfer = (cash_to_transfer / price).floor();

                *asset_balance_ref += lot_to_transfer as usize;
                *cash_balance_ref -= cash_to_transfer;
                
            },
            OrderSide::SELL => {

                if order.quantity > *asset_balance_ref {
                    return false;
                }

                price = order.price.unwrap();
                cash_to_transfer = order.quantity as f32 * price;

                lot_to_transfer = (cash_to_transfer / price).floor();

                *asset_balance_ref -= lot_to_transfer as usize;
                *cash_balance_ref += cash_to_transfer;
            }
        }

        true
    }

    fn route_take_order(&self, order: &Order) -> bool {
        
        let mut cash_balance_ref = self.cash_balance.borrow_mut();
        let mut asset_balance_ref = self.asset_balance.borrow_mut();

        let price;
        let cash_to_transfer;
        let lot_to_transfer;
        match order.side {
            OrderSide::BUY => {
                price = *self.current_ask.borrow();
                cash_to_transfer = order.quantity as f32 * price;

                if cash_to_transfer > *cash_balance_ref {
                    return false
                }

                lot_to_transfer = (cash_to_transfer / price).floor();

                *asset_balance_ref += lot_to_transfer as usize;
                *cash_balance_ref -= cash_to_transfer;
                
            },
            OrderSide::SELL => {

                if order.quantity > *asset_balance_ref {
                    return false;
                }

                price = *self.current_bid.borrow();
                cash_to_transfer = order.quantity as f32 * price;

                lot_to_transfer = (cash_to_transfer / price).floor();

                *asset_balance_ref -= lot_to_transfer as usize;
                *cash_balance_ref += cash_to_transfer;
            }
        }

        true
    }

    fn get_historical_snapshots(&self, _start: chrono::DateTime<chrono::Utc>, _end: chrono::DateTime<chrono::Utc>) -> Vec<MarketSnapshot> {
        panic!("This connector is only used for backtesting and can not provide historical data")
    }

    fn get_historical_aggregates(&self, _start: chrono::DateTime<chrono::Utc>, _end: chrono::DateTime<chrono::Utc>) -> Vec<Aggregate> {
        panic!("This connector is only used for backtesting and can not provide historical data")
    }
}