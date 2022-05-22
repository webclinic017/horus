use horus_data_streams::{receivers::lemon_markets_data_receiver::LemonMarketsDataReceiver, sequences::market_data_sequence::MarketDataSequence};
use horus_data_streams::streams::lemon_markets_stream::LemonMarketsStream;
use horus_finance::{Order, OrderSide};
use horus_order_routing::exchange_connector::ExchangeConnector;
use horus_order_routing::lemon_markets::LemonMarketsConnector;
use horus_order_routing::mock_exchange_connector::MockExchangeConnector;
use horus_order_routing::order_router::route_order;
// use horus_data_streams::streams::twitter_stream::TwitterStream;
use std::sync::{Arc, Mutex};

#[derive(PartialEq)]
enum CrossPosition {
    UNDETERMINED,
    BELOW,
    ABOVE
}

struct StrategyState {
    pub ticker: usize,
    pub cross_position: CrossPosition
}

fn global_strategy(market_data: &Arc<Mutex<MarketDataSequence>>, state: &Arc<Mutex<StrategyState>>, connector: &MockExchangeConnector, instrument: &str) {
    let lock_01 = market_data.lock().unwrap();

    let sma_50 = lock_01.get_sma(&5);
    let sma_200 = lock_01.get_sma(&20);

    let mut state = state.lock().unwrap();

    let price = 1337.;
    let expiration_date = "2022-22-01";

    //check buy
    match sma_50 {
        Some(s_50) => {
            match sma_200 {
                Some(s_200) => {
                    if s_200 < s_50 {
                        if state.cross_position != CrossPosition::ABOVE {
                            let order = Order {
                                expiration_date: String::from("2022-12-31"),
                                instrument_symbol: String::from("FCK"),
                                quantity: 1337.,
                                side: OrderSide::SELL,
                                price: 1337.
                            };
                            route_order(&order, connector);
                            state.ticker = 60;
                            state.cross_position = CrossPosition::ABOVE;
                        }
                    } else {
                        state.cross_position = CrossPosition::BELOW;
                    }
                },
                _ => {},
            }
        },
        _ => {},
    }

    //check sell
    if state.ticker > 0 {

        state.ticker -= 1;

        if state.ticker == 0 {
            let order = Order {
                expiration_date: String::from("2022-12-31"),
                instrument_symbol: String::from("FCK"),
                quantity: 1337.,
                side: OrderSide::SELL,
                price: 1337.
            };
            route_order(&order, connector);
            
        }
    }
}

fn main() {

    let strategy_state = StrategyState{ ticker: 0, cross_position: CrossPosition::UNDETERMINED };

    let market_data: MarketDataSequence = MarketDataSequence::new();
    let market_data_ref_base = Arc::new(Mutex::new(market_data));
    let state_ref_base = Arc::new(Mutex::new(strategy_state));

    let market_data_ref = Arc::clone(&market_data_ref_base);
    let state_ref = Arc::clone(&state_ref_base);

    const LEMON_MARKETS_AUTH_KEY: &str = "";
    const EXCHANGE: &str = "";
    const INSTRUMENT: &str = "LU0274211480";

    let handle_01 = std::thread::Builder::new().name("order_thread".to_string()).spawn(move || {

        // let lemon_markets = LemonMarketsConnector::new(false, LEMON_MARKETS_AUTH_KEY);
        let lemon_markets = MockExchangeConnector::new();

        let strategy = || { global_strategy(&market_data_ref, &state_ref, &lemon_markets, INSTRUMENT) };

        let lemon_markets_stream: LemonMarketsStream = LemonMarketsStream {
            data_sequence: &market_data_ref,
            receiver: LemonMarketsDataReceiver::new(EXCHANGE, INSTRUMENT),
            strategy: Some(&strategy)
        };

        lemon_markets_stream.connect();
    });

    handle_01.unwrap().join().unwrap();
}