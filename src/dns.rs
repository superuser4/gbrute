use std::error::Error;
use async_trait::async_trait;
use hickory_resolver::{config::ResolverConfig, name_server::TokioConnectionProvider, Resolver};
use indicatif::ProgressBar;
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
    async fn exec(&self, word: String, bar: ProgressBar) {
        let split_domain: Vec<&str> = self.engine.url.split("://").collect();
        let new_domain: String = split_domain[0].to_string() + "://" + &word + "." + split_domain[1];

        let resolver = Resolver::builder_with_config(
            ResolverConfig::default(), 
            TokioConnectionProvider::default()
        ).build();
        match resolver.lookup_ip(&new_domain).await {
            Ok(lookup) => {
                for ip in lookup.iter() {
                    let msg: String = format!("Resolved {} -> {}", &new_domain, ip);
                    bar.println(msg);
                }
            }
            Err(_e) => {}
        }
        bar.inc(1);
    }
 }
