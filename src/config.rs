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
        server_addr: "0.0.0.0:8080".to_string(),
        pg: PgConfig::default(),
    };

    let default_host = "0.0.0.0".to_string();
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

#[allow(dead_code)]
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
