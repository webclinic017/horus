use horus_data_streams::streams::data_stream::DataReceiver;
use horus_finance::{aggregate::Aggregate, market_snapshot::MarketSnapshot};

use crate::{market_events::{MarketNewAggregateEvent, MarketNewSnapshotEvent}, simulation_element::SimulationElement};

pub struct MarketsAggregateEventSimulation {
    market_data: Vec<MarketNewAggregateEvent>
}

pub struct MarketsSnapshotEventSimulation {
    market_data: Vec<MarketNewSnapshotEvent>
}

pub struct BacktestSimulation {
    time_series_elements: Vec<SimulationElement<'a, Box<dyn TimeSeriesElement>>>
}

impl MarketsAggregateEventSimulation {
    pub fn new() -> MarketsAggregateEventSimulation {
        MarketsAggregateEventSimulation { 
            market_data: Vec::<MarketNewAggregateEvent>::new() 
        }
    }

    pub fn insert(&self, market_id: &str, data: Vec<Aggregate>) {
        todo!()
    }
}

impl MarketsSnapshotEventSimulation {
    pub fn new() -> MarketsSnapshotEventSimulation {
        MarketsSnapshotEventSimulation { 
            market_data: Vec::<MarketNewSnapshotEvent>::new() 
        }
    }

    pub fn insert(&self, market_id: &str, data: Vec<MarketSnapshot>) {
        todo!()
    } 
}

impl Iterator for MarketsAggregateEventSimulation {

    type Item = MarketNewAggregateEvent;

    fn next(&mut self) -> Option<Self::Item> {

        todo!()
        // if self.market_data.len() > self.index {
        //     self.index += 1;
        //     Some(self.market_data[self.index - 1])
        // } else {
        //     None
        // }
    }
}

impl Iterator for MarketsSnapshotEventSimulation {

    type Item = MarketNewSnapshotEvent;

    fn next(&mut self) -> Option<Self::Item> {

        todo!()
        // if self.market_data.len() > self.index {
        //     self.index += 1;
        //     Some(self.market_data[self.index - 1])
        // } else {
        //     None
        // }
    }
}