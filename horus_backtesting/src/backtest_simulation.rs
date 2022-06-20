use horus_finance::time_series_element::TimeSeriesElement;

use crate::simulation_element::SimulationElement;

pub struct BacktestSimulation<'a> {
    time_series_elements: Vec<SimulationElement<'a, Box<dyn TimeSeriesElement>>>,
}

impl<'a> BacktestSimulation<'a> {
    pub fn run(&mut self) {
        for element in self.time_series_elements.drain(..) {
            element.stream.inject(element.datum);
        }
    }
}