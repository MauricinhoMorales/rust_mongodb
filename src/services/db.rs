use dotenv::dotenv;
use mongodb::bson::oid::ObjectId;
use mongodb::{
    bson::{doc, extjson::de::Error},
    results::{DeleteResult, InsertOneResult},
    Client, Collection,
};

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

    pub async fn delete_wallet(&self, wallet_id: &str) -> Result<DeleteResult, Error> {
        match ObjectId::parse_str(wallet_id) {
            Ok(object_id) => {
                // Create a filter using the _id field
                let filter = doc! { "_id": object_id };

                // Attempt to delete the wallet and return the result
                let result = self
                    .wallets
                    .delete_one(filter)
                    .await
                    .ok()
                    .expect("Error deleting wallet");
                return Ok(result);
            }
            Err(_) => Err(Error::DeserializationError {
                message: String::from("Invalid ID"),
            }),
        }
    }
}
