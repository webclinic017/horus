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

        todo!()
    }

    pub fn get_rate_of_change(&self) -> Option<f32> {
        
        let current_size: usize = self.data.borrow().len().try_into().unwrap();

        if current_size < SIZE {
            return None
        }

        todo!()
    }

    pub fn get_average_true_range(&self) -> Option<f32> {

        let current_size: usize = self.data.borrow().len().try_into().unwrap();

        if current_size < SIZE {
            return None
        }

        todo!()
    }

    pub fn get_momentum(&self) -> Option<f32> {

        let current_size: usize = self.data.borrow().len().try_into().unwrap();

        if current_size < SIZE {
            return None
        }

        todo!()
    }
}

#[cfg(test)]
mod moving_average_tests {

    use horus_finance::Aggregate;

    use crate::sequences::sequence::Sequence;

    #[test]
    fn should_not_compute_moving_average_when_sequence_is_not_ready() {

        // Arrange
        let seq = Sequence::<Aggregate, 10>::new();
        
        // Act
        let moving_average = seq.get_moving_average();

        // Assert
        assert_eq!(None, moving_average);
    }
}

#[cfg(test)]
mod rate_of_change_tests {

    use horus_finance::Aggregate;

    use crate::sequences::sequence::Sequence;

    #[test]
    fn should_not_compute_rate_of_change_when_sequence_is_not_ready() {

        // Arrange
        let seq = Sequence::<Aggregate, 10>::new();
        
        // Act
        let rate_of_change = seq.get_rate_of_change();

        // Assert
        assert_eq!(None, rate_of_change);
    }
 }

#[cfg(test)]
mod average_true_range_tests {

    use horus_finance::Aggregate;

    use crate::sequences::sequence::Sequence;

    #[test]
    fn should_not_compute_average_true_range_when_sequence_is_not_ready() {

        // Arrange
        let seq = Sequence::<Aggregate, 10>::new();
        
        // Act
        let average_true_range = seq.get_average_true_range();

        // Assert
        assert_eq!(None, average_true_range);
    }
 }

#[cfg(test)]
mod momentum_tests {

    use horus_finance::Aggregate;

    use crate::sequences::sequence::Sequence;

    #[test]
    fn should_not_compute_momentum_when_sequence_is_not_ready() {

        // Arrange
        let seq = Sequence::<Aggregate, 10>::new();
        
        // Act
        let momentum = seq.get_momentum();

        // Assert
        assert_eq!(None, momentum);
    }
 }

// mod test_sequences {
//     use horus_finance::Aggregate;
//     use crate::sequences::sequence::Sequence;
 
//     pub fn simulate_linear_falling(sequence: &Sequence::<Aggregate, 10>) {

//         for i in 0..9 {

//             let aggregate = Aggregate {
//                 open: 11. - i as f32,
//                 close: 10. - i as f32
//             };

//             sequence.enqueue(&aggregate);
//         }
//     }

//     pub fn simulate_linear_growing(sequence: &Sequence::<Aggregate, 10>) {

//         for i in 1..10 {

//             let aggregate = Aggregate {
//                 open: i as f32,
//                 close: i as f32 + 1.
//             };

//             sequence.enqueue(&aggregate);
//         }
//     }

//     pub fn simulate_moving_market(sequence: &Sequence::<Aggregate, 10>) {

//         for i in 1..10 {

//             let mut open: f32 = 0.;
//             let mut close: f32 = 0.;

//             if i % 2 != 0 {
//                 open = 6.;
//                 close = 3.;
//             } else {
//                 open = 3.;
//                 close = 6.;
//             }

//             let aggregate = Aggregate {
//                 open,
//                 close,
//             };

//             sequence.enqueue(&aggregate);
//         }
//     }

//     pub fn simulate_stable_market(sequence: &Sequence::<Aggregate, 10>) {

//         for i in 1..10 {

//             let aggregate = Aggregate {
//                 open: 10.,
//                 close: 10.,
//             };

//             sequence.enqueue(&aggregate);
//         }
//     } 
// }