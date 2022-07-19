use horus_finance::position::Position;

pub trait Reporter {
    fn update_cash_balance(&mut self, new_cash_balance: f32);
    fn update_position(&mut self, position: &Position);
}