use clap::{Parser, Subcommand};
use crawler::CrawlerCommand;
use trpl_core::command::Command;

#[derive(Parser)]
#[command(name = "trpl", version="1.0.0", author="Jixon", about="一个支持多功能的 CLI 工具. 编写了一些自己的 Rust 代码来实现一些功能来加深 Rust 印象.")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands
}

#[derive(Subcommand)]
pub enum Commands {
    Crawler(CrawlerCommand),
}

pub async fn run() {
    let cli = Cli::parse();
    command_factory(cli).await;
}

async fn command_factory(cli: Cli) {
    match cli.command {
        Commands::Crawler(cmd) => {
            cmd.run().await;
        }
    }
}