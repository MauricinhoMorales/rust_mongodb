use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Wallet {
    pub _id: ObjectId,
    pub name: String,
    pub amount: u64,
}

#[derive(Debug, Deserialize)]
pub struct WalletRequest {
    pub name: String,
    pub amount: u64,
}

impl TryFrom<WalletRequest> for Wallet {
    type Error = Box<dyn std::error::Error>;

    fn try_from(item: WalletRequest) -> Result<Self, Self::Error> {
        Ok(Self {
            _id: ObjectId::new(),
            name: item.name,
            amount: item.amount,
        })
    }
}
