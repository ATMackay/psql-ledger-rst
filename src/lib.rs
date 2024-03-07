pub mod constants {
    use std::env;

    pub fn build_date() -> String {
        let mut build_date = String::new();
        if let Some(b) = env::var("BUILD_DATE").ok() {
            build_date = b
        };
        build_date
    }

    pub fn service_name() -> String {
        let mut service_name = String::new();
        if let Some(s) = env::var("SERVICE_NAME").ok() {
            service_name = s
        };
        service_name
    }

    pub fn full_version() -> String {
        let mut version = String::new();
        if let Some(v) = env::var("VERSION").ok() {
            if let Some(g) = env::var("GIT_COMMIT").ok() {
                version = format!("{}-{}", v, g)
            };
        };
        version
    }
}

pub mod config {
    #![allow(dead_code)]
    // Service configuration definition
    // with methods to read from file or environment
    // variables
    use deadpool_postgres::Config as PgConfig;
    use deadpool_postgres::SslMode;
    use serde::Deserialize;
    use std::fs::File;
    use std::io::{BufReader, Error};

    #[derive(Debug, Default, Deserialize)]
    pub struct Config {
        pub log_level: String,
        pub server_addr: String,
        pub pg: PgConfig,
    }

    pub fn default_config() -> Config {
        let mut cfg = Config {
            log_level: "info".to_string(),
            server_addr: "localhost:8080".to_string(),
            pg: PgConfig::default(),
        };

        let default_host = "localhost".to_string();
        let default_port = 5432;
        let default_dbname = "bank".to_string();
        let default_user = "root".to_string();
        let default_pswd = "secret".to_string();

        // set vars
        cfg.pg.host = Some(default_host);
        cfg.pg.port = Some(default_port);
        cfg.pg.dbname = Some(default_dbname);
        cfg.pg.user = Some(default_user);
        cfg.pg.password = Some(default_pswd);

        cfg.pg.ssl_mode = Some(SslMode::Disable);

        cfg
    }

    impl Config {
        // read config from .json file
        pub fn from_file(filename: &str) -> Result<Self, Error> {
            let file = File::open(filename)?;
            let reader = BufReader::new(file);

            let config: Config = serde_json::from_reader(reader)?;

            Ok(config)
        }

        // read config from ENV
        pub fn from_env() -> Result<Self, envy::Error> {
            envy::from_env::<Config>()
        }
    }
}

pub mod models {
    use chrono::{DateTime, Utc};
    use serde::{Deserialize, Deserializer, Serialize, Serializer}; //
    use tokio_pg_mapper_derive::PostgresMapper;

    #[derive(Deserialize, PostgresMapper, Serialize, Debug)]
    #[pg_mapper(table = "accounts")]
    pub struct Account {
        pub id: i64,
        pub username: String,
        pub email: String,
        pub balance: i64,
        #[serde(
            serialize_with = "serialize_datetime",
            deserialize_with = "deserialize_datetime"
        )]
        pub created_at: DateTime<Utc>,
    }
    #[derive(Deserialize, PostgresMapper, Serialize, Debug)]
    #[pg_mapper(table = "transactions")]
    pub struct Transaction {
        pub id: i64,
        pub from_account: i64,
        pub to_account: i64,
        pub amount: i64,
        #[serde(
            serialize_with = "serialize_datetime",
            deserialize_with = "deserialize_datetime"
        )]
        pub created_at: DateTime<Utc>,
    }

    // status represents the default JSON
    // response format (also used to encode error messages)
    #[derive(Deserialize, Serialize, Debug)]
    pub struct Status {
        pub service: String,
        pub version: String,
        pub message: String,
    }

    #[derive(Deserialize, Serialize, Debug)]
    pub struct Health {
        pub service: String,
        pub version: String,
        pub message: String,
        pub failures: Vec<String>,
    }

    // Custom serialization function for DateTime<Utc>
    fn serialize_datetime<S>(date: &DateTime<Utc>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = date.to_rfc3339();
        serializer.serialize_str(&s)
    }

    // Custom deserialization function for DateTime<Utc>
    fn deserialize_datetime<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        s.parse::<DateTime<Utc>>().map_err(serde::de::Error::custom)
    }
}

pub mod db {
    use deadpool_postgres::Client;
    use tokio_pg_mapper::FromTokioPostgresRow;

    use super::{
        errors::MyError,
        models::{Account, Transaction},
    };

    pub async fn ping_db(client: &Client) -> Result<(), MyError> {
        let _ = client.query_one("SELECT NOW()", &[]).await?;

        Ok(())
    }

    pub async fn get_accounts(client: &Client) -> Result<Vec<Account>, MyError> {
        let stmt = "SELECT * FROM accounts ORDER BY id";
        let stmt = stmt.replace("$table_fields", &Account::sql_table_fields());
        let stmt = client.prepare(&stmt).await.unwrap();

        let results = client
            .query(&stmt, &[])
            .await?
            .iter()
            .map(|row| Account::from_row_ref(row).unwrap())
            .collect::<Vec<Account>>();

        Ok(results)
    }

    pub async fn get_account_by_id(client: &Client, account_id: i64) -> Result<Account, MyError> {
        let stmt = "SELECT * FROM accounts WHERE id = $1 LIMIT 1";
        let stmt = stmt.replace("$table_fields", &Account::sql_table_fields());
        let stmt = client.prepare(&stmt).await.unwrap();

        client
            .query(&stmt, &[&account_id])
            .await?
            .iter()
            .map(|row| Account::from_row_ref(row).unwrap())
            .collect::<Vec<Account>>()
            .pop()
            .ok_or(MyError::NotFound)
    }

    pub async fn get_transaction_by_id(
        client: &Client,
        account_id: i64,
    ) -> Result<Transaction, MyError> {
        let stmt = "SELECT * FROM transactions WHERE id = $1 LIMIT 1";
        let stmt = stmt.replace("$table_fields", &Account::sql_table_fields());
        let stmt = client.prepare(&stmt).await.unwrap();

        client
            .query(&stmt, &[&account_id])
            .await?
            .iter()
            .map(|row| Transaction::from_row_ref(row).unwrap())
            .collect::<Vec<Transaction>>()
            .pop()
            .ok_or(MyError::NotFound)
    }

    pub async fn create_account(
        client: &Client,
        account_info: Account,
    ) -> Result<Account, MyError> {
        let _stmt = "INSERT INTO accounts (
                username, balance, email
            ) VALUES (
                $1, $2, $3
            )
            RETURNING *";
        let _stmt = _stmt.replace("$table_fields", &Account::sql_table_fields());
        let stmt = client.prepare(&_stmt).await.unwrap();

        client
            .query(&stmt, &[&account_info.email, &account_info.username])
            .await?
            .iter()
            .map(|row| Account::from_row_ref(row).unwrap())
            .collect::<Vec<Account>>()
            .pop()
            .ok_or(MyError::NotFound) // more applicable for SELECTs
    }

    pub async fn get_transactions(client: &Client) -> Result<Vec<Transaction>, MyError> {
        let stmt = "SELECT * FROM transactions ORDER BY id";
        let stmt = stmt.replace("$table_fields", &Transaction::sql_table_fields());
        let stmt = client.prepare(&stmt).await.unwrap();

        let results = client
            .query(&stmt, &[])
            .await?
            .iter()
            .map(|row| Transaction::from_row_ref(row).unwrap())
            .collect::<Vec<Transaction>>();

        Ok(results)
    }

    pub async fn create_transaction(
        client: &Client,
        transaction_info: Transaction,
    ) -> Result<Transaction, MyError> {
        let _stmt = "INSERT INTO transactions (
                from_account, to_account, amount
            ) VALUES (
                $1, $2, $3
            )
            RETURNING *";
        let _stmt = _stmt.replace("$table_fields", &Transaction::sql_table_fields());
        let stmt = client.prepare(&_stmt).await.unwrap();

        client
            .query(
                &stmt,
                &[
                    &transaction_info.from_account,
                    &transaction_info.to_account,
                    &transaction_info.amount,
                ],
            )
            .await?
            .iter()
            .map(|row| Transaction::from_row_ref(row).unwrap())
            .collect::<Vec<Transaction>>()
            .pop()
            .ok_or(MyError::NotFound) // more applicable for SELECTs
    }
}

pub mod errors {
    use actix_web::{HttpResponse, ResponseError};
    use deadpool_postgres::PoolError;
    use derive_more::{Display, From};
    use tokio_pg_mapper::Error as PGMError;
    use tokio_postgres::error::Error as PGError;

    #[derive(Display, From, Debug)]
    pub enum MyError {
        NotFound,
        PGError(PGError),
        PGMError(PGMError),
        PoolError(PoolError),
    }

    impl std::error::Error for MyError {}

    impl ResponseError for MyError {
        fn error_response(&self) -> HttpResponse {
            match *self {
                MyError::NotFound => HttpResponse::NotFound().finish(),
                MyError::PoolError(ref err) => {
                    HttpResponse::InternalServerError().body(err.to_string())
                }
                _ => HttpResponse::InternalServerError().finish(),
            }
        }
    }
}

pub mod handlers {
    use super::{
        constants, db,
        models::{Account, Health, Status, Transaction},
    };
    use actix_web::{web, Error, HttpResponse};
    use chrono::Utc;
    use deadpool_postgres::{Client, Pool};

    // status always responds ok if the service is live and listening for requests
    pub async fn status() -> Result<HttpResponse, Error> {
        //println!("STATUS PINGED");
        let status_response: Status = Status {
            service: constants::service_name(),
            message: "OK".to_string(),
            version: constants::full_version(),
        };
        Ok(HttpResponse::Ok().json(status_response))
    }

    // health pings the postgres database, returning a 503 status code if the postgres ping fails.
    pub async fn health(db_pool: web::Data<Pool>) -> Result<HttpResponse, Error> {
        let mut failures = Vec::new();

        //println!("HEALTH PINGED");

        let client: Client = match db_pool.get().await {
            Ok(client) => client,
            Err(err) => {
                failures.push(err.to_string());
                let health_response: Health = Health {
                    service: constants::service_name(),
                    message: "FAILURES".to_string(),
                    version: constants::full_version(),
                    failures: failures,
                };
                return Ok(HttpResponse::ServiceUnavailable().json(health_response));
            }
        };

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
                Ok(HttpResponse::ServiceUnavailable().json(health_response))
            }
        }
    }

    // get accounts returns the full (non-paginated) list of user accounts from the
    // postgres DB.
    pub async fn get_accounts(db_pool: web::Data<Pool>) -> Result<HttpResponse, Error> {

        //println!("GET ACCOUNTS PINGED");

        let client: Client = match db_pool.get().await {
            Ok(client) => client,
            Err(err) => {
                let response: Status = Status {
                    service: constants::service_name(),
                    message: err.to_string(),
                    version: constants::full_version(),
                };
                return Ok(HttpResponse::ServiceUnavailable().json(response));
            }
        };

        let users = match db::get_accounts(&client).await {
            Ok(users) => users,
            Err(err) => {
                let response: Status = Status {
                    service: constants::service_name(),
                    message: err.to_string(),
                    version: constants::full_version(),
                };
                return Ok(HttpResponse::InternalServerError().json(response));
            }
        };

        Ok(HttpResponse::Ok().json(users))
    }

    // get_account_by_id returns the account details for the account with specified index.
    pub async fn get_account_by_id(
        account_params: web::Json<Account>,
        db_pool: web::Data<Pool>,
    ) -> Result<HttpResponse, Error> {

        let account_info: Account = account_params.into_inner();
        //println!("GET ACCOUNT BY ID: {:?}", account_info);

        let client: Client = match db_pool.get().await {
            Ok(client) => client,
            Err(err) => {
                let response: Status = Status {
                    service: constants::service_name(),
                    message: err.to_string(),
                    version: constants::full_version(),
                };
                return Ok(HttpResponse::ServiceUnavailable().json(response));
            }
        };



        let acc = match db::get_account_by_id(&client, account_info.id).await {
            Ok(acc) => acc,
            Err(err) => {
                let response: Status = Status {
                    service: constants::service_name(),
                    message: err.to_string(),
                    version: constants::full_version(),
                };
                return Ok(HttpResponse::InternalServerError().json(response));
            }
        };

        Ok(HttpResponse::Ok().json(acc))
    }

    // get_transaction_by_id returns the transaction details for the transaction with specified index.
    pub async fn get_transaction_by_id(
        tx_params: web::Json<Transaction>,
        db_pool: web::Data<Pool>,
    ) -> Result<HttpResponse, Error> {

        let tx_info: Transaction = tx_params.into_inner();
        //println!("TX BY ID PINGED: {:?}", tx_info);

        let client: Client = match db_pool.get().await {
            Ok(client) => client,
            Err(err) => {
                let response: Status = Status {
                    service: constants::service_name(),
                    message: err.to_string(),
                    version: constants::full_version(),
                };
                return Ok(HttpResponse::ServiceUnavailable().json(response));
            }
        };

        let acc = match db::get_transaction_by_id(&client, tx_info.id).await {
            Ok(acc) => acc,
            Err(err) => {
                let response: Status = Status {
                    service: constants::service_name(),
                    message: err.to_string(),
                    version: constants::full_version(),
                };
                return Ok(HttpResponse::InternalServerError().json(response));
            }
        };

        Ok(HttpResponse::Ok().json(acc))
    }

    // create_account registers a new account to the server. Provided the
    // PostgesDB write is successful it will return the account details back to the request agent.
    pub async fn create_account(
        account_params: web::Json<Account>,
        db_pool: web::Data<Pool>,
    ) -> Result<HttpResponse, Error> {
        let mut account_info: Account = account_params.into_inner();

        // Set timestamp server-side
        let dt = Utc::now();
        account_info.created_at = dt;

        //println!("CREATE ACCOUNT PINGED: {:?}", account_info);

        let client: Client = match db_pool.get().await {
            Ok(client) => client,
            Err(err) => {
                let response: Status = Status {
                    service: constants::service_name(),
                    message: err.to_string(),
                    version: constants::full_version(),
                };
                return Ok(HttpResponse::InternalServerError().json(response));
            }
        };

        let new_account = match db::create_account(&client, account_info).await {
            Ok(new_account) => new_account,
            Err(err) => {
                let response: Status = Status {
                    service: constants::service_name(),
                    message: err.to_string(),
                    version: constants::full_version(),
                };
                return Ok(HttpResponse::InternalServerError().json(response));
            }
        };

        Ok(HttpResponse::Ok().json(new_account))
    }

    // get_transactions queries the full list of transactions from the postgres DB and returns
    // to the request agent.
    pub async fn get_transactions(db_pool: web::Data<Pool>) -> Result<HttpResponse, Error> {
        let client: Client = match db_pool.get().await {
            Ok(client) => client,
            Err(err) => {
                let response: Status = Status {
                    service: constants::service_name(),
                    message: err.to_string(),
                    version: constants::full_version(),
                };
                return Ok(HttpResponse::InternalServerError().json(response));
            }
        };

        let txs = match db::get_transactions(&client).await {
            Ok(txs) => txs,
            Err(err) => {
                let response: Status = Status {
                    service: constants::service_name(),
                    message: err.to_string(),
                    version: constants::full_version(),
                };
                return Ok(HttpResponse::InternalServerError().json(response));
            }
        };

        Ok(HttpResponse::Ok().json(txs))
    }

    // create_transaction posts a new transaction to the postgres DB and returns
    // the transaction details with unique ID to the request agent.
    pub async fn create_transaction(
        tx_params: web::Json<Transaction>,
        db_pool: web::Data<Pool>,
    ) -> Result<HttpResponse, Error> {
        let mut tx_info: Transaction = tx_params.into_inner();

        // Set timestamp server-side
        let dt = Utc::now();
        tx_info.created_at = dt;

        //println!("CREATE TX PINGED: {:?}", tx_info);

        let client: Client = match db_pool.get().await {
            Ok(client) => client,
            Err(err) => {
                let response: Status = Status {
                    service: constants::service_name(),
                    message: err.to_string(),
                    version: constants::full_version(),
                };
                return Ok(HttpResponse::InternalServerError().json(response));
            }
        };

        let new_tx = match db::create_transaction(&client, tx_info).await {
            Ok(new_tx) => new_tx,
            Err(err) => {
                let response: Status = Status {
                    service: constants::service_name(),
                    message: err.to_string(),
                    version: constants::full_version(),
                };
                return Ok(HttpResponse::InternalServerError().json(response));
            }
        };

        Ok(HttpResponse::Ok().json(new_tx))
    }
}

pub mod client {
    #![allow(dead_code)]
    // client wrappers using Atix Web Client (awc)
    use super::models::{Account, Health, Status, Transaction};
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

    pub async fn create_account(
        server_addr: String,
        account_params: Account,
    ) -> Result<Account, Error> {
        // server_addr string must be of the form <ip>:<port>
        let url = format!("http://{}/create-account", server_addr);

        // sanitize before sending
        let acc_pars = Account {
            id: Default::default(),
            username: account_params.username,
            email: account_params.email,
            balance: Default::default(),
            created_at: Default::default(),
        };

        let body_json = serde_json::to_string(&acc_pars).map_err(|e| {
            actix_web::error::ErrorInternalServerError(format!(
                "Failed to serialize request body: {}",
                e
            ))
        })?;

        let client = Client::default();

        let mut response = client
            .post(&url)
            .send_json(&body_json) // Send JSON body
            .await
            .map_err(|e| {
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

    pub async fn create_transaction(
        server_addr: String,
        tx_params: Transaction,
    ) -> Result<Transaction, Error> {
        // server_addr string must be of the form <ip>:<port>
        let url = format!("http://{}/create-tx", server_addr);

        // sanitize before sending
        let t_pars = Transaction {
            id: Default::default(),
            from_account: tx_params.from_account,
            to_account: tx_params.to_account,
            amount: tx_params.amount,
            created_at: Default::default(),
        };

        let body_json = serde_json::to_string(&t_pars).map_err(|e| {
            actix_web::error::ErrorInternalServerError(format!(
                "Failed to serialize request body: {}",
                e
            ))
        })?;

        let client = Client::default();

        let mut response = client
            .post(&url)
            .send_json(&body_json) // Send JSON body
            .await
            .map_err(|e| {
                actix_web::error::ErrorInternalServerError(format!("Server response error: {}", e))
            })?;

        // Check if the request was successful
        if response.status().is_success() {
            let tx: Transaction = response.json().await.map_err(|e| {
                actix_web::error::ErrorInternalServerError(format!(
                    "Error converting response body: {}",
                    e
                ))
            })?;
            Ok(tx)
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
}
