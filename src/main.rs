mod lib;
use actix_web::{web, App, HttpServer};
use clap;
use env_logger::Env;
use lib::config::{default_config, Config};
use lib::constants::{build_date, full_version, service_name};
use lib::handlers::{
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

    log::info!("welcome to {}", service_name());

    log::info!("version: {}", full_version());

    log::info!("compilation date {}", build_date());

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
