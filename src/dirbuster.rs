use std::error::Error;
use crate::bust::BusterEngine;

pub struct DirBuster {
    engine: BusterEngine,
}

impl DirBuster {
    pub fn new(url: String, wordlist: String, threads: u64, timeout: u64, user_agent: String) -> Self {
        let engine: BusterEngine = BusterEngine::new(url, wordlist, threads, timeout, user_agent);
        Self { engine }
    }
    
    pub async fn task(word: String) -> Result<(), Box<dyn Error + Send>> {
        println!("Running task for word {word}");
        Ok(())
    }

    pub async fn run(&mut self) {
        self.engine.run(DirBuster::task).await.unwrap();
    }
}

