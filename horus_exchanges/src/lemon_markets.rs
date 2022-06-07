use std::{panic};
use horus_finance::{Position, Order};
use serde_json::Value;

use crate::market_connector::ExchangeConnector;

// fn add_order_side(map: &mut HashMap<&str, &str>, order_type: &MarketPosition) {
//     match order_type {
//         MarketPosition::LONG => { map.insert("side", "buy"); },
//         MarketPosition::SHORT => { map.insert("side", "sell"); },
//         _ => {},
//     }
// }

// fn validate_order(side: &MarketPosition, quantity: &f32, price: &f32) {

//     const MIN_BUY: f32 = 50.;
    
//     match side {
//         MarketPosition::LONG => {
//             let price_total = (*quantity as f32) * *price;

//             if price_total < MIN_BUY {
//                 panic!("Invalid order");
//             }
//         }
//         _ => {},
//     }
// }

pub struct LemonMarketsConnector {
    client: reqwest::Client,
    base_url: String,
    auth_key: String
}

impl LemonMarketsConnector {
    pub fn new(real_money: bool, auth_key: &str) -> LemonMarketsConnector {
        LemonMarketsConnector { 
            client: reqwest::Client::new(),
            base_url: if real_money { String::from("https://trading.lemon.markets")} else { String::from("https://paper-trading.lemon.markets") },
            auth_key: String::from(auth_key)
        }
    }

    pub async fn get_market_position(&self, instrument_isin: &str) -> Option<Position> {
        let request = self.client.get(format!("{}/v1/positions?isin={}", self.base_url, instrument_isin))
          .header("Authorization", String::from(&self.auth_key));

        let body = request.send().await.unwrap().text().await.unwrap();

        let v: Value = serde_json::from_str(&body).unwrap();

        let pos = &v[0];

        match pos {
            Value::Object(position) => {
                let raw_quantity = &position["quantity"];
                match raw_quantity {
                    Value::Number(quantity) => { 
                        Some(Position {
                            quantity: quantity.as_f64()? as f32
                        })
                    }
                    Value::String(quantity) => { 
                        Some(Position {
                            quantity: quantity.parse::<f32>().unwrap()
                        })
                    }
                    _ => { panic!(); }
                }
                
            }
            Value::Null => { None }
            _ => { panic!(); }
        } 
    }
}

impl ExchangeConnector for LemonMarketsConnector {

    fn route_order(&self, _order: &Order) -> bool {
        // validate_order(&order.side, &order.quantity, &order.price);

        // let request = self.client.post(format!("{}/v1/orders/", self.base_url))
        //   .header("Authorization", String::from(&self.auth_key));
        
        // let mut map = HashMap::<&str, &str>::new();
        // let price_as_str = order.price.to_string();
        // let amount_as_str = order.quantity.to_string();
        // let venue = String::from("XMUN");
    
        // map.insert("isin", &order.instrument_symbol);
        // add_order_side(&mut map, &order.side);
        // map.insert("quantity", &amount_as_str);
        // map.insert("limit_price", &price_as_str);
        // map.insert("venue", &venue);
        // map.insert("expires_at", &order.expiration_date);
    
        // let sending = request.json(&map).send();
        // futures::executor::block_on(sending).unwrap();
        true
    }
}
