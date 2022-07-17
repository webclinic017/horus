use crate::reporter::Reporter;

pub struct ConsoleReporter {

}

impl ConsoleReporter {
    pub fn new() -> ConsoleReporter {
        ConsoleReporter {  }
    }
}

impl Reporter for ConsoleReporter {
    fn report(&self, order: &horus_finance::order::Order) {
        println!("Received order to buy/sell {}", order.quantity);
    }
}