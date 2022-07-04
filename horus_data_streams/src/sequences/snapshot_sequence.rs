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

    pub fn get_current_bid(&self) -> Option<f32> {

        let newest = self.data.peek();

        match newest {
            Some(newest) => {
                Some(newest.current_bid)
            }
            _ => None
        }
    }

    pub fn get_current_ask(&self) -> Option<f32> {

        let newest = self.data.peek();

        match newest {
            Some(newest) => {
                Some(newest.current_ask)
            }
            _ => None
        }
    }
} 