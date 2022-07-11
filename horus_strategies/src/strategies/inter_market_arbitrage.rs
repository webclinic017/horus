use horus_data_streams::sequences::snapshot_sequence::SnapshotSequence;
use horus_exchanges::connectors::market_connector::MarketConnector;
use horus_finance::{market_snapshot::MarketSnapshot, order::Order};

use super::strategy::Strategy;

pub struct InterMarketArbitrageStrategy<'a, ConnectorOne: MarketConnector, ConnectorTwo: MarketConnector> {
    market_connector_01: &'a ConnectorOne,
    market_connector_02: &'a ConnectorTwo,
    sequence_01: &'a mut SnapshotSequence<16>,
    sequence_02: &'a mut SnapshotSequence<16>,
}

impl<'a, ConnectorOne: MarketConnector, ConnectorTwo: MarketConnector> InterMarketArbitrageStrategy<'a, ConnectorOne, ConnectorTwo> {
    pub fn new(connector_01: &'a ConnectorOne, connector_02: &'a ConnectorTwo, 
    sequence_01: &'a mut SnapshotSequence<16>, sequence_02: &'a mut SnapshotSequence<16>) -> InterMarketArbitrageStrategy<'a, ConnectorOne, ConnectorTwo> {
        InterMarketArbitrageStrategy {
            market_connector_01: connector_01,
            market_connector_02: connector_02,
            sequence_01,
            sequence_02,
        }
    }

    fn run_hot_path(&self) {

        let ask_01 = self.sequence_01.get_current_ask();
        let bid_02 = self.sequence_02.get_current_bid();

        if bid_02 > ask_01 {
            let order = Order { 
                side: horus_finance::order_side::OrderSide::SELL,
                quantity: 0, 
                price: None, 
                expiration_date: None
            };
            //TODO: route order by magic string
            self.market_connector_01.route_take_order(&order);
        }
    }

    pub fn seq_01_next(&mut self, snapshot: MarketSnapshot) {
        self.sequence_01.enqueue(snapshot);
        self.run_hot_path();
    }

    pub fn seq_02_next(&mut self, snapshot: MarketSnapshot) {
        self.sequence_02.enqueue(snapshot);
        self.run_hot_path();
    }
}

impl<'a, ConnectorOne: MarketConnector, ConnectorTwo: MarketConnector> Strategy for InterMarketArbitrageStrategy<'a, ConnectorOne, ConnectorTwo> {

    fn get_name() -> &'static str {
        return "Inter Exchange Arbitrage";
    }
}
