use std::cell::RefCell;
use horus_data_streams::sequences::sequence::Sequence;
use horus_finance::Aggregate;

use super::signal::Signal;

enum GoldenCrossSignalType {
    ShortCrossedLong,
    LongCrossedShort
}

pub struct GoldenCrossSignal<const SHORT_MA: usize, const LONG_MA: usize> {
    short_moving_average_seq: Sequence<Aggregate, SHORT_MA>,
    long_moving_average_seq: Sequence<Aggregate, LONG_MA>,
    short_moving_avg_sum: RefCell<f32>,
    long_moving_avg_sum: RefCell<f32>,
    short_below_long: RefCell<Option<bool>>
}

impl<const SHORT_MA: usize, const LONG_MA: usize> GoldenCrossSignal<SHORT_MA, LONG_MA> {
    fn new() -> GoldenCrossSignal<SHORT_MA, LONG_MA> {
        GoldenCrossSignal { 
            short_moving_average_seq: Sequence::<Aggregate, SHORT_MA>::new(),
            long_moving_average_seq: Sequence::<Aggregate, LONG_MA>::new(),
            short_moving_avg_sum: RefCell::new(0.),
            long_moving_avg_sum: RefCell::new(0.),
            short_below_long: RefCell::new(None)
        }
    }
}

impl<const SHORT_MA: usize, const LONG_MA: usize> Signal<Aggregate, GoldenCrossSignalType> for GoldenCrossSignal<SHORT_MA, LONG_MA> {
    fn next(&self, aggregate: &Aggregate) -> Option<GoldenCrossSignalType> {

        let mut short_sum_borrowed = self.short_moving_avg_sum.borrow_mut();
        let mut long_sum_borrowed = self.long_moving_avg_sum.borrow_mut();

        let first_moving_average_value = self.short_moving_average_seq.enqueue_for_moving_average(aggregate, &mut short_sum_borrowed);
        let second_moving_average_value = self.long_moving_average_seq.enqueue_for_moving_average(aggregate, &mut long_sum_borrowed);

        match first_moving_average_value {
            Some(f_val) => {
                match second_moving_average_value {
                    Some(s_val) => {
                        let next_short_below_long: bool = f_val < s_val;
                        let mut side_to_return: Option<GoldenCrossSignalType> = None;
                        {
                            let tmp_ref = self.short_below_long.borrow();
                            match *tmp_ref {
                                Some(s) => { 
                                    let last_short_below_long = s;
                                    if next_short_below_long && !last_short_below_long {
                                        side_to_return = Some(GoldenCrossSignalType::ShortCrossedLong);
                                    } 
                                    if !next_short_below_long && last_short_below_long {
                                        side_to_return = Some(GoldenCrossSignalType::LongCrossedShort);
                                    }
                                }
                                None => {
                                    side_to_return = None;
                                }
                            }
                        }
                        let mut borrowed_ref = self.short_below_long.borrow_mut();
                        *borrowed_ref = Some(next_short_below_long);
                        side_to_return
                    }
                    None => {
                        None
                    }
                }
            }
            None => {
                None
            }
        }
    }
}