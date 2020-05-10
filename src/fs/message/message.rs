use serde::{Deserialize, Serialize};
use uuid::Uuid;

// Message struct
// Stores the content and details of a message/task object.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub uuid: Uuid,
    callback: &'static str,
}

// Implement the Message struct
// Implements some basic functions like new, to_json etc.
impl Message {
    // Creates and returns a new message object
    pub fn new(callback: &'static str) -> Message {
        Message {
            uuid: Uuid::new_v4(),
            callback: callback,
        }
    }

    // Returns message object as a json string
    pub fn to_string(&self) -> String {
        serde_json::to_string(self).unwrap()
    }

    // Returns message objects to bytes
    pub fn to_bytes(&self) -> Vec<u8> {
        serde_json::to_vec(self).unwrap()
    }
}
