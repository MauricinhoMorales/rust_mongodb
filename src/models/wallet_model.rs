use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Wallet {
    pub _id: ObjectId,
    pub name: String,
    pub amount: i64,
}

#[derive(Debug, Deserialize)]
pub struct CreateWalletRequest {
    pub name: String,
    pub amount: i64,
}

impl TryFrom<CreateWalletRequest> for Wallet {
    type Error = Box<dyn std::error::Error>;

    fn try_from(item: CreateWalletRequest) -> Result<Self, Self::Error> {
        Ok(Self {
            _id: ObjectId::new(),
            name: item.name,
            amount: item.amount,
        })
    }
}

#[derive(Debug, Deserialize)]
pub struct DeleteWalletRequest {
    pub id: String,
}
