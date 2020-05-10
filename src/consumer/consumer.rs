use crate::fs::backend::Backend;

pub trait Consumer {
    fn new() -> Self;
    fn start(backend: &impl Backend);
}
