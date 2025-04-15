use async_trait::async_trait;
use clap::Args;
use scraper::{Html, Selector};
use trpl_core::command;

#[derive(Args)]
pub struct CrawlerCommand {

    #[arg(short = 'u', long = "url", help = "The URL to crawl", required = true)]
    pub url: String,

    #[arg(short = 's', long = "selector", help = "The css selector to crawl", required = true)]
    pub selector: String

}

#[async_trait]
impl command::Command for CrawlerCommand {
    async fn run(&self) {
        let url = &self.url;
        let selector = &self.selector;
        let title = crawler(url, selector).await;
        match title {
            Some(values) => {
                println!("搜索页面 {url} 的选择器 {selector} 完成。");
                for value in values {
                    println!("{}", value);
                }
            },
            None => println!("Failed to fetch the title"),
        }
    }
}

pub async fn crawler(url: &String, input_selector: &String) -> Option<Vec<String>> {
    let response = reqwest::get(url).await.ok()?.text().await.ok()?;
    let selector = Selector::parse(input_selector).ok();
    match selector {
        None => {
            eprintln!("Unsupported selector.");
            None
        }
        Some(selector) => {
            let document = Html::parse_document(&response);
            let select = document.select(&selector);
            Some(select.map(move |e| {
                return e.text().map(move |text| text).collect()
            }).collect())
        }
    }
}