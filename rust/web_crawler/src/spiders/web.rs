use crate::error::Error;
use async_trait::async_trait;

use reqwest::Client;
use scraper::{Html, Selector};
use std::time::Instant;
use tokio::time::Duration;
use url::Url;

pub struct WebSpider {
    http_client: reqwest::Client,
    start_url: String,
}

impl WebSpider {
    pub fn new(start_url: String, worker: usize) -> Self {
        let http_timeout = Duration::from_secs(4);

        let http_client = Client::builder()
            .timeout(http_timeout)
            .user_agent(
                "Mozilla/5.0 (Windows NT 6.1; Win64; x64; rv:47.0) Gecko/20100101 Firefox/47.0",
            )
            .pool_idle_timeout(http_timeout)
            .pool_max_idle_per_host(worker)
            .build()
            .expect("WebSpider: Building HTTP client");

        WebSpider {
            http_client,
            start_url,
        }
    }
}

#[async_trait]
impl super::Spider for WebSpider {
    type Item = String;

    fn name(&self) -> String {
        String::from("WebSpider")
    }

    fn start_url(&self) -> String {
        self.start_url.to_string()
    }

    async fn scrape(&self, url: String) -> Result<Vec<String>, Error> {
        println!("Scraping url: {}", &url);
        let raw_url = Url::parse(&url)?;
        let host = raw_url.scheme().to_owned() + "://" + raw_url.host_str().unwrap();

        let start = Instant::now();
        let body_content = self.http_client.get(&url).send().await?.text().await?;

        let seconds = start.elapsed().as_secs_f64();
        if seconds > 3.0 {
            println!(
                "Parsing res body: {} in \x1B[32m{:.2}s\x1B[0m",
                &url, seconds
            );
        }
        let parser = Html::parse_document(body_content.as_str());
        let selector = Selector::parse("a[href]").unwrap();

        let links: Vec<String> = parser
            .select(&selector)
            .filter_map(|element| element.value().attr("href"))
            .filter_map(|href| {
                let parsed_link = raw_url.join(href);
                match parsed_link {
                    Ok(link) => {
                        let mut absolute_link = link.clone();
                        absolute_link.set_fragment(None);
                        absolute_link.set_query(None);
                        if absolute_link.to_string().starts_with(&host) {
                            Some(absolute_link.to_string())
                        } else {
                            None
                        }
                    }
                    Err(_) => None,
                }
            })
            .collect();

        Ok(links)
    }
}
