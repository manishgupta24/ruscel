use crate::fs::backend::Backend;

// Consumer trait
// Can be of different type async, thread, etc.
pub trait Consumer {
    fn new(worker_count: usize) -> Self;
    fn consume(&mut self, backend: &impl Backend);
}
