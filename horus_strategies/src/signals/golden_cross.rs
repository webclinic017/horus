use horus_finance::aggregate::Aggregate;
use ta::{indicators::SimpleMovingAverage, Next};

#[derive(PartialEq)]
#[derive(Clone, Copy)]
pub enum GoldenCrossSignalType {
    ShortOvertakes,
    LongOvertakes
}

pub struct GoldenCrossSignal<const SHORT_MA: usize, const LONG_MA: usize> {
    short: SimpleMovingAverage,
    long: SimpleMovingAverage,
    last_emmited_event: Option<GoldenCrossSignalType>
}

impl<const SHORT_MA: usize, const LONG_MA: usize> GoldenCrossSignal<SHORT_MA, LONG_MA> {
    pub fn new() -> GoldenCrossSignal<SHORT_MA, LONG_MA> {
        GoldenCrossSignal {
            short: SimpleMovingAverage::new(SHORT_MA).unwrap(),
            long: SimpleMovingAverage::new(LONG_MA).unwrap(),
            last_emmited_event: None
        }
    }
    pub fn next(&mut self, aggregate: Aggregate) -> Option<GoldenCrossSignalType> {
        let current_short = self.short.next(aggregate.close as f64);
        let current_long = self.long.next(aggregate.close as f64);

        if current_short > current_long {
            if self.last_emmited_event == Some(GoldenCrossSignalType::LongOvertakes) {
                self.last_emmited_event = Some(GoldenCrossSignalType::ShortOvertakes);
                return Some(self.last_emmited_event.unwrap())
            }
            if self.last_emmited_event == None {
                self.last_emmited_event = Some(GoldenCrossSignalType::ShortOvertakes);
            }
        } else if current_short < current_long {
            if self.last_emmited_event == Some(GoldenCrossSignalType::ShortOvertakes) {   
                self.last_emmited_event = Some(GoldenCrossSignalType::LongOvertakes);
                return Some(self.last_emmited_event.unwrap())
            }
            if self.last_emmited_event == None {
                self.last_emmited_event = Some(GoldenCrossSignalType::LongOvertakes);
            }
        }
        return None
    }
}