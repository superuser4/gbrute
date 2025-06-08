use std::error::Error;
use async_trait::async_trait;
use crate::bust::{Buster, BusterEngine};

#[derive(Clone)]
pub struct DnsBuster { 
    engine: BusterEngine,
 }

 impl DnsBuster {
     fn new() {}
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
