use crate::{
    models::wallet_model::{Wallet, WalletRequest},
    services::db::Database,
};
use actix_web::{
    post,
    web::{Data, Json},
    HttpResponse,
};

#[post("/wallet")]
pub async fn create_wallet(db: Data<Database>, request: Json<WalletRequest>) -> HttpResponse {
    match db
        .create_wallet(
            Wallet::try_from(WalletRequest {
                name: request.name.clone(),
                amount: request.amount.clone(),
            })
            .expect("Error converting WalletRequest to Wallet"),
        )
        .await
    {
        Ok(wallet) => HttpResponse::Ok().json(wallet),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}
