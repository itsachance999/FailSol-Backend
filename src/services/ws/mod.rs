use actix::{fut, ActorContext, ActorFutureExt, ContextFutureSpawner, WrapFuture};
use actix::{Actor, Addr, StreamHandler};
use actix::{AsyncContext, Handler};
use actix_web_actors::ws::{self, Message, ProtocolError};
use self::lobby::WsServer;
use self::messages::{ Connect, Disconnect, WsMessage};
pub mod lobby;
pub mod messages;

pub struct WsConn {
    
    id: usize,
    server_addr:Addr<WsServer>
}

impl WsConn {
   pub fn new(server_addr:Addr<WsServer>) -> Self {
       Self {id:0,server_addr}
   }
}

impl Actor for WsConn {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        let addr = ctx.address();
        self.server_addr
            .send(Connect {
                addr: addr.recipient(),
            })
            .into_actor(self)
            .then(|res, act, ctx| {
                match res {
                    Ok(id) => act.id = id,
                    _ => ctx.stop(),
                }
                fut::ready(())
            })
            .wait(ctx);
    }

    fn stopping(&mut self, _: &mut Self::Context) -> actix::Running {
        self.server_addr.do_send(Disconnect { id: self.id });
        actix::Running::Stop
    }
}



impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WsConn {
    fn handle(&mut self, msg: Result<Message, ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Ping(msg)) => ctx.pong(&msg),
            Ok(ws::Message::Text(text)) => ctx.text(text),
            Ok(ws::Message::Binary(bin)) => ctx.binary(bin),
            _ => (),
        }
    }
}

impl Handler<WsMessage> for WsConn {
    type Result = ();

    fn handle(&mut self, msg: WsMessage, ctx: &mut Self::Context) {
        
        ctx.text(msg.0);
    }
}
