use crate::fs::message::Message;

// Backend trait
// Modularized trait which enables easy backend integrations.
// Every backend only needs to implement this trait functions to be used with ruscel.
pub trait Backend {
    // Returns a new connection object.
    // This connection object is used to send and recieve message from the queue.
    fn new(conn_string: &'static str, queue_name: &'static str) -> Self;

    // Closes an active connection
    fn close(self);

    // Send a message to the connection.
    fn push(&self, message: Message);

    // Start consuming message from the queue.
    fn pull(&self);
}
