use std::{error::Error, sync::Arc};
use futures::{StreamExt, TryStreamExt};
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
 
    fn create_client(&mut self) -> Result<(), Box<dyn Error>> {
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

    pub async fn run<F, Fut>(&mut self, task: F) -> Result<(), Box<dyn Error + Send>>
    where
        F: Fn(String) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<(), Box<dyn Error + Send>>> + Send + 'static,
    {

        self.create_client()?;
        let content = fs::read_to_string(&self.wordlist).await?;
        let words = content.lines().map(str::to_owned).collect::<Vec<_>>();

        futures::stream::iter(words)
            .map(Ok)
            .try_for_each_concurrent(
                self.threads as usize,
                move |word| {
                    async move {
                        task(word).await
                    }
            },
        )
        .await?;
        Ok(())
    }
}

