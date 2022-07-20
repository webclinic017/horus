use heapless::spsc::Queue;
use horus_finance::market_snapshot::MarketSnapshot;

pub struct SnapshotSequence<const SIZE: usize> {
    data: Queue<MarketSnapshot, SIZE>,
}

impl<const SIZE: usize> SnapshotSequence<SIZE> {
    
    pub fn enqueue(&mut self, new_snapshot: MarketSnapshot) {

        let _ = self.data.enqueue(new_snapshot);
    }

    pub fn get_current_bid(&self) -> Option<f32> {

        let newest = self.data.peek();

        newest.map(|newest| newest.current_bid)
    }

    pub fn get_current_ask(&self) -> Option<f32> {

        let newest = self.data.peek();

        newest.map(|newest| newest.current_ask)
    }
} 