// use yew::prelude::*;
// // use gloo::console::{self, Timer};
// use gloo::timers::callback::Interval;

// struct Position {
//     agent_id: &'static str,
//     exchange_name: &'static str,
//     quote_name: &'static str,
//     currency: &'static str,
//     date_of_buy: &'static str,
//     date_of_sell: &'static str,
//     buy_price: f32,
//     sell_price: f32,
//     amount: f32,
// }

// impl Clone for Position {
//     fn clone(&self) -> Self {
//         Self { 
//             agent_id: self.agent_id.clone(), 
//             exchange_name: self.exchange_name.clone(), 
//             quote_name: self.quote_name.clone(), 
//             currency: self.currency.clone(), 
//             date_of_buy: self.date_of_buy.clone(), 
//             date_of_sell: <&str>::clone(self.date_of_sell), 
//             buy_price: self.buy_price, 
//             sell_price: self.sell_price, 
//             amount: self.amount 
//         }
//     }
// }

// impl Copy for Position { }

// struct PositionTable {
//     positions: Vec<Position>,
//     _publish_task: Interval
// }

// impl Component for PositionTable {
//     type Message = Position;
//     type Properties = ();

//     fn create(ctx: &Context<Self>) -> Self {

//         let publish_interval = {
//             let link = ctx.link().clone();

//             let fake_position = Position {
//                 agent_id: "#A337",
//                 exchange_name: "BINANCE",
//                 quote_name: "BTC",
//                 currency: "EUR",
//                 date_of_buy: "DROELF",
//                 date_of_sell: "DROELF",
//                 buy_price: 13.98,
//                 sell_price: 29.45,
//                 amount: 12.
//             };
//             Interval::new(1000, move || {
//                 link.send_message(fake_position);
//             })
//         };

//         Self {
//             positions: vec![
//                 Position {
//                     agent_id: "#A337",
//                     exchange_name: "BINANCE",
//                     quote_name: "BTC",
//                     currency: "EUR",
//                     date_of_buy: "DROELF",
//                     date_of_sell: "DROELF",
//                     buy_price: 13.98,
//                     sell_price: 29.45,
//                     amount: 12.
//                 },
//                 Position {
//                     agent_id: "#A337",
//                     exchange_name: "BINANCE",
//                     quote_name: "IOTA",
//                     currency: "EUR",
//                     date_of_buy: "DROELF",
//                     date_of_sell: "DROELF",
//                     buy_price: 53.98,
//                     sell_price: 59.45,
//                     amount: 16.
//                 }
//             ],
//             _publish_task: publish_interval,
//         }
//     }

//     fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
//         self.positions.push(msg);
//         true
//     }

//     fn view(&self, _ctx: &Context<Self>) -> Html {
//         html! {
//             <table>
//               <tr>
//                 <th>{ "Agent" }</th>
//                 <th>{ "Exchange" }</th>
//                 <th>{ "Quote" }</th>
//                 <th>{ "Buy" }</th>
//                 <th>{ "Sell" }</th>
//                 <th>{ "Absolute Return" }</th>
//                 <th>{ "Relative Return" }</th>
//                 <th>{ "Holding Time" }</th>
//               </tr>
//               {
//                 self.positions.iter().map(|pos| {
//                   html!{
//                     <tr>
//                       <td>{ pos.agent_id }</td>
//                       <td>{ pos.exchange_name }</td>
//                       <td>{ pos.quote_name }</td>
//                       <td>{ pos.buy_price }</td>
//                       <td>{ pos.sell_price }</td>
//                       <td>{ pos.sell_price - pos.buy_price }</td>
//                       <td>{ "13%" }</td>
//                       <td>{ "5 seconds" }</td>
//                     </tr>
//                   }
//                 }).collect::<Html>()
//               }
//             </table> 
//         }
//     }
// }

fn main() {
    // yew::start_app::<PositionTable>();
}
