// use horus_data_streams::models::tweet::Tweet;
// use horus_exchanges::connectors::market_connector::MarketConnector;
// use horus_finance::aggregate::Aggregate;

// use super::strategy::Strategy;

// pub struct TwitterActivityStrategy<'a, Market: MarketConnector> {
//     market: &'a Market,
// }

// impl<'a, Market: MarketConnector> TwitterActivityStrategy<'a, Market> {
//     pub fn new(market: &'a Market) -> TwitterActivityStrategy<'a, Market> {
//         TwitterActivityStrategy {
//             market
//         }
//     }

//     pub fn next_aggregate(&mut self, _aggregate: Aggregate) {
//         todo!();
//     }

//     pub fn next_tweet(&mut self, _tweet: Tweet) {
//         todo!();
//     }
// }

// impl<'a, Market: MarketConnector> Strategy for TwitterActivityStrategy<'a, Market> {
    
//     fn get_name() -> &'static str {
//         return "Twitter Activity";
//     }
// }