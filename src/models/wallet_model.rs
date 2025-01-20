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
pub struct IdWalletRequest {
    pub id: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdateWalletRequest {
    pub id: String,
    pub name: String,
    pub amount: i64,
}

impl TryFrom<UpdateWalletRequest> for Wallet {
    type Error = Box<dyn std::error::Error>;

    fn try_from(item: UpdateWalletRequest) -> Result<Self, Self::Error> {
        Ok(Self {
            _id: ObjectId::parse_str(item.id.as_str())?,
            name: item.name,
            amount: item.amount,
        })
    }
}
