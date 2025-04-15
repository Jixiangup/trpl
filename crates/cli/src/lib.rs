use clap::{Parser, Subcommand};
use crawler::CrawlerCommand;

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