use std::{error::Error, sync::Arc};
use futures::{StreamExt, TryStreamExt};
use async_trait::async_trait;
use indicatif::{ProgressBar, ProgressStyle};

#[async_trait]
pub trait Buster: Send + Sync {
    async fn run(&mut self) -> Result<(), Box<dyn Error + Send>>;
    async fn exec(&self, word: String, bar: ProgressBar); 
}


#[derive(Clone)]
pub struct BusterEngine {
    pub url: Arc<String>,
    wordlist: String,
    threads: u64,
    pub http_client: Arc<reqwest::Client>,
}


impl BusterEngine {
    pub fn new(url: String, wordlist: String, threads: u64, timeout: u64, user_agent: String) -> Result<Self, reqwest::Error> {
        let url = Arc::new(url);
        let http_client = Arc::new(BusterEngine::create_client(&user_agent, timeout, threads)?); 
        Ok(Self { url, wordlist, threads, http_client })
    }
    fn create_client(user_agent: &String, timeout_s: u64, threads: u64) -> Result<reqwest::Client, reqwest::Error> {
        let headers: reqwest::header::HeaderMap = Default::default();
        let client = 
            reqwest::ClientBuilder::new()
            .user_agent(user_agent)
            .default_headers(headers)
            .timeout(std::time::Duration::from_millis(timeout_s))
            .redirect(reqwest::redirect::Policy::none())
            .pool_max_idle_per_host(threads as usize)
            .build()?;
        Ok(client)
    }
    pub async fn run<B: Buster + ?Sized>(&mut self, buster: &B) -> Result<(), Box<dyn Error + Send>> {
        let content = tokio::fs::read_to_string(&self.wordlist)
            .await
            .map_err(|e| Box::new(e) as Box<dyn Error + Send>)?; 
        let words = content.lines().map(str::to_owned).collect::<Vec<_>>();
        let bar = ProgressBar::new(words.len() as u64);
        bar.set_style(ProgressStyle::with_template("[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {msg}")
            .unwrap()
            .progress_chars("##-"));
        
        futures::stream::iter(words)
            .map(|word| {
                let bar = bar.clone();
                async move {
                    buster.exec(word, bar).await;
                    Ok(())
                }
            })
            .buffer_unordered(self.threads as usize)
            .try_collect::<()>()
            .await?;
        
        Ok(())
    }
}

