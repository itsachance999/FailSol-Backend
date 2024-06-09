use actix::{prelude::{Actor, Context, Handler, Recipient}, Message};
use actix_web_actors::ws::{self};
use std::collections::{HashMap, HashSet};
use uuid::Uuid;

use super::messages::{BroadcastMessage, BroadcastMessages, Connect, Disconnect, WsMessage};



pub struct WsServer {
    sessions: HashMap<usize, Recipient<WsMessage>>, //self id to self
    next_id:usize
}



impl WsServer {
    pub fn new() -> Self {
        Self {
            sessions:HashMap::new(),
            next_id:0
        }
    }
    

    fn broadcast_message(&self,message: &Vec<BroadcastMessage>) {
        let message_json = serde_json::to_string(message).unwrap_or_default();
        for  session in self.sessions.values() {
            let _ = session.do_send(WsMessage(message_json.to_owned()));
        }
    }
}

impl Actor for WsServer {
    type Context = Context<Self>;
}

impl Handler<Connect> for WsServer {
    type Result = usize;

    fn handle(&mut self, msg: Connect, _: &mut Self::Context) -> Self::Result {
        let id = self.next_id;
        self.next_id += 1;
        self.sessions.insert(id, msg.addr);
        id
    }
}

impl Handler<Disconnect> for WsServer {
    type Result = ();

    fn handle(&mut self, msg: Disconnect, _: &mut Self::Context) {
        self.sessions.remove(&msg.id);
    }
}

impl Handler<BroadcastMessages> for WsServer {
    type Result = ();

    fn handle(&mut self, msg: BroadcastMessages, _: &mut Self::Context) {
        self.broadcast_message(&msg.messages);
    }
}