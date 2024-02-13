use crate::{
    lib::constants,
    lib::db,
    lib::errors::MyError,
    lib::models::{Account, Health, Status, Transaction},
};
use actix_web::{web, Error, HttpResponse};
use deadpool_postgres::{Client, Pool};

// status always responds ok if the service is live and listening for requests
pub async fn status() -> Result<HttpResponse, Error> {
    let health_response: Status = Status {
        service: constants::service_name(),
        message: "OK".to_string(),
        version: constants::full_version(),
    };
    Ok(HttpResponse::Ok().json(health_response))
}

// health pings the postgres database, returning a 503 status code if the postgres ping fails.
pub async fn health(db_pool: web::Data<Pool>) -> Result<HttpResponse, Error> {
    let client: Client = db_pool.get().await.map_err(MyError::PoolError)?;

    let mut failures = Vec::new();

    match db::ping_db(&client).await {
        Ok(_) => {
            let health_response: Health = Health {
                service: constants::service_name(),
                message: "OK".to_string(),
                version: constants::full_version(),
                failures: failures,
            };
            Ok(HttpResponse::Ok().json(health_response))
        }
        Err(err) => {
            failures.push(err.to_string());
            let health_response: Health = Health {
                service: constants::service_name(),
                message: "FAILURES".to_string(),
                version: constants::full_version(),
                failures: failures,
            };
            Ok(HttpResponse::InternalServerError().json(health_response))
        }
    }
}

pub async fn get_accounts(db_pool: web::Data<Pool>) -> Result<HttpResponse, Error> {
    let client: Client = db_pool.get().await.map_err(MyError::PoolError)?;

    let users = db::get_accounts(&client).await?;

    Ok(HttpResponse::Ok().json(users))
}

pub async fn get_account_by_id(
    account_params: web::Json<Account>,
    db_pool: web::Data<Pool>,
) -> Result<HttpResponse, Error> {
    let client: Client = db_pool.get().await.map_err(MyError::PoolError)?;

    let account_info: Account = account_params.into_inner();
    let acc = db::get_account_by_id(&client, account_info.id).await?;

    Ok(HttpResponse::Ok().json(acc))
}

pub async fn create_account(
    account_params: web::Json<Account>,
    db_pool: web::Data<Pool>,
) -> Result<HttpResponse, Error> {
    let account_info: Account = account_params.into_inner();

    let client: Client = db_pool.get().await.map_err(MyError::PoolError)?;

    let new_account = db::create_account(&client, account_info).await?;

    Ok(HttpResponse::Ok().json(new_account))
}

pub async fn get_transactions(db_pool: web::Data<Pool>) -> Result<HttpResponse, Error> {
    let client: Client = db_pool.get().await.map_err(MyError::PoolError)?;

    let txs = db::get_transactions(&client).await?;

    Ok(HttpResponse::Ok().json(txs))
}

pub async fn create_transaction(
    tx_params: web::Json<Transaction>,
    db_pool: web::Data<Pool>,
) -> Result<HttpResponse, Error> {
    let tx_info: Transaction = tx_params.into_inner();

    let client: Client = db_pool.get().await.map_err(MyError::PoolError)?;

    let new_tx = db::create_transaction(&client, tx_info).await?;

    Ok(HttpResponse::Ok().json(new_tx))
}
