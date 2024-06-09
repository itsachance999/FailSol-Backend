use std::time::Instant;

use crate::{
    model::signature_model::UpdateRequest,
    services::{
        db::Database,
        ws::{
            lobby::WsServer,
            messages::{BroadcastMessage, BroadcastMessages},
        },
    },
};
use actix::Addr;
use actix_web::{
    get, post,
    web::{self, Json},
    HttpResponse,
};
use futures_util::StreamExt;
use mongodb::{
    bson::{doc, Timestamp},
    options::FindOptions,
};
use serde::Deserialize;

#[post("/created")]
pub async fn update_img_url(
    db: web::Data<Database>,
    req: Json<Vec<UpdateRequest>>,
    srv: web::Data<Addr<WsServer>>,
) -> HttpResponse {
    let update_data = req.into_inner();
    let mut messages = Vec::new();
    for update_d in update_data {
        match db
            .histories
            .find_one_and_update(
                doc! {"signature": &update_d.signature},
                doc! {"$set": {"img_url": &update_d.img_url}},
                None,
            )
            .await
        {
            Ok(history) => match history {
                Some(_his) => {
                    messages.push(BroadcastMessage {
                        signature: _his.signature,
                        address: _his.address,
                        timestamp: _his.timestamp,
                    });

                    continue;
                }
                None => return HttpResponse::InternalServerError().finish(),
            },
            Err(e) => return HttpResponse::InternalServerError().body(e.to_string()),
        }
    }

    srv.do_send(BroadcastMessages { messages });
    HttpResponse::Ok().body("success")
}

#[derive(Deserialize)]
struct PaginationParams {
    page: Option<u32>,
    page_size: Option<u32>,
}

#[get("/get_histories")]
pub async fn get_histories(
    db: web::Data<Database>,
    web::Query(pagination): web::Query<PaginationParams>,
) -> HttpResponse {
    let page = pagination.page.unwrap_or(1);
    let page_size = pagination.page_size.unwrap_or(10);
    let skip = (page - 1) * page_size;

    let find_options = FindOptions::builder()
        .skip(skip as u64)
        .limit(page_size as i64).sort(doc! {"timestamp":-1})
        .build();
    match db.histories.find(None, find_options).await {
        Ok(mut cursor) => {
            let mut histories = Vec::new();
            while let Some(result) = cursor.next().await {
                match result {
                    Ok(document) => histories.push(document),
                    Err(e) => return HttpResponse::InternalServerError().body(e.to_string()),
                }
            }
            HttpResponse::Ok().json(histories)
        }
        Err(e) => return HttpResponse::InternalServerError().body(e.to_string()),
    }
}
