use std::cell::RefCell;

use heapless::spsc::Queue;
use horus_finance::Aggregate;

use super::sequence::Sequence;

impl<const SIZE: usize> Sequence<Aggregate, SIZE> {
    pub fn new() -> Sequence<Aggregate, SIZE> {
        Sequence{
            data: RefCell::new(Queue::new())
        }
    }

    pub fn enqueue(&self, aggregate: &Aggregate) -> Option<Aggregate> {

        let mut dequeued: Option<Aggregate> = None;
        if self.data.borrow().is_full() {
            dequeued = self.data.borrow_mut().dequeue();
        }

        self.data.borrow_mut().enqueue(*aggregate).unwrap();

        dequeued
    }

    pub fn enqueue_for_moving_average(&self, aggregate: &Aggregate, sequence_sum: &mut f32) -> Option<f32> {

        let current_size: usize = self.data.borrow().len();

        let is_ready = current_size >= SIZE - 1;

        let dequeued = self.enqueue(aggregate);

        *sequence_sum += aggregate.close;

        if let Some(removed_agg) = dequeued {
            *sequence_sum -= removed_agg.close;
        }

        if is_ready {
            Some(*sequence_sum / (SIZE - 1) as f32)
        } else {
            None
        }
    }

    pub fn enqueue_for_rate_of_change(&self, aggregate: &Aggregate) -> Option<f32> {

        let dequeued = self.enqueue(aggregate);

        dequeued.map(|deq| (aggregate.close - deq.close) * 100. / deq.close)
    }
}

impl<const SIZE: usize> Default for Sequence<Aggregate, SIZE> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod moving_average_tests {

    use horus_finance::Aggregate;

    use crate::sequences::{sequence::Sequence, aggregate_sequence::test_sequences::{create_linear_growing_market, create_moving_market, create_stable_market}};

    #[test]
    fn should_compute_correct_moving_average_in_stable_market() {

        // Arrange
        let seq = Sequence::<Aggregate, 7>::new();
        let market = create_stable_market();
        let mut ma_sum: f32 = 0.;

        let mut moving_average: Option<f32> = None;
        
        // Act
        for datum in market {

            moving_average = seq.enqueue_for_moving_average(&datum, &mut ma_sum);
        }

        // Assert
        assert_eq!(Some(10.), moving_average);
    }

    #[test]
    fn should_compute_correct_moving_average_in_growing_market() {

        // Arrange
        let seq = Sequence::<Aggregate, 5>::new();
        let market = create_linear_growing_market();
        let mut ma_sum: f32 = 0.;

        let mut moving_average: Option<f32> = None;
        
        // Act
        for datum in market {

            moving_average = seq.enqueue_for_moving_average(&datum, &mut ma_sum);
        }

        // Assert
        assert_eq!(Some(8.5), moving_average);
    }

    #[test]
    fn should_compute_correct_moving_average_in_moving_market() {

        // Arrange
        let seq = Sequence::<Aggregate, 5>::new();
        let market = create_moving_market();
        let mut ma_sum: f32 = 0.;

        let mut moving_average: Option<f32> = None;
        
        // Act
        for datum in market {

            moving_average = seq.enqueue_for_moving_average(&datum, &mut ma_sum);
        }

        // Assert
        assert_eq!(Some(4.5), moving_average);
    }
}

#[cfg(test)]
mod rate_of_change_tests {

    use float_cmp::approx_eq;
    use horus_finance::Aggregate;

    use crate::sequences::{sequence::Sequence, aggregate_sequence::test_sequences::{create_stable_market, create_linear_growing_market, create_linear_falling_market, create_moving_market}};

    #[test]
    fn should_compute_correct_rate_of_change_in_stable_market() {

        // Arrange
        let seq = Sequence::<Aggregate, 8>::new();
        let market = create_stable_market();

        let mut rate_of_change: Option<f32> = None;
        
        // Act
        for datum in market {

            rate_of_change = seq.enqueue_for_rate_of_change(&datum);
        }

        // Assert
        assert_eq!(Some(0.), rate_of_change);
    }

    #[test]
    fn should_compute_correct_rate_of_change_in_linear_growing_market() {

        // Arrange
        let seq = Sequence::<Aggregate, 5>::new();
        let market = create_linear_growing_market();

        let mut rate_of_change: Option<f32> = None;
        
        // Act
        for datum in market {

            rate_of_change = seq.enqueue_for_rate_of_change(&datum);
        }

        // Assert
        assert!( approx_eq!(f32, 66.6666666, rate_of_change.unwrap(), ulps = 2) );
    }

    #[test]
    fn should_compute_correct_rate_of_change_in_linear_falling_market() {

        // Arrange
        let seq = Sequence::<Aggregate, 5>::new();
        let market = create_linear_falling_market();

        let mut rate_of_change: Option<f32> = None;
        
        // Act
        for datum in market {

            rate_of_change = seq.enqueue_for_rate_of_change(&datum);
        }

        // Assert
        assert!( approx_eq!(f32, -80.00, rate_of_change.unwrap(), ulps = 2) );
    }

    #[test]
    fn should_compute_correct_rate_of_change_in_moving_market() {

        // Arrange
        let seq = Sequence::<Aggregate, 4>::new();
        let market = create_moving_market();

        let mut rate_of_change: Option<f32> = None;
        
        // Act
        for datum in market {

            rate_of_change = seq.enqueue_for_rate_of_change(&datum);
        }

        // Assert
        assert!( approx_eq!(f32, 100.00, rate_of_change.unwrap(), ulps = 2) );
    }
 }

// #[cfg(test)]
// mod average_true_range_tests {

//     use horus_finance::Aggregate;

//     use crate::sequences::sequence::Sequence;

//     #[test]
//     fn should_not_compute_average_true_range_when_sequence_is_not_ready() {

//         // Arrange
//         let seq = Sequence::<Aggregate, 10>::new();
        
//         // Act
//         let average_true_range = seq.get_average_true_range();

//         // Assert
//         assert_eq!(None, average_true_range);
//     }
//  }

// #[cfg(test)]
// mod momentum_tests {

//     use horus_finance::Aggregate;

//     use crate::sequences::sequence::Sequence;

//     #[test]
//     fn should_not_compute_momentum_when_sequence_is_not_ready() {

//         // Arrange
//         let seq = Sequence::<Aggregate, 10>::new();
        
//         // Act
//         let momentum = seq.get_momentum();

//         // Assert
//         assert_eq!(None, momentum);
//     }
//  }

 #[cfg(test)]
mod test_sequences {
    use horus_finance::Aggregate;
 
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