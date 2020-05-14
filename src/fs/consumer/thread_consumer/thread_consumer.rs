use crate::fs::backend::Backend;
use crate::fs::consumer::Consumer;
use crossbeam_channel::{bounded, Receiver, Sender};
use crossbeam_utils::thread;
use std::{thread as std_thread, time};

pub struct ThreadConsumer {
    pub sender: Sender<String>,
    pub receiver: Receiver<String>,
    pub worker_count: usize,
}

impl Consumer for ThreadConsumer {
    fn new(worker_count: usize) -> Self {
        let (sender, receiver) = bounded(worker_count);
        ThreadConsumer {
            sender: sender,
            receiver: receiver,
            worker_count: worker_count,
        }
    }

    fn consume(&mut self, backend: &(impl Backend + std::marker::Sync)) {
        thread::scope(|s| {
            // pass the channel sender to consume messages from broker
            s.spawn(|_| {
                backend.pull(&self.sender);
            });

            for i in 0..self.worker_count {
                let th_receiver = self.receiver.clone();
                s.spawn(move |_| loop {
                    let message = th_receiver.recv().unwrap();
                    if message.is_empty() {
                        println!(
                            "Thread {} - No Message Received. Sleeping for 3 seconds.",
                            i
                        );
                        std_thread::sleep(time::Duration::from_secs(3));
                    } else {
                        println!("Thread {} - Message Received {}", i, message);
                    }
                });
            }
        })
        .unwrap();
    }
}
