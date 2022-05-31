use yew::prelude::*;
use gloo::console::{self, Timer};
use gloo::timers::callback::{Interval, Timeout};

struct Position {
    agent_id: &'static str,
    // exchange_name: CompactString,
    // quote_name: CompactString,
    // currency: CompactString,
    // date_of_buy: CompactString,
    // date_of_sell: CompactString,
    // buy_price: f32,
    // sell_price: f32,
    // amount: f32,
}

impl Clone for Position {
    fn clone(&self) -> Self {
        Self { agent_id: self.agent_id.clone() }
    }
}

impl Copy for Position {
    
}

struct PositionTable {
    positions: [Position; 2],
    publish_task: Interval
}

impl Component for PositionTable {
    type Message = Position;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {

        let publish_interval = {
            let link = ctx.link().clone();

            let fake_position = Position {
                agent_id: "#A337",
                // exchange_name: CompactString::from("BINANCE"),
                // quote_name: CompactString::from("BTC"),
                // currency: CompactString::from("EUR"),
                // date_of_buy: CompactString::from("DROELF"),
                // date_of_sell: CompactString::from("DROELF"),
                // buy_price: 13.98,
                // sell_price: 29.45,
                // amount: 12.
            };
            Interval::new(1000, move || link.send_message(fake_position))
        };

        Self {
            positions: [
                Position {
                    agent_id: "#A337",
                    // exchange_name: CompactString::from("BINANCE"),
                    // quote_name: CompactString::from("BTC"),
                    // currency: CompactString::from("EUR"),
                    // date_of_buy: CompactString::from("DROELF"),
                    // date_of_sell: CompactString::from("DROELF"),
                    // buy_price: 13.98,
                    // sell_price: 29.45,
                    // amount: 12.
                },
                Position {
                    agent_id: "#A337",
                    // exchange_name: CompactString::from("BINANCE"),
                    // quote_name: CompactString::from("IOTA"),
                    // currency: CompactString::from("EUR"),
                    // date_of_buy: CompactString::from("DROELF"),
                    // date_of_sell: CompactString::from("DROELF"),
                    // buy_price: 53.98,
                    // sell_price: 59.45,
                    // amount: 16.
                }
            ],
            publish_task: publish_interval,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        true
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <table>
              <tr>
                <th>{ "Agent" }</th>
                <th>{ "Exchange" }</th>
                <th>{ "Quote" }</th>
                <th>{ "Buy" }</th>
                <th>{ "Sell" }</th>
                <th>{ "Absolute Return" }</th>
                <th>{ "Relative Return" }</th>
                <th>{ "Holding Time" }</th>
              </tr>
              {
                self.positions.iter().map(|pos| {
                  html!{
                    <tr>
                      <td>{ pos.agent_id }</td>
                    //   <td>{ pos.exchange_name }</td>
                    //   <td>{ pos.agent_id }</td>
                    //   <td>{ pos.exchange_name }</td>
                    //   <td>{ pos.agent_id }</td>
                    //   <td>{ pos.exchange_name }</td>
                    //   <td>{ pos.agent_id }</td>
                    //   <td>{ pos.exchange_name }</td>
                    </tr>
                  }
                }).collect::<Html>()
              }
            </table> 
        }
    }
}

fn main() {
    yew::start_app::<PositionTable>();
}
