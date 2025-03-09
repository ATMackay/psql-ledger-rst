use actix_contrib_logger::middleware::Logger;
use actix_web::{http::StatusCode, web, App, HttpServer};
use env_logger::Env;
use log::Level;
use tokio_postgres::NoTls;
use crate::config::{default_config, Config};
use crate::handlers::{
    create_account, create_transaction, get_account_by_id, get_accounts, get_transaction_by_id,
    get_transactions, health, status,
};

pub async fn run_server(config_file: &str) -> std::io::Result<()> {
    // Load configuration
    let config = match Config::from_file(config_file) {
        Ok(config) => {
            log::info!("Loaded configuration from file.");
            config
        }
        Err(err) => {
            log::warn!("Failed to load configuration from file: {}", err);
            log::info!("Using default configuration.");
            default_config()
        }
    };

    // Initialize logger
    env_logger::Builder::from_env(Env::default().default_filter_or(&config.log_level))
        .format_timestamp_millis()
        .init();

    log::info!("Welcome to {}", env!("SERVICE_NAME"));
    log::info!("Version: {}", env!("VERSION"));
    log::info!("Compilation date: {}", env!("BUILD_DATE"));
    log::info!("Log level: {}", &config.log_level);
    log::info!("Using config file: {}", config_file);
    log::debug!("PostgreSQL Configuration: {:?}", config.pg);

    // Create PostgreSQL connection pool
    let pool = config.pg.create_pool(None, NoTls).unwrap();

    // Start Actix Web server
    let server = HttpServer::new(move || {
        let logger = Logger::default().custom_level(|status| {
            if status.is_server_error() {
                Level::Error
            } else if status == StatusCode::NOT_FOUND || status == StatusCode::BAD_REQUEST {
                Level::Warn
            } else {
                Level::Info
            }
        });

        App::new()
            .app_data(web::Data::new(pool.clone()))
            .wrap(logger)
            .service(web::resource("/status").route(web::get().to(status)))
            .service(web::resource("/health").route(web::get().to(health)))
            .service(web::resource("/accounts").route(web::get().to(get_accounts)))
            .service(web::resource("/transactions").route(web::get().to(get_transactions)))
            .service(web::resource("/account-by-id").route(web::post().to(get_account_by_id)))
            .service(
                web::resource("/transaction-by-id").route(web::post().to(get_transaction_by_id)),
            )
            .service(web::resource("/create-account").route(web::put().to(create_account)))
            .service(web::resource("/create-tx").route(web::put().to(create_transaction)))
    })
    .bind(config.server_addr.clone())?
    .run();

    log::info!("PSQL Server running at http://{}", config.server_addr);

    server.await
}
