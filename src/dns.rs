use std::error::Error;
use async_trait::async_trait;
use crate::bust::{Buster, BusterEngine};

#[derive(Clone)]
pub struct DnsBuster { 
    engine: BusterEngine,
 }

 impl DnsBuster {
  pub fn new(url: String, wordlist: String, threads: u64, timeout: u64, user_agent: String) -> Self {
        let engine: BusterEngine = BusterEngine::new(url, wordlist, threads, timeout, user_agent).expect("Buster Engine creation failed");
        Self { engine }
    }   
 }

#[async_trait]
 impl Buster for DnsBuster {
    async fn run(&mut self) -> Result<(), Box<dyn Error+ Send>> {
        let this = self.clone();
        match self.engine.run(&this).await {
            Ok(_) => return Ok(()),
            Err(e) => return Err(e),
        }
    }
    async fn exec(&self, word: String) {
        let split_domain: Vec<&str> = self.engine.url.split("://").collect();
        let new_domain: String = split_domain[0].to_string() + "://" + &word + "." + split_domain[1];
        

        if let Ok(resp) = self
            .engine
            .http_client
            .head(&new_domain)
            .send()
            .await
        {

            if !resp.status().is_client_error() {
                println!("Busted: /{}", &new_domain);
            }
        }
 
    }
 }
