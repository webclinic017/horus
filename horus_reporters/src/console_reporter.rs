use heapless::spsc::Queue;
use horus_finance::position::Position;

use crate::reporter::Reporter;

static column_widths: [i8; 9] = [ 5, 18, 12, 18, 18, 20, 20, 16, 4 ];

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
                break;
            }

            let pos = self.active_positions[index];

            if pos.exchange == new_position.exchange && pos.market == new_position.market {
                self.active_positions.remove(index);
                break;
            }
        }
    }

    fn clear_console(&self) {
        print!("{}[2J", 27 as char);
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

    fn render_header(&self) {
        println!("AWESOME HEADER");
    }
    fn render_position(&self, position: &Position) {
        println!("Position goes here");
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

        match position.sell_price {
            Some(p) => {
                self.closed_positions.enqueue(*position);
            }
            None => {
                self.insert_active_position(position);
            }
        }
        self.render_table();
    }
}