mod cli;
mod config;
mod db;
mod errors;
mod handlers;
mod model;
mod server;

use clap::Parser;
use cli::{Cli, Commands};
use server::run_server;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Run(args) => run_server(&args.config).await,
        Commands::Version => {
            println!("Version: {}", env!("CARGO_PKG_VERSION"));
            println!("Compilation Date: {}", env!("BUILD_DATE"));
            println!("Git Version: {}", env!("GIT_VERSION_TAG"));
            println!("Commit Hash: {}", env!("GIT_COMMIT"));
            println!("Commit Date: {}", env!("GIT_COMMIT_DATE"));
            Ok(())
        }
    }
}
