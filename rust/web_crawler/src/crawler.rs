use crate::spiders::Spider;
use futures::stream::StreamExt;
use std::{
    collections::HashSet,
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc,
    },
    time::Duration,
};
use tokio::{
    sync::{mpsc, Barrier},
    time::sleep,
};

pub struct Crawler {
    crawling_concurrency: usize,
}

impl Crawler {
    pub fn new(crawling_concurrency: usize) -> Self {
        Crawler {
            crawling_concurrency,
        }
    }

    pub async fn run<T: Send + 'static>(&self, spider: Arc<dyn Spider<Item = String>>) -> usize {
        let mut visited_urls = HashSet::<String>::new();
        let crawling_concurrency = self.crawling_concurrency;
        let crawling_queue_capacity = crawling_concurrency * 400;
        let active_spiders = Arc::new(AtomicUsize::new(0));

        let (urls_to_visit_tx, urls_to_visit_rx) = mpsc::channel(crawling_queue_capacity);
        let (new_urls_tx, mut new_urls_rx) = mpsc::channel(crawling_queue_capacity);
        let barrier = Arc::new(Barrier::new(2));

        let url = spider.start_url();
        visited_urls.insert(url.clone());
        let _ = urls_to_visit_tx.send(url).await;

        self.launch_scrapers(
            crawling_concurrency,
            spider.clone(),
            urls_to_visit_rx,
            new_urls_tx.clone(),
            active_spiders.clone(),
            barrier.clone(),
        );

        loop {
            if let Some((visited_url, new_urls)) = new_urls_rx.try_recv().ok() {
                visited_urls.insert(visited_url);

                for url in new_urls {
                    if !visited_urls.contains(&url) {
                        visited_urls.insert(url.clone());
                        let _ = urls_to_visit_tx.send(url).await;
                    }
                }
            }

            if new_urls_tx.capacity() == crawling_queue_capacity // new_urls channel is empty
            && urls_to_visit_tx.capacity() == crawling_queue_capacity // urls_to_visit channel is empty
            && active_spiders.load(Ordering::SeqCst) == 0
            {
                // no more work, we leave
                break;
            }

            sleep(Duration::from_millis(0)).await;
        }

        println!("Finished crawling {:?}", spider.start_url());

        // we drop the transmitter in order to close the stream
        drop(urls_to_visit_tx);

        // and then we wait for the streams to complete
        barrier.wait().await;

        return visited_urls.len();
    }

    fn launch_scrapers(
        &self,
        concurrency: usize,
        spider: Arc<dyn Spider<Item = String>>,
        urls_to_visit: mpsc::Receiver<String>,
        new_urls_tx: mpsc::Sender<(String, Vec<String>)>,
        active_spiders: Arc<AtomicUsize>,
        barrier: Arc<Barrier>,
    ) {
        tokio::spawn(async move {
            tokio_stream::wrappers::ReceiverStream::new(urls_to_visit)
                .for_each_concurrent(concurrency, |queued_url| {
                    let queued_url = queued_url.clone();
                    async {
                        active_spiders.fetch_add(1, Ordering::SeqCst);
                        let mut urls: Vec<String> = Vec::new();
                        let res = spider
                            .scrape(queued_url.clone())
                            .await
                            .map_err(|err| {
                                println!("{}", err);
                                err
                            })
                            .ok();

                        if let Some(new_urls) = res {
                            urls = new_urls;
                        }

                        let _ = new_urls_tx.send((queued_url, urls)).await;
                        active_spiders.fetch_sub(1, Ordering::SeqCst);
                    }
                })
                .await;

            barrier.wait().await;
        });
    }
}
