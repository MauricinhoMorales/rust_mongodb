use crate::{
    models::wallet_model::{CreateWalletRequest, IdWalletRequest, UpdateWalletRequest, Wallet},
    services::db::Database,
};
use actix_web::{
    delete, get, post, put,
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
            .expect("Error converting CreateWalletRequest to Wallet"),
        )
        .await
    {
        Ok(wallet) => HttpResponse::Ok().json(wallet),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[delete("/wallet")]
pub async fn delete_wallet(db: Data<Database>, request: Json<IdWalletRequest>) -> HttpResponse {
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

#[get("/wallet")]
pub async fn get_wallet(db: Data<Database>, request: Json<IdWalletRequest>) -> HttpResponse {
    match db.get_wallet(&request.id).await {
        Ok(get_result) => {
            if let Some(wallet) = get_result {
                HttpResponse::Ok().json(wallet)
            } else {
                HttpResponse::NotFound().body("Wallet not found")
            }
        }
        Err(err) => {
            HttpResponse::InternalServerError().body(format!("Error getting wallet: {}", err))
        }
    }
}

#[put("/wallet")]
pub async fn update_wallet(db: Data<Database>, request: Json<UpdateWalletRequest>) -> HttpResponse {
    match db
        .update_wallet(
            &request.id,
            Wallet::try_from(UpdateWalletRequest {
                id: request.id.clone(),
                name: request.name.clone(),
                amount: request.amount.clone(),
            })
            .expect("Error converting UpdateWalletRequest to Wallet"),
        )
        .await
    {
        Ok(wallet) => HttpResponse::Ok().json(wallet),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}
