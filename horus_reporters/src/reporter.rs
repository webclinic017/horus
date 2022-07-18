use horus_finance::position::Position;

pub trait Reporter {
    fn add_position(&mut self, position: &Position);
}