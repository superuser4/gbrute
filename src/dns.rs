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
        Ok(())
    }
    async fn exec(&self, word: String) -> Result<(), Box<dyn Error + Send>> {
        Ok(())
    }
 }
