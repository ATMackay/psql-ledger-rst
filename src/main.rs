mod config {
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
            log_level: "debug".to_string(),
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

        //pub fn from_env() -> Result<Self, envy::Error> {
        //    envy::from_env::<Config>()
        //}
    }
}

mod models {
    //use chrono::{DateTime, Utc};
    use serde::{Deserialize, Serialize}; // Serializer
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

    #[derive(Deserialize, Serialize)]
    pub struct Status {
        pub service: String,
        pub version: String,
        pub message: String,
    }

    #[derive(Deserialize, Serialize)]
    pub struct Health {
        pub service: String,
        pub version: String,
        pub message: String,
        pub failures: Vec<String>,
    }

    // Custom serialization function for DateTime<Utc> - TODO
    //fn serialize_datetime<S>(date: &DateTime<Utc>, serializer: S) -> Result<S::Ok, S::Error>
    //where
    //    S: Serializer,
    //{
    //    let s = date.to_rfc3339();
    //    serializer.serialize_str(&s)
    //}
}

mod constants {

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

    use crate::{
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

mod handlers {
    use crate::{
        constants, db,
        errors::MyError,
        models::{Account, Health, Status, Transaction},
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
}

use crate::config::{default_config, Config};
use actix_web::{web, App, HttpServer};
use clap;
use env_logger::Env;
use handlers::{
    create_account, create_transaction, get_account_by_id, get_accounts, get_transactions, health,
    status,
};
use tokio_postgres::NoTls;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Read config file from flag

    let matches = clap::App::new("MyApp")
        .arg(
            clap::Arg::with_name("config")
                .long("config")
                .value_name("FILE")
                .help("Sets the config file to use")
                .takes_value(true),
        )
        .get_matches();

    // Check if the user provided a file name via the --config flag
    let config_file = matches.value_of("config").unwrap_or("config.json");

    let config = match Config::from_file(config_file) {
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

    let log_level = config.log_level;

    env_logger::Builder::from_env(Env::default().default_filter_or(&log_level))
        .format_timestamp_millis()
        .init();

    log::info!("welcome to {}", constants::service_name());

    log::info!("version: {}", constants::full_version());

    log::info!("compilation date {}", constants::build_date());

    log::info!("log level: {}", &log_level);

    log::info!("using config file: {}", config_file);

    log::info!("Server Address: {}", config.server_addr);

    log::debug!("PostgreSQL Configuration: {:?}", config.pg);

    let pool = config.pg.create_pool(None, NoTls).unwrap();

    let server = HttpServer::new(move || {
        let app = App::new()
            .app_data(web::Data::new(pool.clone()))
            .service(web::resource("/status").route(web::get().to(status)))
            .service(web::resource("/health").route(web::get().to(health)))
            .service(web::resource("/create_account").route(web::post().to(create_account)))
            .service(web::resource("/accounts").route(web::get().to(get_accounts)))
            .service(web::resource("/account_by_id").route(web::get().to(get_account_by_id)))
            .service(web::resource("/transactions").route(web::get().to(get_transactions)))
            .service(
                web::resource("/create_transaction").route(web::post().to(create_transaction)),
            );

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
