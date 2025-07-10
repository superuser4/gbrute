use std::error::Error;
use crate::bust::BusterEngine;
use crate::bust::Buster;
use async_trait::async_trait;
use indicatif::ProgressBar;

#[derive(Clone)]
pub struct DirBuster {
    engine: BusterEngine,
    status_code: Vec<u16>,
    recursive: bool,
}

impl DirBuster {
    pub fn new(url: String, wordlist: String, threads: u64, timeout: u64, user_agent: String, status_code: Vec<u16>, recursive: bool) -> Result<Self, reqwest::Error> {
        let engine: BusterEngine = BusterEngine::new(url, wordlist, threads, timeout, user_agent)?; 
        Ok(Self { engine , status_code, recursive})
    }   
}

#[async_trait]
impl Buster for DirBuster {
    async fn exec(&self, word: String, bar: ProgressBar) {
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
            let code = resp.status().as_u16();
            if !self.status_code.contains(&code) {
                let msg = format!("Busted: /{word} -> {code}");
                bar.println(msg);

                if self.recursive {
                    todo!();
                }
            }
        }
        bar.inc(1);
    }

    async fn run(&mut self) -> Result<(), Box<dyn Error + Send>> {
        let this = self.clone();
        match self.engine.run(&this).await {
            Ok(_) => return Ok(()),
            Err(e) => return Err(e),
        }
    }
}

