use crate::fs::backend::Backend;
use crate::fs::message::Message;
use amiquip::{
    Channel, Connection, ConsumerMessage, ConsumerOptions, Exchange, Publish, QueueDeclareOptions,
};

// RabbitMQBackend
// Implements the Backend trait to enable
// communication with a RabbitMQ message queue
pub struct RabbitMQBackend {
    pub connection: Connection,
    pub channel: Channel,
    pub queue_name: &'static str,
}

// Implementing Backend the trait functions for RabbitMQ Backend
impl Backend for RabbitMQBackend {
    fn new(conn_string: &'static str, queue_name: &'static str) -> Self {
        let mut connection = Connection::insecure_open(conn_string).unwrap();
        let channel = connection.open_channel(None).unwrap();

        let _ = channel.queue_declare(
            queue_name,
            QueueDeclareOptions {
                durable: true,
                ..QueueDeclareOptions::default()
            },
        );

        RabbitMQBackend {
            connection: connection,
            channel: channel,
            queue_name: queue_name,
        }
    }

    fn close(self) {
        self.connection.close().unwrap()
    }

    fn push(&self, message: Message) {
        let exchange = Exchange::direct(&self.channel);
        exchange
            .publish(Publish::new(&message.to_bytes(), self.queue_name))
            .unwrap();
    }

    fn pull(&self) {
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

        let consumer = queue.consume(ConsumerOptions::default()).unwrap();
        for (i, message) in consumer.receiver().iter().enumerate() {
            match message {
                ConsumerMessage::Delivery(delivery) => {
                    let body = String::from_utf8_lossy(&delivery.body);
                    println!("({:>3}) Received [{}]", i, body);
                    consumer.ack(delivery).unwrap();
                }
                other => {
                    println!("No More Messages. Sleeping for 3 seconds: {:?}", other);
                    break;
                }
            }
        }
    }
}
