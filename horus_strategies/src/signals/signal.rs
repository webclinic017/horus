pub trait Signal<I, O> {
    fn next(&self, input: &I) -> Option<O>;
}