use std::env;
use pinata_sdk::{PinByFile, PinByJson, PinataApi};
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Debug, Serialize, Deserialize)]
struct Attribute {
    trait_type: String,
    value: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Attributes {
    attributes: Vec<Attribute>,
}

pub async fn upload_pinata(transaction: String, block: String, fee: f64, number: u64) -> Result<String, String> {
    let pinata_api_key = env::var("PINATA_API_KEY").unwrap();
    let pinata_secret_key = env::var("PINATA_API_SECRET_KEY").unwrap();
    let nft_name = env::var("NFT_NAME").unwrap_or("FailSol".to_string());
    let description = env::var("DESCRIPTION").unwrap_or("This is an NFT on Solana".to_string());
    let api = PinataApi::new(&pinata_api_key, &pinata_secret_key).unwrap();

    // Test that you can connect to the API:
    let result = api.pin_file(PinByFile::new("output/result.png")).await;
    match result {
        Ok(pinned_object) => {
            let hash = pinned_object.ipfs_hash;
            let img_url = format!("https://white-giant-bird-563.mypinata.cloud/ipfs/{}", hash);

            let json = json!({
                "name": format!("{}#{}", nft_name, number),
                "description": description,
                "image": format!("https://white-giant-bird-563.mypinata.cloud/ipfs/{}", hash),
                "attributes": [
                    {"trait_type": "signature", "value": transaction},
                    {"trait_type": "block", "value": block},
                    {"trait_type": "fee", "value": fee},
                ],
                "properties": {
                    "files": [
                        {
                            "type": "image/png",
                            "uri": img_url
                        }
                    ]
                }
            });

            let json_result = api.pin_json(PinByJson::new(json)).await;
            match json_result {
                Ok(pinned_object) => {
                    let hash = pinned_object.ipfs_hash;
                    Ok(hash)
                },
                Err(e) => Err(format!("Failed to pin JSON: {}", e)),
            }
        },
        Err(e) => Err(format!("Failed to pin file: {}", e)),
    }
}
