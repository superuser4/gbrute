use std::error::Error;
use crate::bust::BusterEngine;
use crate::bust::Buster;
use async_trait::async_trait;

#[derive(Clone)]
pub struct DirBuster {
    engine: BusterEngine,
}

impl DirBuster {
    pub fn new(url: String, wordlist: String, threads: u64, timeout: u64, user_agent: String) -> Self {
        let engine: BusterEngine = BusterEngine::new(url, wordlist, threads, timeout, user_agent).expect("Buster Engine creation failed");
        Self { engine }
    }   
}

#[async_trait]
impl Buster for DirBuster {
    async fn exec(&self, word: String) -> Result<(), Box<dyn Error + Send>> {
        println!("Running task for word {word}");
        Ok(())
    }

    async fn run(&mut self) -> Result<(), Box<dyn Error + Send>> {
        let this = self.clone();
        let _ = self.engine.run(&this).await;
        Ok(())
    }
}

