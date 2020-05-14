extern crate clap;
extern crate crossbeam_channel;
extern crate crossbeam_utils;
extern crate log;
extern crate serde;
extern crate serde_json;

// Initializing Modules
mod cli;
mod fs;

use cli::cli_builder;
use fs::backend::{Backend, RabbitMQBackend};
use fs::config::Config;
use fs::consumer::{Consumer, ThreadConsumer};
use fs::message::Message;

fn publish_message() {
    let rmq_backend = RabbitMQBackend::new("amqp://user:password@localhost:5672/", "testqueue2");
    for _ in 0..1000 {
        let message = Message::new("some_callback");
        println!("Message: {}", message.to_string());
        rmq_backend.push(message);
    }
    rmq_backend.close();
}

fn subscribe_messsage() {
    let rmq_backend = RabbitMQBackend::new("amqp://user:password@localhost:5672/", "testqueue2");
    let num_threads = 3;
    let mut consumer = ThreadConsumer::new(num_threads);
    consumer.consume(&rmq_backend);
    rmq_backend.close();
}

fn main() {
    let cli_app = cli_builder();
    let _config = Config::load_config(&cli_app.config);
    if cli_app.worker {
        subscribe_messsage();
    } else {
        publish_message();
    }
}
