use super::{NetworkTradeable, Message, DeserializationErrors};
use serde::{Serialize, Deserialize};

pub const REQUEST_HEADER: &str = "UNSUB";
pub const REPLY_HEADER: &str = "UNSUB_REPL";

#[derive(Debug, Serialize, Deserialize)]
pub struct Request {
    pub sub_id: String
}

impl Request {
    pub fn new(sub_id: String) -> Request {
        Request {
            sub_id
        }
    }
}

impl NetworkTradeable<Request> for Request {
    fn as_message(&self) -> Message {
        Message::new(REQUEST_HEADER.to_string(), bson::to_bson(self).unwrap())
    }

    fn from_message(message: Message) -> Result<Request, DeserializationErrors> {
        if message.msg_type != REQUEST_HEADER {
            return Err(DeserializationErrors::IncompatibleMessageType);
        }
        match bson::from_bson(message.payload) {
            Ok(val) => Ok(val),
            Err(err) => Err(DeserializationErrors::InvalidMessageStructure(err.to_string()))
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Reply {
    pub sub_id: String,
    pub broker_id: String
}

impl Reply {
    pub fn new(sub_id: String, broker_id: String) -> Reply {
        Reply {
            sub_id: sub_id,
            broker_id: broker_id
        }
    }
}

impl NetworkTradeable<Reply> for Reply {
    fn as_message(&self) -> Message {
        Message::new(REPLY_HEADER.to_string(), bson::to_bson(self).unwrap())
    }

    fn from_message(message: Message) -> Result<Reply, DeserializationErrors> {
        if message.msg_type != REPLY_HEADER {
            return Err(DeserializationErrors::IncompatibleMessageType);
        }
        match bson::from_bson(message.payload) {
            Ok(val) => Ok(val),
            Err(err) => Err(DeserializationErrors::InvalidMessageStructure(err.to_string()))
        }
    }
}
