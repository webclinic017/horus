use heapless::spsc::Queue;
use colored::Colorize;
use horus_finance::position::Position;

use crate::reporter::Reporter;

static COLUMN_WIDTHS: [u8; 3] = [ 20, 16, 6 ];

pub struct ConsoleReporter {
    active_positions: Vec::<Position>,
    closed_positions: Queue::<Position, 20>
}

impl ConsoleReporter {
    pub fn new() -> ConsoleReporter {
        ConsoleReporter {
            active_positions: Vec::<Position>::with_capacity(20),
            closed_positions: Queue::<Position, 20>::new()
         }
    }

    fn insert_active_position(&mut self, position: &Position) {
        self.active_positions.push(*position);
    }

    fn remove_active_position(&mut self, new_position: &Position) {
        
        let mut index = 0;

        loop {
            if index >= self.active_positions.len() {
                return
            }

            let pos = self.active_positions[index];

            if pos.exchange == new_position.exchange && pos.market == new_position.market {
                self.active_positions.remove(index);
                return
            }

            index += 1;
        }
    }

    fn clear_console(&self) {
        print!("{}[2J", 27 as char);
    }

    fn render_header(&self) {
        println!("AWESOME HEADER");
    }

    fn pad(&self, original: &String, padded: &mut String, length: u8) {

        let index: u8 = 0;
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
        }
    }

    fn render_position(&self, position: &Position) {
        let mut padded_exchange = String::new();
        self.pad(&position.exchange.to_string(), &mut padded_exchange, COLUMN_WIDTHS[0]);
        print!("{}", padded_exchange);
        
        let mut padded_market = String::new();
        self.pad(&position.market.to_string(), &mut padded_market, COLUMN_WIDTHS[1]);
        print!("{}", padded_market);

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

    fn render_active_positions(&self) {
        for position in &self.active_positions {
            self.render_position(&position);
        } 
    }

    fn render_closed_positions(&self) {
        for position in &self.active_positions {
            self.render_position(&position);
        }
    }

    pub fn render_table(&mut self) {

        self.clear_console();
        self.render_header();

        self.render_active_positions();
        self.render_closed_positions();   
    }
}

impl Reporter for ConsoleReporter {
    fn add_position(&mut self, position: &Position) {

        self.remove_active_position(position);

        if position.sell_price.is_some() {
            let _ = self.closed_positions.enqueue(*position);
        } else {
            self.insert_active_position(position);
        }
        self.render_table();
    }
}