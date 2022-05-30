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

    pub fn enqueue(&self, next_aggregate: &Aggregate) {

        if self.data.borrow().is_full() {
            self.data.borrow_mut().dequeue().unwrap();
        }

        self.data.borrow_mut().enqueue(*next_aggregate).unwrap();
        // enqueue
        // update moving avg
        // update ...
    }

    pub fn get_moving_average(&self) -> Option<f32> {

        let current_size: usize = self.data.borrow().len().try_into().unwrap();

        if current_size < SIZE {
            return None
        }

        let start = current_size - SIZE;

        let mut total: f32 = 0.0;

        for index in start..current_size {

            total += self.data.borrow().iter().nth(index.into()).unwrap().close;
        }

        Some(total / (SIZE as f32))
    }

    pub fn get_rate_of_change(&self) -> Option<f32> {
        todo!();
    }

    pub fn get_average_true_range(&self) -> Option<f32> {
        todo!();
    }

    pub fn get_momentum(&self) -> Option<f32> {
        todo!();
    }
}

#[cfg(test)]
mod moving_average_tests {

    use horus_finance::Aggregate;

    use crate::sequences::sequence::Sequence;

    #[test]
    fn should_not_compute_moving_average_when_range_is_higher_then_length_of_sequence() {

        // Arrange
        let seq = Sequence::<Aggregate, 10>::new();
        
        // Act
        let moving_average = seq.get_moving_average();

        // Assert
        assert_eq!(None, moving_average);
    }

    #[test]
    fn should_compute_correct_moving_average_when_queue_is_partially_filled() {
        
        // Arrange
        let mut seq = Sequence::<Aggregate, 10>::new();
        fill_sequence(&mut seq);
        
        // Act
        let moving_average = seq.get_moving_average();

        // Assert
        assert_eq!(Some(10.0), moving_average);
    }

    #[test]
    fn should_compute_correct_moving_average_after_queue_is_dequeued() {
        // Arrange
        let mut seq = Sequence::<Aggregate, 10>::new();
        fill_sequence(&mut seq);
        let aggregate = Aggregate {
            open: 10.0,
            close: 30.0
        };
        seq.enqueue(&aggregate);

        // Act
        let moving_average_5 = seq.get_moving_average();
        let moving_average_2 = seq.get_moving_average();

        // Assert
        assert_eq!(Some(14.0), moving_average_5);
        assert_eq!(Some(20.0), moving_average_2);
    }

    fn fill_sequence(sequence: &mut Sequence::<Aggregate, 10>) {

        for _ in 0..9{

            let aggregate = Aggregate {
                open: 10.0,
                close: 10.0
            };

            sequence.enqueue(&aggregate);
        }
    }
}

#[cfg(test)]
mod rate_of_change_tests { }

#[cfg(test)]
mod average_true_range_tests { }

#[cfg(test)]
mod momentum_tests { }

mod fake_sequences { 
    
}