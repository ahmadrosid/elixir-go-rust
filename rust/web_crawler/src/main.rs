use clap::Parser;
use std::sync::Arc;
use std::time::Instant;

mod crawler;
mod error;
mod measure;
mod spiders;

use crate::crawler::Crawler;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Set the URL to crawl
    #[arg(short, long, value_name = "URL")]
    url: String,

    /// Sets the number of workers
    #[arg(short, long, value_name = "NUM", default_value = "5")]
    worker: usize,
}
#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    let start = Instant::now();
    let result = async move {
        let url = cli.url;
        let worker_count = cli.worker;

        let crawler = Crawler::new(worker_count);
        let spider = Arc::new(spiders::web::WebSpider::new(url));
        crawler.run(spider).await
    }
    .await;

    let elapsed = start.elapsed();
    println!("\x1B[34mFinished in: {} seconds", elapsed.as_secs_f64());
    measure::print_mem_usage();

    print!(
        "Total links scraped: {}, concurrency: {}\n",
        result, cli.worker
    );
}
