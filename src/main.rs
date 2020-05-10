// Initialize Macros
// #[macro_use]
extern crate serde;
extern crate serde_json;

// Initializing Modules
mod consumer;
mod fs;

use consumer::{Consumer, ThreadConsumer};
use fs::backend::{Backend, RabbitMQBackend};
use fs::message::Message;

fn publish_message() {
    let rmq_backend = RabbitMQBackend::new("amqp://user:password@localhost:5672/", "testqueue2");
    let message = Message::new("some_callback");
    println!("Message: {}", message.to_string());
    rmq_backend.push(message);
    rmq_backend.close()
}

fn subscribe_messsage() {
    let rmq_backend = RabbitMQBackend::new("amqp://user:password@localhost:5672/", "testqueue2");
    ThreadConsumer::start(&rmq_backend);
    rmq_backend.close();
}

fn main() {
    publish_message();
    // subscribe_messsage();
}
