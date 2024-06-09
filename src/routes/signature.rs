use std::{fmt::Debug, path::PathBuf, str::FromStr};

use actix_files::NamedFile;
use actix_web::{
    get, post,
    web::{self, Data, Json, Path},
    HttpRequest, HttpResponse, Result,
};
use futures_util::StreamExt;
use mongodb::{
    bson::{doc, oid::ObjectId},
    options::FindOptions,
};
use solana_client::{rpc_client::RpcClient, rpc_config::RpcTransactionConfig};
use solana_sdk::{commitment_config::CommitmentConfig, signature::Signature as SolSignature};
use solana_transaction_status::{EncodedTransaction, UiMessage, UiTransactionEncoding};

use crate::{
    model::signature_model::{CreateRequest, CreateRequestWithAddress, History, HistoryRequest, ResponseModel},
    services::db::Database,
    utils::{generate_image::generate::generate, transaction::fetch_transaction::get_failed_tx},
};

#[get("/signature/{count}")]
async fn get_signatures_handler(db: web::Data<Database>, path: Path<usize>,web::Query(address):web::Query<CreateRequestWithAddress>) -> HttpResponse {
    let count: usize = path.into_inner();
    let address = address.address.clone();
    let temp_db = db.clone();
    let signatures = get_failed_tx(count).await.unwrap();
    let mut responses = Vec::new();
    for signature in signatures {
        let payload = CreateRequest{signature,address:address.clone()};
        match temp_db
            .clone()
            .histories
            .insert_one(History::try_from(payload).unwrap(), None)
            .await
            .ok()
        {
            Some(_) => {
                let fetched_transaction = web::block(move || {
                    let client = RpcClient::new("https://api.mainnet-beta.solana.com");
                    // let signature = SolSignature::from_str(&result._id.clone()).unwrap();
                    let config = RpcTransactionConfig {
                        encoding: Some(UiTransactionEncoding::Json),
                        commitment: Some(CommitmentConfig::confirmed()),
                        max_supported_transaction_version: Some(0),
                    };
                    client.get_transaction_with_config(&signature, config)
                })
                .await;
                match fetched_transaction {
                    Ok(transaction) => match transaction {
                        Ok(t) => {
                            let s = &t.transaction.transaction;
                            let (signature, signer) = match s {
                                EncodedTransaction::Json(ui_transaction) => {
                                    let signer = match &ui_transaction.message {
                                        UiMessage::Raw(ui_message) => &ui_message.account_keys[0],
                                        _ => {
                                            todo!()
                                        }
                                    };
                                    (ui_transaction.signatures[0].clone(), signer)
                                }
                                _ => {
                                    todo!()
                                }
                            };
                            let fee = t.transaction.meta.clone().unwrap().fee;
                            let log_messages = match t.transaction.meta.clone().unwrap().log_messages {
                                        solana_transaction_status::option_serializer::OptionSerializer::Some(e) => {e},
                                        solana_transaction_status::option_serializer::OptionSerializer::None => todo!(),
                                        solana_transaction_status::option_serializer::OptionSerializer::Skip => todo!(), };
                            match generate(db.clone(), &signature,t.slot,t.block_time, &signer,fee,log_messages,).await {
                                Ok((hash,number)) => {
                                    let response = ResponseModel {
                                        // transaction: t,
                                        hash,
                                        number,
                                        signature
                                    };
                                    responses.push(response);
                                },
                                Err(e) => {return  HttpResponse::InternalServerError().body(e.to_string());},
                            }
                            
                           
                        }
                        Err(e) => {
                            eprintln!("Error{:?}", e);
                           return  HttpResponse::InternalServerError().body(e.to_string());
                        }
                    },
                    Err(e) => {
                        eprintln!("Error fetching transaction: {:?}", e);
                       return  HttpResponse::InternalServerError().body(e.to_string());
                    }
                }
            }
            None => continue,
        };
    }
    HttpResponse::Ok().json(responses)

    // Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
}

// async fn get_and_update_signatures(
//     db: &web::Data<Database>,
//     count: i64,
// ) -> Result<Vec<Signature>, Box<dyn std::error::Error>> {
//     let filter = doc! { "status": false };
//     let update = doc! { "$set": { "status": true } };
//     let find_options = FindOptions::builder().limit(count).build();

//     let mut cursor = db.signature.find(filter.clone(), find_options).await?;
//     let mut signatures = Vec::new();

//     while let Some(result) = cursor.next().await {
//         match result {
//             Ok(signature) => {
//                 // Update the document status
//                 db.signature
//                     .update_one(doc! { "_id": &signature._id }, update.clone(), None)
//                     .await?;
//                 signatures.push(signature);
//             }
//             Err(e) => {
//                 eprintln!("Error retrieving document: {}", e);
//             }
//         }
//     }

//     Ok(signatures)
// }

// #[get("/signature/{count}")]
// pub async fn get_signature(db: Data<Database>,path:Path<u8>) -> HttpResponse {
//     let count:u8 = path.into_inner();
//     println!("count=={}",count);
//     if let Some(result) = db
//         .signature
//         .find_oneand_update(
//             doc! { "status": false },
//             doc! { "$set": { "status": true } },
//             None,
//         )
//         .await
//         .ok()
//         .expect("get error")
//     {

//         let fetched_transaction = web::block(move || {
//             let client = RpcClient::new("https://api.mainnet-beta.solana.com");
//             let signature = SolSignature::from_str(&result._id.clone()).unwrap();
//             let config = RpcTransactionConfig {
//                 encoding: Some(UiTransactionEncoding::Json),
//                 commitment: Some(CommitmentConfig::confirmed()),
//                 max_supported_transaction_version: Some(0),
//             };
//             client.get_transaction_with_config(&signature, config)
//         })
//         .await;

//         match fetched_transaction {
//             Ok(transaction) => {
//                 match transaction {
//                     Ok(t) => {
//                         let s = &t.transaction.transaction;
//                         let (signature, signer) = match s {
//                             EncodedTransaction::Json(ui_transaction) => {
//                                 let signer = match &ui_transaction.message {
//                                     UiMessage::Raw(ui_message) => &ui_message.account_keys[0],
//                                     _ => {
//                                         todo!()
//                                     }
//                                 };
//                                 (ui_transaction.signatures[0].clone(), signer)
//                             }
//                             _ => {
//                                 todo!()
//                             }
//                         };
//                         let fee = t.transaction.meta.clone().unwrap().fee;
//                         let log_messages = match t.transaction.meta.clone().unwrap().log_messages {
//                             solana_transaction_status::option_serializer::OptionSerializer::Some(e) => {e},
//                             solana_transaction_status::option_serializer::OptionSerializer::None => todo!(),
//                             solana_transaction_status::option_serializer::OptionSerializer::Skip => todo!(), };
//                        let hash = generate(&signature, t.slot, t.block_time, &signer, fee,log_messages).await.unwrap();
//                         let response = ResponseModel{transaction:t,hash};
//                         HttpResponse::Ok().json(response)
//                     }
//                     Err(e) => {
//                         eprintln!("Error{:?}", e);
//                         HttpResponse::InternalServerError().finish()
//                     }
//                 }
//             }
//             Err(e) => {
//                 eprintln!("Error fetching transaction: {:?}", e);
//                 HttpResponse::InternalServerError().finish()
//             }
//         }
//     } else {
//         HttpResponse::InternalServerError().body("getError".to_string())
//     }
// }

#[get("/image")]
pub async fn get_image(db: Data<Database>) -> Result<NamedFile> {
    // let file_name = format!(".output/result.png",query.name );
    let path = PathBuf::from("./output/result.png");

    Ok(NamedFile::open(path)?)
}

// #[post("/created")]
// pub async fn created_nft(db:Data<Database>,data:Json<HistoryRequest>) -> HttpResponse {
//     match db.histories.update_many(doc! {"signature":data.signature}, update, options) {

//     }
// }

#[get("/sp_signature/{tx}")]
pub async fn get_specific_signature(db: web::Data<Database>, path: Path<String>,web::Query(address):web::Query<CreateRequestWithAddress>) -> HttpResponse {
    let signature: String = path.into_inner();
    let address = address.address;
    let temp_db = db.clone();
    match SolSignature::from_str(&signature) {
        Ok(signature) => {
            let mut responses = Vec::new();
            let status = web::block(move || {
                let client = RpcClient::new("https://api.mainnet-beta.solana.com");
                client.get_signature_status(&signature)
            })
            .await;

            match status {
                Ok(stat) => {
                    match stat {
                        Ok(status) => match status {
                            Some(result) => match result {
                                Ok(_) => HttpResponse::InternalServerError()
                                    .body("The signature is not failed transaction".to_string()),
                                Err(e) => {
                                    let payload = CreateRequest { signature,address:address.clone()};
                                    match temp_db
                                        .clone()
                                        .histories
                                        .insert_one(History::try_from(payload).unwrap(), None)
                                        .await
                                    {
                                        Ok(_) => {
                                            let fetched_transaction = web::block(move || {
                                                let client = RpcClient::new(
                                                    "https://api.mainnet-beta.solana.com",
                                                );
                                                // let signature = SolSignature::from_str(&result._id.clone()).unwrap();
                                                let config = RpcTransactionConfig {
                                                    encoding: Some(UiTransactionEncoding::Json),
                                                    commitment: Some(CommitmentConfig::confirmed()),
                                                    max_supported_transaction_version: Some(0),
                                                };
                                                client
                                                    .get_transaction_with_config(&signature, config)
                                            })
                                            .await;
                                            match fetched_transaction {
                                                Ok(transaction) => match transaction {
                                                    Ok(t) => {
                                                        let s = &t.transaction.transaction;
                                                        let (signature, signer) = match s {
                                                            EncodedTransaction::Json(
                                                                ui_transaction,
                                                            ) => {
                                                                let signer = match &ui_transaction
                                                                    .message
                                                                {
                                                                    UiMessage::Raw(ui_message) => {
                                                                        &ui_message.account_keys[0]
                                                                    }
                                                                    _ => {
                                                                        todo!()
                                                                    }
                                                                };
                                                                (
                                                                    ui_transaction.signatures[0]
                                                                        .clone(),
                                                                    signer,
                                                                )
                                                            }
                                                            _ => {
                                                                todo!()
                                                            }
                                                        };
                                                        let fee =
                                                            t.transaction.meta.clone().unwrap().fee;
                                                        let log_messages = match t.transaction.meta.clone().unwrap().log_messages {
                                           solana_transaction_status::option_serializer::OptionSerializer::Some(e) => {e},
                                           solana_transaction_status::option_serializer::OptionSerializer::None => todo!(),
                                           solana_transaction_status::option_serializer::OptionSerializer::Skip => todo!(), };
                                                        match generate(db.clone(), &signature,t.slot,t.block_time, &signer,fee,log_messages,).await {
                                                            Ok((hash,number)) => {

                                                                let response = ResponseModel {
                                                                    // transaction: t,
                                                                    hash,
                                                                    number,
                                                                    signature
                                                                };
                                                                responses.push(response);
                                                                HttpResponse::Ok().json(responses)
                                                            },
                                                            Err(e) => {
                                                                HttpResponse::InternalServerError().body(e.to_string())
                                                            },
                                                        }
                                                        
                                                    }
                                                    Err(e) => {
                                                        eprintln!("Error{:?}", e);
                                                        HttpResponse::InternalServerError()
                                                            .body(e.to_string())
                                                    }
                                                },
                                                Err(e) => {
                                                    eprintln!(
                                                        "Error fetching transaction: {:?}",
                                                        e
                                                    );
                                                    HttpResponse::InternalServerError()
                                                        .body(e.to_string())
                                                }
                                            }
                                        }
                                        Err(e) => HttpResponse::InternalServerError()
                                            .body("The signature already exist".to_string()),
                                    }
                                }
                            },
                            None => HttpResponse::InternalServerError()
                                .body("The signature is not failed signature".to_string()),
                        },
                        Err(e) => HttpResponse::InternalServerError()
                            .body("The signature is not exist".to_string()),
                    }
                }
                Err(e) => {
                    println!("err={:?}", e);
                    HttpResponse::InternalServerError().body(e.to_string())
                }
            }
        }

        Err(e) => {
            println!("err={:?}", e);
            HttpResponse::InternalServerError().body(e.to_string())
        }
    }
}
