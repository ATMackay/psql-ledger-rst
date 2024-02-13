#![allow(dead_code)]
// client wrappers using Atix Web Client (awc)
use crate::lib::models::{Account, Health, Status, Transaction};
use actix_web::Error;
use awc::Client;

pub async fn status(server_addr: String) -> Result<Status, Error> {
    // server_addr string must be of the form <ip>:<port>
    let url = format!("http://{}/status", server_addr);

    let client = Client::default();
    let mut response = client.get(&url).send().await.map_err(|e| {
        actix_web::error::ErrorInternalServerError(format!("Server response error: {}", e))
    })?;

    // Check if the request was successful
    if response.status().is_success() {
        let status: Status = response.json().await.map_err(|e| {
            actix_web::error::ErrorInternalServerError(format!(
                "Error converting response body: {}",
                e
            ))
        })?;
        Ok(status)
    } else {
        // If the request failed, return an error response
        Err(actix_web::error::ErrorInternalServerError(format!(
            "Got error response code: {}",
            response.status().as_str(),
        )))
    }
}

pub async fn health(server_addr: String) -> Result<Health, Error> {
    // server_addr string must be of the form <ip>:<port>
    let url = format!("http://{}/health", server_addr);

    let client = Client::default();
    let mut response = client.get(&url).send().await.map_err(|e| {
        actix_web::error::ErrorInternalServerError(format!("Server response error: {}", e))
    })?;

    // Check if the request was successful
    if response.status().is_success() {
        let health: Health = response.json().await.map_err(|e| {
            actix_web::error::ErrorInternalServerError(format!(
                "Error converting response body: {}",
                e
            ))
        })?;
        Ok(health)
    } else {
        // If the request failed, return an error response
        Err(actix_web::error::ErrorInternalServerError(format!(
            "Got error response code: {}",
            response.status().as_str(),
        )))
    }
}

pub async fn create_account(server_addr: String, account_params: Account) -> Result<Account, Error> {
    // server_addr string must be of the form <ip>:<port>
    let url = format!("http://{}/create_account", server_addr);

    // sanitize before sending
    let acc_pars = Account{
        id: Default::default(),
        username: account_params.username,
        email: account_params.email,
        balance: Default::default(),
    };

    let body_json = serde_json::to_string(&acc_pars).map_err(|e| {
        actix_web::error::ErrorInternalServerError(format!("Failed to serialize request body: {}", e))
    })?;

    let client = Client::default();

    let mut response = client.post(&url)
    .send_json(&body_json) // Send JSON body
    .await.map_err(|e| {
        actix_web::error::ErrorInternalServerError(format!("Server response error: {}", e))
    })?;

    // Check if the request was successful
    if response.status().is_success() {
        let account: Account = response.json().await.map_err(|e| {
            actix_web::error::ErrorInternalServerError(format!(
                "Error converting response body: {}",
                e
            ))
        })?;
        Ok(account)
    } else {
        // If the request failed, return an error response
        Err(actix_web::error::ErrorInternalServerError(format!(
            "Got error response code: {}",
            response.status().as_str(),
        )))
    }
}

pub async fn get_accounts(server_addr: String) -> Result<Vec<Account>, Error> {
    // server_addr string must be of the form <ip>:<port>
    let url = format!("http://{}/accounts", server_addr);

    let client = Client::default();
    let mut response = client.get(&url).send().await.map_err(|e| {
        actix_web::error::ErrorInternalServerError(format!("Server response error: {}", e))
    })?;

    // Check if the request was successful
    if response.status().is_success() {
        let accounts: Vec<Account> = response.json().await.map_err(|e| {
            actix_web::error::ErrorInternalServerError(format!(
                "Error converting response body: {}",
                e
            ))
        })?;
        Ok(accounts)
    } else {
        // If the request failed, return an error response
        Err(actix_web::error::ErrorInternalServerError(format!(
            "Got error response code: {}",
            response.status().as_str(),
        )))
    }
}

pub async fn get_transactions(server_addr: String) -> Result<Vec<Transaction>, Error> {
    // server_addr string must be of the form <ip>:<port>
    let url = format!("http://{}/transactions", server_addr);

    let client = Client::default();
    let mut response = client.get(&url).send().await.map_err(|e| {
        actix_web::error::ErrorInternalServerError(format!("Server response error: {}", e))
    })?;

    // Check if the request was successful
    if response.status().is_success() {
        let transactions: Vec<Transaction> = response.json().await.map_err(|e| {
            actix_web::error::ErrorInternalServerError(format!(
                "Error converting response body: {}",
                e
            ))
        })?;
        Ok(transactions)
    } else {
        // If the request failed, return an error response
        Err(actix_web::error::ErrorInternalServerError(format!(
            "Got error response code: {}",
            response.status().as_str(),
        )))
    }
}
