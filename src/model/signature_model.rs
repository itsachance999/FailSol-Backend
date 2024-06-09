use std::error::Error;

use chrono::Utc;
use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
use solana_sdk::signature::Signature as SolSignature;
// use solana_transaction_status::EncodedConfirmedTransactionWithStatusMeta;
#[derive(Debug, Deserialize, Serialize)]
pub struct Signature {
    pub _id: String,
    pub status: bool,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct History {
    pub _id: ObjectId,
    pub status: bool,
    pub signature:String,
    pub img_url:String,
    pub address:String,
    pub timestamp:String,
}

#[derive(Debug, Deserialize, Serialize)]

pub struct CreateRequest {
   pub signature:SolSignature,
  pub  address:String
}

impl TryFrom<CreateRequest> for History {
    type Error = Box<dyn Error>;

    fn try_from(value: CreateRequest) -> Result<Self, Self::Error> {
        Ok(Self {
            _id:ObjectId::new(),
            status:false,
            signature:value.signature.to_string(),
            img_url:String::new(),
            address:value.address.to_string(),
            timestamp:Utc::now().to_rfc3339()
        })
    }
}
#[derive(Debug, Deserialize, Serialize)]
pub struct HistoryRequest {
    pub signature:String
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ImageQuery {
    pub name: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ResponseModel {
    // pub transaction: EncodedConfirmedTransactionWithStatusMeta,
    pub hash: String,
    pub number:u64,
    pub signature:String
}
#[derive(Debug, Deserialize, Serialize)]
pub struct UpdateRequest {
    // pub transaction: EncodedConfirmedTransactionWithStatusMeta,
    #[serde(rename="imgUrl")]
    pub img_url:String,
    pub signature:String,
}
#[derive(Debug, Deserialize, Serialize)]

pub struct CreateRequestWithAddress {
    pub address:String
}




