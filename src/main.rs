mod config {
    use serde::Deserialize;
    use std::fs::File;
    use std::io::{BufReader, Error};
    use deadpool_postgres::Config as PgConfig;
    use deadpool_postgres::SslMode ;

    #[derive(Debug, Default, Deserialize)]
    pub struct Config {
        pub server_addr: String,
        pub pg: PgConfig,
    }

    pub fn default_config() -> Config {

        let mut cfg =  Config {
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
        pub fn from_file(filename: &str) -> Result<Self, Error> {
            let file = File::open(filename)?;
            let reader = BufReader::new(file);
    
            // Handle file reading and parsing errors here
            // For example:
            let config: Config = serde_json::from_reader(reader)?;
    
            Ok(config)
        }
    
        pub fn from_env() -> Result<Self, envy::Error> {
            envy::from_env::<Config>()
        }
    }
}

mod models {
    use serde::{Deserialize, Serialize, Serializer}; 
    use chrono::{DateTime, Utc};
    use tokio_pg_mapper_derive::PostgresMapper;

    #[derive(Deserialize, PostgresMapper, Serialize)]
    #[pg_mapper(table = "accounts")] 
    pub struct Account {
        pub id: i64,
        pub username: String,
        pub email: String,
        pub balance: i64,
        //#[serde(serialize_with = "serialize_datetime")]
        //pub created_at: DateTime<Utc>,
    }
    #[derive(Deserialize, PostgresMapper, Serialize)]
    #[pg_mapper(table = "transactions")] 
    pub struct Transaction {
        pub id: i64,
        pub from_account: i64,
        pub to_account: i64,
        pub amount: i64,
        //#[serde(serialize_with = "serialize_datetime")]
        //pub created_at: DateTime<Utc>,
    }

    // Custom serialization function for DateTime<Utc> - TODO
    fn serialize_datetime<S>(date: &DateTime<Utc>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = date.to_rfc3339();
        serializer.serialize_str(&s)
    }
}

mod errors {
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

mod db {
    use deadpool_postgres::Client;
    use tokio_pg_mapper::FromTokioPostgresRow;

    use crate::{errors::MyError, models::{Account, Transaction}};

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

    pub async fn get_account_by_id(client: &Client, account_id: i64) -> Result<Account, MyError>  {
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

    pub async fn create_account(client: &Client, account_info: Account) -> Result<Account, MyError> {
        let _stmt = "INSERT INTO accounts (
            username, balance, email
        ) VALUES (
            $1, $2, $3
        )
        RETURNING *";
        let _stmt = _stmt.replace("$table_fields", &Account::sql_table_fields());
        let stmt = client.prepare(&_stmt).await.unwrap();

        client
            .query(
                &stmt,
                &[
                    &account_info.email,
                    &account_info.username,
                ],
            )
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

    pub async fn create_transaction(client: &Client, transaction_info: Transaction) -> Result<Transaction, MyError> {
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

mod handlers {
    use actix_web::{web, Error, HttpResponse};
    use deadpool_postgres::{Client, Pool};

    use crate::{db, errors::MyError, models::{Account, Transaction}};

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
}

use crate::config::{Config, default_config};
use actix_web::{web, App, HttpServer};
use handlers::{get_accounts, get_account_by_id, create_account, create_transaction, get_transactions};
use tokio_postgres::NoTls;
use env_logger::Env;
use std::env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    
    env_logger::Builder::from_env(Env::default().default_filter_or("info"))
        .format_timestamp_millis()
        .init();

    log::info!("welcome to psql-ledger-rst");

    if let Some(semver) = env::var("VERSION").ok() {
        log::info!("version: {}", semver);
    }
    if let Some(git_hash) = env::var("GIT_HASH").ok() {
        log::info!("git commit: {}", git_hash);
    }
    if let Some(build_date) = env::var("BUILD_DATE").ok() {
        log::info!("compilation date {}", build_date);
    }


    let config = match Config::from_file("config.json") {
        Ok(config) => {
            log::info!("Loaded configuration from file.");
            // Use the loaded config
            config
        }
        Err(err) => {
            log::warn!("Failed to load configuration from file: {}", err);
            log::info!("Using default configuration.");
            default_config() // Set default config
        }
    };

    
    log::info!("Server Address: {}", config.server_addr);
    log::debug!("PostgreSQL Configuration: {:?}", config.pg);

    let pool = config.pg.create_pool(None, NoTls).unwrap();

    let server = HttpServer::new(move || {
        let app = App::new().app_data(web::Data::new(pool.clone()))
        .service(web::resource("/create_account")
            .route(web::post().to(create_account)))
        .service(web::resource("/accounts")
            .route(web::get().to(get_accounts)))
        .service(web::resource("/account_by_id")
            .route(web::get().to(get_account_by_id)))
        .service(web::resource("/transactions")
            .route(web::get().to(get_transactions)))
        .service(web::resource("/create_transaction")
            .route(web::post().to(create_transaction)));

        // Log all available endpoints - TODO
        //for resource in app.resources() {
         //   log::info!("registered endpoint: {}", resource.path());
        //}

        app
    })
    .bind(config.server_addr.clone())?
    .run();
    log::info!("PSQL Server running at http://{}", config.server_addr);

    server.await
}