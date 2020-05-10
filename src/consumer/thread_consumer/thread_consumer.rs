use crate::consumer::Consumer;
use crate::fs::backend::Backend;
use std::{
    sync::{Arc, Mutex},
    thread,
};

pub struct ThreadConsumer {}

impl Consumer for ThreadConsumer {
    fn new() -> Self {
        ThreadConsumer {}
    }

    fn start(backend: &impl Backend) {
        backend.pull();
        // let mut consumers = Vec::new();

        // let shared_backend = Arc::new(Mutex::new(backend));
        // for _ in 0..3 {
        //     let th_backend = shared_backend.clone();
        //     let consumer = thread::spawn(move || {
        //         // let shared = th_backend.lock().unwrap();
        //         th_backend. .pull();
        //     });
        //     consumers.push(consumer);
        // }
        // for consumer in consumers {
        //     consumer.join().expect("Consumer Crashed !!!");
        // }
    }
}
