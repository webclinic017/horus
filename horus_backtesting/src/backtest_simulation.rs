use horus_finance::time_series_element::TimeSeriesElement;

use crate::simulation_element::SimulationElement;

pub struct BacktestSimulation<'a> {
    time_series_elements: Vec<SimulationElement<'a, Box<dyn TimeSeriesElement>>>,
    index: usize
}

impl<'a> BacktestSimulation<'a> {
    pub fn run(&self) {
        for element in &self.time_series_elements {
            element.stream.inject(element.datum);
        }
    }
}