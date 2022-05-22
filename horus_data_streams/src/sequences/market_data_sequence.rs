use heapless::spsc::Queue;
use horus_finance::Aggregate;

const QUEUE_SIZE: usize = 256;

pub struct MarketDataSequence {
    pub data: Queue<Aggregate, QUEUE_SIZE>
}

impl MarketDataSequence {

    pub fn new() -> MarketDataSequence {
        MarketDataSequence{
            data: Queue::new()
        }
    }

    pub fn enqueue(&mut self, next_aggregate: Aggregate) {

        if self.data.is_full() {
            self.data.dequeue().unwrap();
        }

        self.data.enqueue(next_aggregate).unwrap();
        // enqueue
        // update moving avg
        // update ...
    }

    pub fn get_sma(&self, range: &usize) -> Option<f32> {

        let current_size = self.data.len();

        if current_size < *range {
            return None
        }

        let start: usize = current_size - range;

        let mut total: f32 = 0.0;

        for index in start..current_size {

            total += self.data.iter().nth(index).unwrap().close;
        }

        Some(total / (*range as f32))
    }
}

#[cfg(test)]
mod tests {
    use horus_finance::Aggregate;

    use crate::sequences::market_data_sequence::MarketDataSequence;

    use super::QUEUE_SIZE;

    #[test]
    fn should_not_compute_sma_when_range_is_higher_then_length_of_sequence() {

        // Arrange
        let seq = MarketDataSequence::new();
        
        // Act
        let sma = seq.get_sma(&2);

        // Assert
        assert_eq!(None, sma);
    }

    #[test]
    fn should_compute_correct_sma_when_queue_is_partially_filled() {
        
        // Arrange
        let mut seq = MarketDataSequence::new();
        fill_sequence(&mut seq, 45);
        
        // Act
        let sma = seq.get_sma(&2);

        // Assert
        assert_eq!(Some(10.0), sma);
    }

    #[test]
    fn should_compute_correct_sma_after_queue_is_dequeued() {
        // Arrange
        let mut seq = MarketDataSequence::new();
        fill_sequence(&mut seq, QUEUE_SIZE);
        let aggregate = Aggregate {
            open: 10.0,
            close: 30.0
        };
        seq.enqueue(aggregate);

        // Act
        let sma_5 = seq.get_sma(&5);
        let sma_2 = seq.get_sma(&2);

        // Assert
        assert_eq!(Some(14.0), sma_5);
        assert_eq!(Some(20.0), sma_2);
    }

    fn fill_sequence(sequence: &mut MarketDataSequence, size: usize) {

        for _ in 0..size-1{

            let aggregate = Aggregate {
                open: 10.0,
                close: 10.0
            };

            sequence.enqueue(aggregate);
        }
    }
}