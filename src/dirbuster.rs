use std::{pin::Pin, sync::Arc};

use crate::bust;

pub struct DirBuster {
    url: Arc<String>,
    wordlist: String,
    threads: u64,
    timeout: u64,
    user_agent: String,
    http_client: Option<Arc<reqwest::Client>>,
}


impl DirBuster {
    pub fn new(url: String, wordlist: String, threads: u64, timeout: u64, user_agent: String) -> Self {
        let url = Arc::new(url);
        Self { url, wordlist, threads, timeout, user_agent, http_client: None }
    }

    
    fn create_client(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let headers: reqwest::header::HeaderMap = Default::default();
        let client =
            reqwest::ClientBuilder::new()
            .user_agent(&self.user_agent)
            .default_headers(headers)
            .timeout(std::time::Duration::from_millis(self.timeout))
            .redirect(reqwest::redirect::Policy::none())
            .pool_max_idle_per_host(self.threads as usize)
            .build()?;
        self.http_client = Some(Arc::new(client));
        Ok(())
    } 
}

 impl bust::Buster for DirBuster {
    fn url(&self) -> Arc<String> {
        Arc::clone(&self.url)
    }
    fn wordlist(&self) -> String {
        self.wordlist.clone()
    }
    fn http_client(&self) -> Arc<reqwest::Client> {
        Arc::clone(match &self.http_client {
            Some(e) => e,
            _ => todo!(),
        })
    }
    fn threads(&self) -> u64 {
        self.threads
    }
    fn user_agent(&self) -> String {
        self.user_agent.clone()
    }
    fn timeout(&self) -> u64{
        self.timeout
    }
    
    fn bust_fn(
        &mut self,
    ) -> Box<
        dyn Fn(Arc<String>, Arc<reqwest::Client>, String) -> Pin<Box<dyn Future<Output = ()> + Send>>
            + Send
            + Sync,
    > {
        self.create_client().expect("Failed to create http client");
        Box::new(|url, client, word| {
            Box::pin(async move {
                let uri: String = format!("{}{}", url, word);
                let response_builder: reqwest::RequestBuilder = client.head(&uri);
                let response = response_builder.send().await;
                let ok = match response {
                    Ok(resp)=> resp,
                    _ => return,
                };
 
                let code: reqwest::StatusCode = ok.status();
                if !code.is_client_error() {
                    println!("Busted: {uri}:{code}");
                }
            })
        })
    }

}
