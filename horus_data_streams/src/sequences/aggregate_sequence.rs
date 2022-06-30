use std::cell::RefCell;

use heapless::spsc::Queue;
use horus_finance::aggregate::Aggregate;

pub struct AggregateSequence<const SIZE: usize> {
    data: RefCell<Queue<Aggregate, SIZE>>,
    sequence_sum: f32,
    pub moving_average: Option<f32>,
    pub rate_of_change: Option<f32>,
}

impl<const SIZE: usize> AggregateSequence<SIZE> {
    pub fn new() -> AggregateSequence<SIZE> {
        AggregateSequence{
            data: RefCell::new(Queue::new()),
            sequence_sum: 0.,
            moving_average: None,
            rate_of_change: None
        }
    }

    pub fn enqueue(&mut self, new_aggregate: Aggregate) {

        let mut dequeued: Option<Aggregate> = None;
        if self.data.borrow().is_full() {
            dequeued = self.data.borrow_mut().dequeue();
        }

        let _ = self.data.borrow_mut().enqueue(new_aggregate);

        self.calculate_moving_average(new_aggregate, dequeued);
        self.calculate_rate_of_change(new_aggregate, dequeued);
    }

    fn calculate_moving_average(&mut self, new_aggregate: Aggregate, dequeued: Option<Aggregate>) {

        self.sequence_sum += new_aggregate.close;

        if let Some(removed_agg) = dequeued {
            self.sequence_sum -= removed_agg.close;
            self.moving_average = Some(self.sequence_sum / (SIZE - 1) as f32);
        }

        self.moving_average = None;
    }

    fn calculate_rate_of_change(&mut self, new_aggregate: Aggregate, dequeued: Option<Aggregate>) {

        match dequeued {
            Some(deq) => {
                let roc = (new_aggregate.close - deq.close) * 100. / deq.close;
                self.rate_of_change = Some(roc);
            },
            _ => { self.rate_of_change = None; }
        }
    }
}

#[cfg(test)]
mod moving_average_tests {

    use crate::sequences::aggregate_sequence::{test_sequences::{create_linear_growing_market, create_moving_market, create_stable_market}, AggregateSequence};

    #[test]
    fn should_compute_correct_moving_average_in_stable_market() {

        // Arrange
        let mut seq = AggregateSequence::<7>::new();
        let market = create_stable_market();
        
        // Act
        for datum in market {
            seq.enqueue(datum);
        }

        // Assert
        assert_eq!(Some(10.), seq.moving_average);
    }

    #[test]
    fn should_compute_correct_moving_average_in_growing_market() {

        // Arrange
        let mut seq = AggregateSequence::<5>::new();
        let market = create_linear_growing_market();
        
        // Act
        for datum in market {

            seq.enqueue(datum);
        }

        // Assert
        assert_eq!(Some(8.5), seq.moving_average);
    }

    #[test]
    fn should_compute_correct_moving_average_in_moving_market() {

        // Arrange
        let mut seq = AggregateSequence::<5>::new();
        let market = create_moving_market();
        
        // Act
        for datum in market {

            seq.enqueue(datum);
        }

        // Assert
        assert_eq!(Some(4.5), seq.moving_average);
    }
}

#[cfg(test)]
mod rate_of_change_tests {

    use float_cmp::approx_eq;

    use crate::sequences::aggregate_sequence::{test_sequences::{create_linear_growing_market, create_linear_falling_market, create_moving_market, create_stable_market}, AggregateSequence};

    #[test]
    fn should_compute_correct_rate_of_change_in_stable_market() {

        // Arrange
        let mut seq = AggregateSequence::<8>::new();
        let market = create_stable_market();
        
        // Act
        for datum in market {

            seq.enqueue(datum);
        }

        // Assert
        assert_eq!(Some(0.), seq.rate_of_change);
    }

    #[test]
    fn should_compute_correct_rate_of_change_in_linear_growing_market() {

        // Arrange
        let mut seq = AggregateSequence::<5>::new();
        let market = create_linear_growing_market();
        
        // Act
        for datum in market {

            seq.enqueue(datum);
        }

        // Assert
        assert!( approx_eq!(f32, 66.6666666, seq.rate_of_change.unwrap(), ulps = 2) );
    }

    #[test]
    fn should_compute_correct_rate_of_change_in_linear_falling_market() {

        // Arrange
        let mut seq = AggregateSequence::<5>::new();
        let market = create_linear_falling_market();
        
        // Act
        for datum in market {

            seq.enqueue(datum);
        }

        // Assert
        assert!( approx_eq!(f32, -80.00, seq.rate_of_change.unwrap(), ulps = 2) );
    }

    #[test]
    fn should_compute_correct_rate_of_change_in_moving_market() {

        // Arrange
        let mut seq = AggregateSequence::<4>::new();
        let market = create_moving_market();
        
        // Act
        for datum in market {

            seq.enqueue(datum);
        }

        // Assert
        assert!( approx_eq!(f32, 100.00, seq.rate_of_change.unwrap(), ulps = 2) );
    }
 }

 #[cfg(test)]
mod test_sequences {
    use horus_finance::aggregate::Aggregate;
 
    pub fn create_linear_falling_market() -> Vec<Aggregate> {

        let mut aggregates: Vec<Aggregate> = Vec::new();

        for i in 0..10 {

            let aggregate = Aggregate {
                open: 11. - i as f32,
                close: 10. - i as f32
            };

            aggregates.push(aggregate);
        }

        aggregates
    }

    pub fn create_linear_growing_market() -> Vec<Aggregate> {

        let mut aggregates: Vec<Aggregate> = Vec::new();

        for i in 0..10 {

            let aggregate = Aggregate {
                open: i as f32,
                close: i as f32 + 1.
            };

            aggregates.push(aggregate);
        }

        aggregates
    }

    pub fn create_moving_market() -> Vec<Aggregate> {

        let mut aggregates: Vec<Aggregate> = Vec::new();

        for i in 1..11 {

            let open: f32;
            let close: f32;

            if i % 2 != 0 {
                open = 6.;
                close = 3.;
            } else {
                open = 3.;
                close = 6.;
            }

            let aggregate = Aggregate {
                open,
                close,
            };

            aggregates.push(aggregate);
        }

        aggregates
    }

    pub fn create_stable_market() -> Vec<Aggregate> {

        let mut aggregates: Vec<Aggregate> = Vec::new();

        for _ in 1..10 {

            let aggregate = Aggregate {
                open: 10.,
                close: 10.,
            };

            aggregates.push(aggregate);
        }

        aggregates
    } 
}