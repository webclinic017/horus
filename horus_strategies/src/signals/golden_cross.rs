use std::cell::RefCell;
use horus_data_streams::sequences::aggregate_sequence::AggregateSequence;
use horus_finance::aggregate::Aggregate;

use super::signal::Signal;

enum GoldenCrossSignalType {
    ShortCrossedLong,
    LongCrossedShort
}

pub struct GoldenCrossSignal<const SHORT_MA: usize, const LONG_MA: usize> {
    sequence: &'a AggregateSequence
}

impl<const SHORT_MA: usize, const LONG_MA: usize> Signal<Aggregate, GoldenCrossSignalType> for GoldenCrossSignal<SHORT_MA, LONG_MA> {
    fn next(&self, aggregate: Aggregate) -> Option<GoldenCrossSignalType> {

        todo!()
    }
}