use actix::{Message, Recipient};
use serde::{Deserialize, Serialize};

// Define a custom message type
#[derive(Message)]
#[rtype(result = "()")]
pub struct WsMessage(pub String);

#[derive(Message)]
#[rtype(usize)]
pub struct Connect {
    pub addr: Recipient<WsMessage>,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct Disconnect {
    pub id: usize,
}

#[derive(Message,Serialize,Deserialize)]
#[rtype(result = "()")]
pub struct BroadcastMessage {
    pub signature: String,
    pub address:String,
    pub timestamp:String
}

#[derive(Message,Serialize,Deserialize)]
#[rtype(result = "()")]
pub struct BroadcastMessages {
    pub messages: Vec<BroadcastMessage>,
}