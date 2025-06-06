use async_trait::async_trait;
use std::sync::Arc;
use std::future::Future;
use std::pin::Pin;
use std::error::Error;
use tokio::fs;
use futures::stream::StreamExt;
use clap::ValueEnum; 

#[derive(ValueEnum, Clone, Debug)]
pub enum DirBusterMode {
    Bust,
    Domain,
    Fuzz,
}

#[async_trait]
pub trait Buster {
    fn url(&self) -> Arc<String>;
    fn wordlist(&self) -> &str;
    fn http_client(&self) -> Arc<reqwest::Client>;
    fn threads(&self) -> usize;
    fn user_agent(&self) -> String;
    fn timeout(&self) -> usize;

    fn bust_fn(
        &self,
    ) -> Box<
        dyn Fn(Arc<String>, Arc<reqwest::Client>, String) -> Pin<Box<dyn Future<Output = ()> + Send>>
            + Send
            + Sync,
            >;

    async fn run(&self) -> Result<(), Box<dyn Error>> {
        let content = fs::read_to_string(self.wordlist()).await?;
        let words = content.lines().map(str::to_owned).collect::<Vec<_>>();

        let bust_fn = self.bust_fn(); 
        let bust_fn = Arc::new(bust_fn); 

        futures::stream::iter(words)
            .map(|word| {
                let url = Arc::clone(&self.url());
                let client = Arc::clone(&self.http_client());
                let bust_fn = Arc::clone(&bust_fn);

                async move {
                    bust_fn(url, client, word).await;
                }
            })
            .buffer_unordered(self.threads())
            .for_each(|_| async {})
            .await;

        Ok(())
    }
}

