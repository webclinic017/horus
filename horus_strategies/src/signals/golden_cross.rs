use horus_finance::aggregate::Aggregate;

enum GoldenCrossSignalType {
    ShortCrossedLong,
    LongCrossedShort
}

pub struct GoldenCrossSignal<const SHORT_MA: usize, const LONG_MA: usize> {
}

impl<const SHORT_MA: usize, const LONG_MA: usize> GoldenCrossSignal<SHORT_MA, LONG_MA> {
    pub fn next(&self, aggregate: Aggregate) -> Option<GoldenCrossSignalType> {
        todo!()
    }
}