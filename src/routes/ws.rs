use actix::Addr;
use actix_web::{get, web::{self}, HttpRequest, HttpResponse};
use actix_web_actors::ws;

use crate::services::ws::{lobby::WsServer, WsConn};


#[get("/ws")]
pub async fn get_ws(
    req: HttpRequest,
    stream: web::Payload,
    srv: web::Data<Addr<WsServer>>,
) -> Result<HttpResponse, actix_web::Error> {
    ws::start(WsConn::new(srv.get_ref().clone()), &req, stream)
}