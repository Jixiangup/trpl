use clap::Parser;
use cli::{Cli, Commands};
use trpl_core::command::Command;

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    match cli.command {
        Commands::Crawler(cmd) => {
            cmd.run().await;
        }
    }
}

