use dotenv::dotenv;
use mongodb::{bson::extjson::de::Error, results::InsertOneResult, Client, Collection};

use crate::{config, models::wallet_model::Wallet};

pub struct Database {
    wallets: Collection<Wallet>,
}

impl Database {
    pub async fn init() -> Self {
        dotenv().ok();
        let env = config::init();
        let uri = env.mongo_uri;

        let client = Client::with_uri_str(uri).await.unwrap();
        let db = client.database("wallets");

        let wallets: Collection<Wallet> = db.collection("wallets");

        Database { wallets }
    }

    pub async fn create_wallet(&self, wallet: Wallet) -> Result<InsertOneResult, Error> {
        let result: InsertOneResult = self
            .wallets
            .insert_one(wallet)
            .await
            .ok()
            .expect("Error creating wallet");

        Ok(result)
    }
}
