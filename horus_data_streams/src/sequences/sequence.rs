use std::cell::RefCell;

use heapless::spsc::Queue;
use horus_finance::Aggregate;

const QUEUE_SIZE: usize = 256;

pub struct Sequence<T> {
    pub data: RefCell<Queue<T, QUEUE_SIZE>>
}

impl Sequence<Aggregate> {
    pub fn new() -> Sequence<Aggregate> {
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

    pub fn get_sma(&self, range: &u16) -> Option<f32> {

        let current_size: u16 = self.data.borrow().len().try_into().unwrap();

        if current_size < *range {
            return None
        }

        let start: u16 = current_size - range;

        let mut total: f32 = 0.0;

        for index in start..current_size {

            total += self.data.borrow().iter().nth(index.into()).unwrap().close;
        }

        Some(total / (*range as f32))
    }
}


#[cfg(test)]
mod tests {
    use horus_finance::Aggregate;

    use crate::sequences::sequence::Sequence;

    use super::QUEUE_SIZE;

    #[test]
    fn should_not_compute_sma_when_range_is_higher_then_length_of_sequence() {

        // Arrange
        let seq = Sequence::<Aggregate>::new();
        
        // Act
        let sma = seq.get_sma(&2);

        // Assert
        assert_eq!(None, sma);
    }

    #[test]
    fn should_compute_correct_sma_when_queue_is_partially_filled() {
        
        // Arrange
        let mut seq = Sequence::<Aggregate>::new();
        fill_sequence(&mut seq, 45);
        
        // Act
        let sma = seq.get_sma(&2);

        // Assert
        assert_eq!(Some(10.0), sma);
    }

    #[test]
    fn should_compute_correct_sma_after_queue_is_dequeued() {
        // Arrange
        let mut seq = Sequence::<Aggregate>::new();
        fill_sequence(&mut seq, QUEUE_SIZE);
        let aggregate = Aggregate {
            open: 10.0,
            close: 30.0
        };
        seq.enqueue(&aggregate);

        // Act
        let sma_5 = seq.get_sma(&5);
        let sma_2 = seq.get_sma(&2);

        // Assert
        assert_eq!(Some(14.0), sma_5);
        assert_eq!(Some(20.0), sma_2);
    }

    fn fill_sequence(sequence: &mut Sequence::<Aggregate>, size: usize) {

        for _ in 0..size-1{

            let aggregate = Aggregate {
                open: 10.0,
                close: 10.0
            };

            sequence.enqueue(&aggregate);
        }
    }
}