use crate::consumer::Consumer;
use crate::fs::backend::Backend;
use crossbeam_channel::{bounded, Receiver, Sender};
use crossbeam_utils::thread;
use std::{thread as std_thread, time};

pub struct ThreadConsumer {
    pub sender: Sender<String>,
    pub receiver: Receiver<String>,
}

impl Consumer for ThreadConsumer {
    fn new() -> Self {
        let (sender, receiver) = bounded(3);
        ThreadConsumer {
            sender: sender,
            receiver: receiver,
        }
    }

    fn consume(&mut self, backend: &(impl Backend + std::marker::Sync)) {
        thread::scope(|s| {
            // pass the channel sender to consume messages from broker
            s.spawn(|_| {
                backend.pull(&self.sender);
            });

            for i in 0..3 {
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
