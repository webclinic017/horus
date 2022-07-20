use heapless::spsc::Queue;
use colored::Colorize;
use horus_finance::position::Position;

use crate::reporter::Reporter;

static INTRO_LABEL_PADDING_LENGTH: u8 = 16;
static COLUMN_WIDTHS: [u8; 4] = [ 8, 8, 13, 13 ];
static DEFAULT_PLACEHOLDER: &str = "-";

enum AgentStatus {
    Inactive,
    Unknown,
    Active
}

pub struct ConsoleReporter {
    agent_status: AgentStatus,
    agent_status_update: String,
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
        let reporter = ConsoleReporter {
            agent_status: AgentStatus::Unknown,
            agent_status_update: String::new(),
            exchange_name: exchange.to_string(),
            market_name: market.to_string(),
            market_currency_symbol: currency.to_string(),
            initial_cash_balance: None,
            cash_balance: 0.,
            open_position: None,
            closed_positions: Queue::<Position, 20>::new()
        };

        reporter.render();

        reporter
    }

    fn get_current_date_string(&self) -> String {
        "14.07.2022, 02:34 AM".to_string()
    }

    fn clear_console(&self) {
        print!("{esc}c", esc = 27 as char);
    }

    fn pad(&self, original: &str, padded: &mut String, length: u8) {

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

    fn render_intro_label(&self, label: &str) {

        let lead = ":";
        let mut padded_lead = String::new();
        let padding_length = INTRO_LABEL_PADDING_LENGTH - (label.len() as u8);
        self.pad(lead, &mut padded_lead, padding_length);
        print!("{}{}", label.bold(), padded_lead);
    }

    fn render_intro_status(&self) {

        self.render_intro_label("Status");

        match self.agent_status {
            
            AgentStatus::Inactive => {
                println!("{}", "Inactive".red())
            },
            AgentStatus::Unknown => {
                println!("{}", "Unknown".yellow())
            },
            AgentStatus::Active => {
                println!("{}", "Active".green())
            }
        }
    }

    fn render_intro_status_update(&self) {

        match self.agent_status {
            
            AgentStatus::Inactive => {
                self.render_intro_label("Stop");
                println!("{}", self.agent_status_update);
            },
            AgentStatus::Unknown => {
                self.render_intro_label("Start");
                println!("{}", DEFAULT_PLACEHOLDER);
            },
            AgentStatus::Active => {
                self.render_intro_label("Start");
                println!("{}", self.agent_status_update);
            }
        }
    }

    fn render_intro_exchange(&self) {
        
        self.render_intro_label("Exchange");
        println!("{}", self.exchange_name);
    }

    fn render_intro_market(&self) {
        
        self.render_intro_label("Market");
        println!("{}", self.market_name);
    }

    fn render_cash_balance(&self) {

        let mut cash_balance_display = "?".to_string();
        let mut cash_balance_indicator = "".to_string();

        if let Some(init) = self.initial_cash_balance {
            cash_balance_display = self.cash_balance.to_string();
            cash_balance_display.push_str(&self.market_currency_symbol);

            let abs_diff = self.cash_balance - init;
            let _rel_diff = (self.cash_balance / init - 1.) * 100.;

            if abs_diff > 0. {
                cash_balance_indicator = "\u{2197}".green().to_string();
            }

            if abs_diff < 0. {
                cash_balance_indicator = "\u{2198}".red().to_string();
            }

            if abs_diff == 0. {
                cash_balance_indicator = "\u{27A1}".to_string();
            }
        }

        self.render_intro_label("Cash Balance");
        println!("{} {}", cash_balance_display, cash_balance_indicator);
    }

    fn render_intro_section(&self) {

        self.render_intro_status();
        self.render_intro_status_update();
        self.render_intro_exchange();
        self.render_intro_market();
        self.render_cash_balance();
        println!();
    }

    fn render_header(&self) {

        let mut padded = String::new();
        self.pad("STATUS", &mut padded, COLUMN_WIDTHS[0]);
        print!("{}", padded.bold());

        let mut padded = String::new();
        self.pad("LOT", &mut padded, COLUMN_WIDTHS[1]);
        print!("{}", padded.bold());

        let mut padded = String::new();
        self.pad("BUY PRICE", &mut padded, COLUMN_WIDTHS[2]);
        print!("{}", padded.bold());

        let mut padded = String::new();
        self.pad("SELL PRICE", &mut padded, COLUMN_WIDTHS[3]);
        print!("{}", padded.bold());

        println!();
    }

    fn render_position_status(&self, position: &Position) {
        
        let mut padded = String::new();

        if position.sell_price.is_some() {
            self.pad("Closed", &mut padded, COLUMN_WIDTHS[0]);
            print!("{}", padded.red())
        } else {
            self.pad("Open", &mut padded, COLUMN_WIDTHS[0]);
            print!("{}", padded.green())
        }
    }

    fn render_position_lot(&self, position: &Position) {
        
        let mut padded = String::new();
        self.pad(&position.quantity.to_string(), &mut padded, COLUMN_WIDTHS[1]);
        print!("{}", padded)
    }

    fn render_position_buy(&self, position: &Position) {
        
        let mut padded = String::new();
        self.pad(&position.buy_price.to_string(), &mut padded, COLUMN_WIDTHS[2]);
        print!("{}", padded)
    }

    fn render_position_sell(&self, position: &Position) {
        
        let mut padded = String::new();

        if let Some(price) = position.sell_price {
            self.pad(&price.to_string(), &mut padded, COLUMN_WIDTHS[3]);
        } else {
            self.pad(DEFAULT_PLACEHOLDER, &mut padded, COLUMN_WIDTHS[3]);
        }
        print!("{}", padded)
    }

    fn render_position_return(&self, position: &Position) {
        
        if let Some(sell) = position.sell_price {
            let abs_diff = sell - position.buy_price;
            let _rel_diff = (sell / position.buy_price - 1.) * 100.;

            if abs_diff > 0. {
                print!("{}", "\u{2197}".green().to_string());
            }

            if abs_diff < 0. {
                print!("{}", "\u{2198}".red().to_string());
            }

            if abs_diff == 0. {
                print!("{}", "\u{27A1}".to_string());
            }
        }
    }

    fn render_position(&self, position: &Position) {

        self.render_position_status(position);
        self.render_position_lot(position);
        self.render_position_buy(position);
        self.render_position_sell(position);
        self.render_position_return(position);
        println!();
    }

    fn render_open_position(&self) {
        if let Some(pos) = self.open_position {
            self.render_position(&pos);
        }
    }

    fn render_closed_positions(&self) {
        for position in &self.closed_positions {
            self.render_position(position);
        }
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

    fn update_status(&mut self, becoming_active: bool) {
        if becoming_active {
            self.agent_status = AgentStatus::Active;
        } else {
            self.agent_status = AgentStatus::Inactive;
        }

        self.agent_status_update = self.get_current_date_string();

        self.render();
    }

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