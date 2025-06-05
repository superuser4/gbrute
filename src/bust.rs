use futures::stream::StreamExt;
use reqwest::StatusCode;
use std::{error::Error, sync::Arc};
use tokio::fs;
use clap::ValueEnum;

#[derive(ValueEnum, Clone, Debug)]
pub enum DirBusterMode {
    Bust,
    Domain,
    Fuzz,
}

pub struct DirBuster {
    url: Arc<String>,
    wordlist: String,
    threads: u64,
    timeout: u64,
    user_agent: String,
    http_client: Option<Arc<reqwest::Client>>,
    mode: DirBusterMode, 
}

 async fn bust_dir(url: Arc<String>, cli: Arc<reqwest::Client>,dir: &str){
     let uri: String = format!("{}{}", url, dir);
     let response_builder: reqwest::RequestBuilder = cli.head(&uri);
     let response = match response_builder.send().await {
         Ok(resp) => resp,
         Err(_) => return,
     };
 
 
     let code: reqwest::StatusCode = response.status();
     if !code.is_client_error() {
         println!("Busted: {uri}:{code}");
     }
 }
 async fn bust_domain(url: Arc<String>, cli: Arc<reqwest::Client>, subd: &str) {
     // https://example.com -> ["https", "example.com"]
     let spl: Vec<&str> = url.split("://").collect();
     
     // ["https", "example.com"] -> https://internal.example.com
     let new_uri: String = spl[0].to_string() + "://" + subd + "." + spl[1];

     let response_builder: reqwest::RequestBuilder = cli.head(&new_uri);
     let response = match response_builder.send().await {
         Ok(r) => r,
         Err(_) => return,
     };
     let code: StatusCode = response.status();
     if !code.is_client_error() { 
         println!("Busted: {new_uri} -> {code}");
     }

 }
 async fn bust_fuzz(url: Arc<String>, cli: Arc<reqwest::Client>, user: String, pass: String) {}
    

 impl DirBuster {
    pub fn new(url: String, wordlist: String, threads: u64, timeout: u64, user_agent: String, mode: DirBusterMode) -> Self {
        let url = Arc::new(url);
        Self { url, wordlist, threads, timeout, user_agent, http_client: None , mode}
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
    
    pub async fn bust(&mut self) -> Result<(), Box<dyn Error>> {
        self.create_client().expect("Error could not create HTTP Client");
        if matches!(self.mode,DirBusterMode::Bust) && !self.url.ends_with('/')  { 
            let mut tmp = (*Arc::clone(&self.url)).clone();
            tmp.push('/');
            self.url = Arc::new(tmp);
        }
    
        let cont = fs::read_to_string(&self.wordlist).await?;
        let dirs = cont.lines().map(String::from);

        futures::stream::iter(dirs)
            .map( |dir| {
               let cli_clone = Arc::clone(match &self.http_client {
                   Some(cli_clone) => cli_clone,
                   None => todo!(),
               });
               let url_clone = Arc::clone(&self.url);
               let mode = &self.mode;
               async move {
                   match mode {
                       DirBusterMode::Bust => bust_dir(url_clone, cli_clone, dir.as_str()).await,
                       DirBusterMode::Domain => bust_domain(url_clone, cli_clone, dir.as_str()).await,
                       DirBusterMode::Fuzz => todo!(), 
                   }
               }
        })
        .buffer_unordered(self.threads.try_into().unwrap())
        .for_each(|_| async {})
        .await;
                
        Ok(())
    }
}
