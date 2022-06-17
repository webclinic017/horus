use horus_data_streams::streams::data_stream::DataStream;
use horus_exchanges::connectors::market_connector::MarketConnector;
use horus_finance::{aggregate::Aggregate, market_snapshot::MarketSnapshot, order::Order};

use super::strategy::Strategy;

pub struct InterMarketArbitrageStrategy<'a, 
        ConnectorOne: MarketConnector, 
        ConnectorTwo: MarketConnector, 
        StreamOne: DataStream<'a, MarketSnapshot>, 
        StreamTwo: DataStream<'a, MarketSnapshot>> {
    market_connector_01: &'a ConnectorOne,
    market_connector_02: &'a ConnectorTwo,
    market_stream_01: &'a mut StreamOne,
    market_stream_02: &'a mut StreamTwo,
    ask_01: f32,
    bid_02: f32,
}

impl<'a, ConnectorOne: MarketConnector, ConnectorTwo: MarketConnector, 
StreamOne: DataStream<'a, MarketSnapshot>, 
StreamTwo: DataStream<'a, MarketSnapshot>> InterMarketArbitrageStrategy<'a, ConnectorOne, ConnectorTwo, StreamOne, StreamTwo> {
    pub fn new(connector_01: &'a ConnectorOne, 
    connector_02: &'a ConnectorTwo, 
    stream_01: &'a mut StreamOne, 
    stream_02: &'a mut StreamTwo) -> InterMarketArbitrageStrategy<'a, ConnectorOne, ConnectorTwo, StreamOne, StreamTwo> {
        InterMarketArbitrageStrategy {
            market_connector_01: connector_01,
            market_connector_02: connector_02,
            market_stream_01: stream_01,
            market_stream_02: stream_02,
            ask_01: 0.,
            bid_02: 0.
        }
    }
}

impl<'a, 
ConnectorOne: MarketConnector, 
ConnectorTwo: MarketConnector, 
StreamOne: DataStream<'a, MarketSnapshot>, 
StreamTwo: DataStream<'a, MarketSnapshot>> Strategy for InterMarketArbitrageStrategy<'a, ConnectorOne, ConnectorTwo, StreamOne, StreamTwo> {
    fn run(&self) -> std::thread::JoinHandle<()> {
        let on_data_receive = |market_key:&str, new_snapshot: MarketSnapshot| {
            //TODO: adjust bid/ask
            if self.bid_02 > self.ask_01 {
                let order = Order { 
                    exchange_id: "".to_string(), 
                    instrument_symbol: "".to_string(), 
                    side: horus_finance::order_side::OrderSide::SELL, 
                    quantity: 0, 
                    price: None, 
                    expiration_date: None
                };
                //TODO: route order by magic string
                self.market_connector_01.route_take_order(&order);
            }
        };
        let handle_01 = self.market_stream_01.start_listening(&on_data_receive);
        let handle_02 = self.market_stream_02.start_listening(&on_data_receive);

        todo!("Join handles!");
    }

    fn get_name() -> &'static str {
        return "Inter Exchange Arbitrage";
    }
}
