use std::error::Error;
use crate::bust::BusterEngine;
use crate::bust::Buster;
use async_trait::async_trait;

#[derive(Clone)]
pub struct DirBuster {
    engine: BusterEngine,
}

impl DirBuster {
    pub fn new(url: String, wordlist: String, threads: u64, timeout: u64, user_agent: String) -> Result<Self, reqwest::Error> {
        let engine: BusterEngine = BusterEngine::new(url, wordlist, threads, timeout, user_agent)?; 
        Ok(Self { engine })
    }   
}

#[async_trait]
impl Buster for DirBuster {
    async fn exec(&self, word: String) -> Result<(), Box<dyn Error + Send>> {
        let mut new_uri = self.engine.url.to_string();
        if !new_uri.ends_with('/') {
            new_uri.push('/');
        }
        new_uri.push_str(&word);

        if let Ok(resp) = self
            .engine
            .http_client
            .head(new_uri)
            .send()
            .await
        {

            if !resp.status().is_client_error() {
                println!("Busted: /{word}");
            }
        }

        Ok(())
    }

    async fn run(&mut self) -> Result<(), Box<dyn Error + Send>> {
        let this = self.clone();
        match self.engine.run(&this).await {
            Ok(_) => return Ok(()),
            Err(e) => return Err(e),
        }
    }
}

