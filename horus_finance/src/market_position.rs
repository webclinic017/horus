/// The side that an algorithm can take in a market.
#[derive(Debug)]
#[derive(PartialEq)]
pub enum MarketPosition {
    SHORT,
    NEUTRAL,
    LONG
}