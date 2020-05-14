use crate::fs::backend::Backend;
use crate::fs::message::Message;
use amiquip::{
    Channel, Connection, ConsumerMessage, ConsumerOptions, Exchange, Publish, QueueDeclareOptions,
};
use crossbeam_channel::Sender;
use log::{debug, info};

// RabbitMQBackend
// Implements the Backend trait to enable
// communication with a RabbitMQ message queue
pub struct RabbitMQBackend {
    pub connection: Connection,
    pub channel: Channel,
    pub queue_name: &'static str,
}

unsafe impl Sync for RabbitMQBackend {}

// Implementing Backend the trait functions for RabbitMQ Backend
impl Backend for RabbitMQBackend {
    fn new(conn_string: &'static str, queue_name: &'static str) -> Self {
        let mut connection = Connection::insecure_open(conn_string).unwrap();
        debug!("Connected to RabbitMQ Backend");
        let channel = connection.open_channel(None).unwrap();
        let _ = channel.queue_declare(
            queue_name,
            QueueDeclareOptions {
                durable: true,
                ..QueueDeclareOptions::default()
            },
        );
        debug!("Initialized Queue {}", queue_name);
        RabbitMQBackend {
            connection: connection,
            channel: channel,
            queue_name: queue_name,
        }
    }

    fn close(self) {
        debug!("Closing RabbitMQ Backend Connection");
        self.connection.close().unwrap();
        debug!("Closed RabbitMQ Backend Connection");
    }

    fn push(&self, message: Message) {
        let exchange = Exchange::direct(&self.channel);
        info!("Sending Message {:?} To Queue {}", message, self.queue_name);
        exchange
            .publish(Publish::new(&message.to_bytes(), self.queue_name))
            .unwrap();
    }

    fn pull(&self, sender: &Sender<String>) {
        let queue = self
            .channel
            .queue_declare(
                self.queue_name,
                QueueDeclareOptions {
                    durable: true,
                    ..QueueDeclareOptions::default()
                },
            )
            .unwrap();

        info!("Listening To Queue {}", self.queue_name);
        let consumer = queue.consume(ConsumerOptions::default()).unwrap();
        for (_, message) in consumer.receiver().iter().enumerate() {
            match message {
                ConsumerMessage::Delivery(delivery) => {
                    let body = String::from_utf8_lossy(&delivery.body);
                    sender.send(body.to_owned().to_string()).unwrap();
                    consumer.ack(delivery).unwrap();
                }
                other => {
                    println!("Message Type not recognized: {:?}", other);
                    break;
                }
            }
        }
    }
}
