use crate::{
    models::wallet_model::{CreateWalletRequest, DeleteWalletRequest, Wallet},
    services::db::Database,
};
use actix_web::{
    delete, post,
    web::{Data, Json},
    HttpResponse,
};

#[post("/wallet")]
pub async fn create_wallet(db: Data<Database>, request: Json<CreateWalletRequest>) -> HttpResponse {
    match db
        .create_wallet(
            Wallet::try_from(CreateWalletRequest {
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

#[delete("/wallet")]
pub async fn delete_wallet(db: Data<Database>, request: Json<DeleteWalletRequest>) -> HttpResponse {
    match db.delete_wallet(&request.id).await {
        Ok(delete_result) => {
            if delete_result.deleted_count > 0 {
                HttpResponse::Ok().body("Wallet deleted successfully")
            } else {
                HttpResponse::NotFound().body("Wallet not found")
            }
        }
        Err(err) => {
            HttpResponse::InternalServerError().body(format!("Error deleting wallet: {}", err))
        }
    }
}
