use mongodb::{bson::doc, error::Result, options::IndexOptions, Client, Collection, IndexModel};

use crate::model::signature_model::{History, Signature};
#[derive(Clone)]
pub struct Database {
    pub signature: Collection<Signature>,
    pub histories:Collection<History>
}

impl Database {
    pub async fn init() -> Result<Self> {
        let url = "mongodb+srv://zhongxi1992:1FIZfgsoYDkS0Bg3@cluster0.x56nkq9.mongodb.net/?retryWrites=true&w=majority&appName=Cluster0".to_string();

        let client = Client::with_uri_str(url.clone()).await?;

        let db = client.database("signatures");

        let signature = db.collection("signature");
        let histories = db.collection("histories");
        
        let index_model = IndexModel::builder().keys(doc! {"signature":1}).options(IndexOptions::builder().unique(true).build()).build();

        histories.create_index(index_model, None).await.unwrap();
        Ok(Database { signature,histories })
    }

    // pub async fn check_connection() -> Result<()> {
    //     let url = "mongodb://149.51.230.248:27017".to_string();
    //     let client = Client::with_uri_str(url.clone()).await?;
    //     client.list_database_names(None,None).await?;
    //     Ok(())
    // }
}
