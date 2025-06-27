use crate::bust::{Buster, BusterEngine};
use async_trait::async_trait;
use indicatif::ProgressBar;
use std::{error::Error, process::exit};


#[derive(Clone)]
pub struct FuzzBuster {
    engine: BusterEngine,
}

impl FuzzBuster {
  pub fn new(url: String, wordlist: String, threads: u64, timeout: u64, user_agent: String) -> Self {
        let engine: BusterEngine = BusterEngine::new(url, wordlist, threads, timeout, user_agent).expect("Buster Engine creation failed");
        Self { engine }
    }   
}

#[async_trait]
 impl Buster for FuzzBuster {
    async fn run(&mut self) -> Result<(), Box<dyn Error+ Send>> {
        let this = self.clone();
        match self.engine.run(&this).await {
            Ok(_) => return Ok(()),
            Err(e) => return Err(e),
        }
    }
    async fn exec(&self, word: String, bar: ProgressBar) {
        if !self.engine.url.contains("FUZZ") {
            println!("Error: <FUZZ> not found in the url");
            exit(1);
        }
        let payload = self.engine.url.replace("FUZZ", &word);

        if let Ok(resp) = self
            .engine
            .http_client
            .head(&payload)
            .send()
            .await
        {
            let code = resp.status();
            if !code.is_client_error() {
                println!("Busted: /{} -> {}", &payload, code);
            }
        }
        bar.inc(1); 
    }
 }
