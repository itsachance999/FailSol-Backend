use std::str::FromStr;

use actix_web::{error::BlockingError, web};
use solana_client::rpc_client::RpcClient;
use solana_sdk::{pubkey::Pubkey, signature::Signature};

pub async fn get_failed_tx(count: usize) -> Result<Vec<Signature>,BlockingError> {
    web::block(move || {

        let client = RpcClient::new("https://api.mainnet-beta.solana.com");
        // let account_address = Pubkey::from_str("So11111111111111111111111111111111111111112").unwrap();
        let address = Pubkey::from_str("So11111111111111111111111111111111111111112").unwrap();
        let mut failed_signs = Vec::new();
        let signatures = client.get_signatures_for_address(&address).unwrap();
    
        for signature in signatures {
            let sign = Signature::from_str(&signature.signature).unwrap();
            let status = client.get_signature_status(&sign);
    
            match status {
                Ok(status) => match status {
                    Some(result) => match result {
                        Ok(_) => {
                            continue;
                        }
                        Err(e) => {
                            failed_signs.push(sign);
                            if failed_signs.len() == count {
                                break;
                            }
                        }
                    },
                    None => {
                        continue;
                    }
                },
                Err(e) => {
                    continue;
                }
            }
        }
        return failed_signs;
    }).await

}
