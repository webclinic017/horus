pub struct MarketSimulation<TYPE> {
    market_data: Vec<Vec<TYPE>>
}

impl Iterator for MarketSimulation<Aggregate> {

    type Item = Aggregate;

    fn next(&mut self) -> Option<Self::Item> {

        if self.market_data.len() > self.index {
            self.index += 1;
            Some(self.market_data[self.index - 1])
        } else {
            None
        }
    }
}