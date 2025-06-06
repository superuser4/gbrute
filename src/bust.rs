use std::sync::Arc;
use futures::StreamExt;
use tokio::fs; 



pub struct BusterEngine {
    url: Arc<String>,
    wordlist: String,
    threads: u64,
    timeout: u64,
    user_agent: String,
    http_client: Option<Arc<reqwest::Client>>,
}


impl BusterEngine {

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

    async fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        let content = fs::read_to_string(self.wordlist).await?;
        let words = content.lines().map(str::to_owned).collect::<Vec<_>>();

        let bust_fn = self.bust_fn(); 
        let bust_fn = Arc::new(bust_fn); 

        futures::stream::iter(words)
            .map(|word| {
                let url = Arc::clone(&self.url);
                let client = Arc::clone(&self.http_client);
                let bust_fn = Arc::clone(&bust_fn);

                async move {
                    bust_fn(url, client, word).await;
                }
            })
            .buffer_unordered(self.threads as usize) 
            .for_each(|_| async {})
            .await;
        Ok(())
    }
}

