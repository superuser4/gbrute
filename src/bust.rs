use std::{error::Error, sync::Arc};
use futures::{StreamExt, TryStreamExt};
use async_trait::async_trait;

#[async_trait]
pub trait Buster: Send + Sync {
    async fn run(&mut self) -> Result<(), Box<dyn Error + Send>>;
    async fn exec(word: String) -> Result<(), Box<dyn Error + Send>>;
}


pub struct BusterEngine {
    url: Arc<String>,
    wordlist: String,
    threads: u64,
    timeout: u64,
    user_agent: String,
    http_client: Option<Arc<reqwest::Client>>,
}


impl BusterEngine {

    pub fn new(url: String, wordlist: String, threads: u64, timeout: u64, user_agent: String) -> Self {
        let url = Arc::new(url);
        Self { url, wordlist, threads, timeout, user_agent, http_client: None } 
    }
 
    fn create_client(&mut self) -> Result<(), Box<dyn Error>> {
        let headers: reqwest::header::HeaderMap = Default::default();
        let client =
            reqwest::ClientBuilder::new()
            .user_agent(&self.user_agent)
            .default_headers(headers)
            .timeout(std::time::Duration::from_millis(self.timeout))
            .redirect(reqwest::redirect::Policy::none())
            .pool_max_idle_per_host(self.threads as usize)
            .build()?;
        self.http_client = Some(Arc::new(client));
        Ok(())
    }
    pub async fn run<B: Buster + ?Sized>(&mut self, buster: &B) -> Result<(), Box<dyn Error + Send>> {
        // Create HTTP client if not already created
        if self.http_client.is_none() {
            self.create_client().map_err(|e| Box::new(e) as Box<dyn Error + Send>)?;
        }
        
        let content = tokio::fs::read_to_string(&self.wordlist)
            .await
            .map_err(|e| Box::new(e) as Box<dyn Error + Send>)?;
        
        let words = content.lines().map(str::to_owned).collect::<Vec<_>>();
        
        let client = self.http_client.as_ref().unwrap().clone();
        let url = self.url.clone();
        
        futures::stream::iter(words)
            .map(|word| {
                let buster = buster;
                let client = client.clone();
                let url = url.clone();
                
                async move {
                    buster.exec(word).await?;
                    // Or if you want to use the client/url:
                    // client.get(format!("{}{}", url, word)).send().await?;
                    Ok(())
                }
            })
            .buffer_unordered(self.threads as usize)
            .try_collect::<()>()
            .await?;
        
        Ok(())
    }
}

