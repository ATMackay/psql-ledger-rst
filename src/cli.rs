use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(
    name = env!("CARGO_PKG_NAME"),  // Get the package name from Cargo.toml
    version = env!("CARGO_PKG_VERSION"),  // Get the version from Cargo.toml
    about = "A simple transaction ledger written in Rust. Integrates with PostgreSQL DB",
    long_about = concat!(
        "Service Name: ", env!("CARGO_PKG_NAME"), "\n",
        "Version: ", env!("CARGO_PKG_VERSION"), "\n",
        "Build Date: ", env!("BUILD_DATE"), "\n"
    )
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Start the Actix Web server, connect to Postgres
    Run(RunArgs),

    /// Print full version details
    Version,
}

#[derive(Parser)]
pub struct RunArgs {
    #[arg(
        long,
        default_value = "config.json",
        help = "Path to the configuration file"
    )]
    pub config: String,
}
