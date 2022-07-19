use heapless::spsc::Queue;
use colored::Colorize;
use horus_finance::position::Position;

use crate::reporter::Reporter;

static COLUMN_WIDTHS: [u8; 3] = [ 20, 16, 6 ];

pub struct ConsoleReporter {
    exchange_name: String,
    market_name: String,
    market_currency_symbol: String,
    initial_cash_balance: Option<f32>,
    cash_balance: f32,
    open_position: Option<Position>,
    closed_positions: Queue::<Position, 20>
}

impl ConsoleReporter {
    pub fn new(exchange: &str, market: &str, currency: &str) -> ConsoleReporter {
        ConsoleReporter {
            exchange_name: exchange.to_string(),
            market_name: market.to_string(),
            market_currency_symbol: currency.to_string(),
            initial_cash_balance: None,
            cash_balance: 0.,
            open_position: None,
            closed_positions: Queue::<Position, 20>::new()
         }
    }

    fn clear_console(&self) {
        print!("{}[2J", 27 as char);
    }

    fn render_header(&self) {

        let mut padded = String::new();
        self.pad(&"STATUS".to_string(), &mut padded, COLUMN_WIDTHS[2]);
        print!("{}", padded.bold());
        println!();
    }

    fn pad(&self, original: &String, padded: &mut String, length: u8) {

        let mut index: u8 = 0;
        let mut chars = original.chars();

        loop {
            if index >= length {
                return
            }

            if let Some(ch) = chars.next() {
                padded.push(ch);
            } else {
                padded.push(' ');
            }
            index += 1;
        }
    }

    fn render_position(&self, position: &Position) {

        let mut padded_side = String::new();

        if position.sell_price.is_none() {
            self.pad(&"Open".to_string(), &mut padded_side, COLUMN_WIDTHS[2]);
            print!("{}", padded_side.green());
        } else {
            self.pad(&"Closed".to_string(), &mut padded_side, COLUMN_WIDTHS[2]);
            print!("{}", padded_side.red());
        }

        // print!("{}", position.buy_price);
        println!();
    }

    fn render_open_position(&self) {
        if let Some(pos) = self.open_position {
            self.render_position(&pos);
        }
    }

    fn render_closed_positions(&self) {
        for position in &self.closed_positions {
            self.render_position(&position);
        }
    }

    fn render_intro_section(&self) {

        let mut cash_balance_display = "?".to_string();
        let mut cash_balance_indicator = "".to_string();

        if let Some(init) = self.initial_cash_balance {
            cash_balance_display = self.cash_balance.to_string();

            let abs_diff = self.cash_balance - init;
            let _rel_diff = (self.cash_balance / init - 1.) * 100.;

            // if abs_diff > 0. {
            //     cash_balance_indicator = "\u{2197}".green().to_string();
            // }

            // if abs_diff < 0. {
            //     cash_balance_indicator = "\u{2198}".red().to_string();
            // }

            // if abs_diff == 0. {
            //     cash_balance_indicator = "\u{27A1}".to_string();
            // }
            cash_balance_indicator = "\u{2198}".red().to_string();
        }

        println!("Status:       {}", "Active".green());
        println!("Exchange:     {}", self.exchange_name);
        println!("Market:       {}", self.market_name);
        println!("Cash Balance: {}{} {}", cash_balance_display, self.market_currency_symbol, cash_balance_indicator);
        println!();
    }

    fn render_table(&self) {

        self.render_header();

        self.render_open_position();
        self.render_closed_positions();   
    }

    fn render(&self) {

        self.clear_console();
        self.render_intro_section();
        self.render_table()
    }
}

impl Reporter for ConsoleReporter {

    fn update_cash_balance(&mut self, new_cash_balance: f32) {

        if self.initial_cash_balance.is_none() {
            self.initial_cash_balance = Some(new_cash_balance);
        }

        self.cash_balance = new_cash_balance;

        self.render();
    }

    fn update_position(&mut self, position: &Position) {

        if position.sell_price.is_some() {
            self.open_position = None;
            let _ = self.closed_positions.enqueue(*position);
        } else {
            self.open_position = Some(*position);
        }
        self.render();
    }
}