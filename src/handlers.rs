use crate::{
    db,
    model::{Account, AccountParams, Health, Status, Transaction, TransactionParams},
};
use actix_web::{web, Error, HttpResponse};
use chrono::Utc;
use deadpool_postgres::{Client, Pool};

// status always responds ok if the service is live and listening for requests
pub async fn status() -> Result<HttpResponse, Error> {
    let status_response: Status = Status {
        service: env!("SERVICE_NAME").to_string(),
        message: "OK".to_string(),
        version: env!("VERSION").to_string(),
    };
    Ok(HttpResponse::Ok().json(status_response))
}

// health pings the postgres database, returning a 503 status code if the postgres ping fails.
pub async fn health(db_pool: web::Data<Pool>) -> Result<HttpResponse, Error> {
    let mut health_response: Health = Health {
        service: env!("SERVICE_NAME").to_string(),
        version: env!("VERSION").to_string(),
        failures: Vec::new(),
    };

    let client: Client = match db_pool.get().await {
        Ok(client) => client,
        Err(err) => {
            health_response.failures = vec![err.to_string()];
            return Ok(HttpResponse::ServiceUnavailable().json(health_response));
        }
    };

    match db::ping_db(&client).await {
        Ok(_) => Ok(HttpResponse::Ok().json(health_response)),
        Err(err) => {
            health_response.failures = vec![err.to_string()];
            Ok(HttpResponse::ServiceUnavailable().json(health_response))
        }
    }
}

// get accounts returns the full (non-paginated) list of user accounts from the
// postgres DB.
pub async fn get_accounts(db_pool: web::Data<Pool>) -> Result<HttpResponse, Error> {
    let client: Client = match db_pool.get().await {
        Ok(client) => client,
        Err(err) => {
            let response: Status = Status {
                service: env!("SERVICE_NAME").to_string(),
                message: err.to_string(),
                version: env!("VERSION").to_string(),
            };
            return Ok(HttpResponse::ServiceUnavailable().json(response));
        }
    };

    let users = match db::get_accounts(&client).await {
        Ok(users) => users,
        Err(err) => {
            let response: Status = Status {
                service: env!("SERVICE_NAME").to_string(),
                message: err.to_string(),
                version: env!("VERSION").to_string(),
            };
            return Ok(HttpResponse::InternalServerError().json(response));
        }
    };

    Ok(HttpResponse::Ok().json(users))
}

// get_account_by_id returns the account details for the account with specified index.
pub async fn get_account_by_id(
    account_params: web::Json<AccountParams>,
    db_pool: web::Data<Pool>,
) -> Result<HttpResponse, Error> {
    let account_info: AccountParams = account_params.into_inner();

    let mut response: Status = Status {
        service: env!("SERVICE_NAME").to_string(),
        message: "".to_string(),
        version: env!("VERSION").to_string(),
    };

    // check user supplied parameters
    if account_info.id.is_none() {
        response.message = "No id supplied".to_string();
        return Ok(HttpResponse::BadRequest().json(response));
    }

    let client: Client = match db_pool.get().await {
        Ok(client) => client,
        Err(err) => {
            response.message = err.to_string();
            return Ok(HttpResponse::ServiceUnavailable().json(response));
        }
    };

    let acc = match db::get_account_by_id(&client, account_info.id.unwrap()).await {
        // will panic id id not supplied - fix
        Ok(acc) => acc,
        Err(err) => {
            response.message = err.to_string();
            if err.to_string() == "NotFound" {
                return Ok(HttpResponse::NotFound().json(response));
            }
            return Ok(HttpResponse::InternalServerError().json(response));
        }
    };

    Ok(HttpResponse::Ok().json(acc))
}

// get_transaction_by_id returns the transaction details for the transaction with specified index.
pub async fn get_transaction_by_id(
    tx_params: web::Json<TransactionParams>,
    db_pool: web::Data<Pool>,
) -> Result<HttpResponse, Error> {
    let tx_info: TransactionParams = tx_params.into_inner();

    let mut response: Status = Status {
        service: env!("SERVICE_NAME").to_string(),
        message: "".to_string(),
        version: env!("VERSION").to_string(),
    };

    // check user supplied parameters
    if tx_info.id.is_none() {
        response.message = "No id supplied".to_string();
        return Ok(HttpResponse::BadRequest().json(response));
    }

    let client: Client = match db_pool.get().await {
        Ok(client) => client,
        Err(err) => {
            response.message = err.to_string();
            return Ok(HttpResponse::ServiceUnavailable().json(response));
        }
    };

    let acc = match db::get_transaction_by_id(&client, tx_info.id.unwrap()).await {
        // will panic id id not supplied - fix
        Ok(acc) => acc,
        Err(err) => {
            response.message = err.to_string();
            if err.to_string() == "NotFound" {
                return Ok(HttpResponse::NotFound().json(response));
            }
            return Ok(HttpResponse::InternalServerError().json(response));
        }
    };

    Ok(HttpResponse::Ok().json(acc))
}

// create_account registers a new account to the server. Provided the
// PostgesDB write is successful it will return the account details back to the request agent.
pub async fn create_account(
    account_params: web::Json<AccountParams>,
    db_pool: web::Data<Pool>,
) -> Result<HttpResponse, Error> {
    let account_info: AccountParams = account_params.into_inner();

    let mut response: Status = Status {
        service: env!("SERVICE_NAME").to_string(),
        message: "".to_string(),
        version: env!("VERSION").to_string(),
    };

    // check user supplied values
    if account_info.email.is_none() {
        response.message = "No email supplied".to_string();
        return Ok(HttpResponse::BadRequest().json(response));
    }
    if account_info.username.is_none() {
        response.message = "No username supplied".to_string();
        return Ok(HttpResponse::BadRequest().json(response));
    }
    // Set timestamp server-side
    let dt = Utc::now();
    let account: Account = Account {
        id: None, // To be set by Postgres
        username: account_info.username,
        email: account_info.email,
        balance: Some(0),
        created_at: Some(dt),
    };

    let client: Client = match db_pool.get().await {
        Ok(client) => client,
        Err(err) => {
            response.message = err.to_string();
            return Ok(HttpResponse::InternalServerError().json(response));
        }
    };

    let new_account = match db::create_account(&client, account).await {
        Ok(new_account) => new_account,
        Err(err) => {
            response.message = err.to_string();
            return Ok(HttpResponse::InternalServerError().json(response));
        }
    };

    Ok(HttpResponse::Ok().json(new_account))
}

// get_transactions queries the full list of transactions from the postgres DB and returns
// to the request agent.
pub async fn get_transactions(db_pool: web::Data<Pool>) -> Result<HttpResponse, Error> {
    let mut response: Status = Status {
        service: env!("SERVICE_NAME").to_string(),
        message: "".to_string(),
        version: env!("VERSION").to_string(),
    };
    let client: Client = match db_pool.get().await {
        Ok(client) => client,
        Err(err) => {
            response.message = err.to_string();
            return Ok(HttpResponse::InternalServerError().json(response));
        }
    };

    let txs = match db::get_transactions(&client).await {
        Ok(txs) => txs,
        Err(err) => {
            response.message = err.to_string();
            return Ok(HttpResponse::InternalServerError().json(response));
        }
    };

    Ok(HttpResponse::Ok().json(txs))
}

// create_transaction posts a new transaction to the postgres DB and returns
// the transaction details with unique ID to the request agent.
pub async fn create_transaction(
    tx_params: web::Json<TransactionParams>,
    db_pool: web::Data<Pool>,
) -> Result<HttpResponse, Error> {
    let tx_info: TransactionParams = tx_params.into_inner();

    let mut response: Status = Status {
        service: env!("SERVICE_NAME").to_string(),
        message: "".to_string(),
        version: env!("VERSION").to_string(),
    };

    // check user supplied values
    if tx_info.from_account.is_none() {
        response.message = "No from account supplied".to_string();
        return Ok(HttpResponse::BadRequest().json(response));
    }
    if tx_info.to_account.is_none() {
        response.message = "No to account supplied".to_string();
        return Ok(HttpResponse::BadRequest().json(response));
    }
    if tx_info.amount.is_none() {
        response.message = "No amount supplied".to_string();
        return Ok(HttpResponse::BadRequest().json(response));
    }
    // Set timestamp server-side
    let dt = Utc::now();
    let tx: Transaction = Transaction {
        id: None, // To be set by Postgres
        from_account: tx_info.from_account,
        to_account: tx_info.to_account,
        amount: tx_info.amount,
        created_at: Some(dt),
    };

    let client: Client = match db_pool.get().await {
        Ok(client) => client,
        Err(err) => {
            response.message = err.to_string();
            return Ok(HttpResponse::InternalServerError().json(response));
        }
    };

    let new_tx = match db::create_transaction(&client, tx).await {
        Ok(new_tx) => new_tx,
        Err(err) => {
            response.message = err.to_string();
            return Ok(HttpResponse::InternalServerError().json(response));
        }
    };

    Ok(HttpResponse::Ok().json(new_tx))
}
