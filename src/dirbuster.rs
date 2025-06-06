use std::{pin::Pin, sync::Arc};

use crate::bust::{self, BusterEngine};

pub struct DirBuster {
    engine: BusterEngine,
}

impl DirBuster {
    pub fn new(url: String, wordlist: String, threads: u64, timeout: u64, user_agent: String) -> Self {
        let engine: BusterEngine = BusterEngine::new(url, wordlist, threads, timeout, user_agent);
    }

   }

