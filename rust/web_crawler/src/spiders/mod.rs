use crate::error::Error;
use async_trait::async_trait;

pub mod web;

#[async_trait]
pub trait Spider: Send + Sync {
    type Item;

    fn name(&self) -> String;
    fn start_url(&self) -> String;
    async fn scrape(&self, url: String) -> Result<Vec<Self::Item>, Error>;
}
