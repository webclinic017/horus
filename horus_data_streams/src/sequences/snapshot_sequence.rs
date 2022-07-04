use std::cell::RefCell;

use heapless::spsc::Queue;
use horus_finance::market_snapshot::MarketSnapshot;

pub struct SnapshotSequence<const SIZE: usize> {
    data: Queue<MarketSnapshot, SIZE>,
}

impl<const SIZE: usize> SnapshotSequence<SIZE> {
    
    pub fn enqueue(&mut self, new_snapshot: MarketSnapshot) {

        let mut dequeued: Option<MarketSnapshot> = None;
        if self.data.is_full() {
            dequeued = self.data.dequeue();
        }

        let _ = self.data.enqueue(new_snapshot);
    }
} 