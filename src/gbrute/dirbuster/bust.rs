use tokio::sync::Semaphore;
use tokio::io::{BufReader, AsyncBufReadExt};
use tokio::fs::File;
use tokio::task::JoinHandle;
use std::{error::Error, sync::Arc};

async fn bust_dir(url: &String, client: &reqwest::Client, dir: &str){
    let uri = url.to_owned() + dir;
    let response_builder: reqwest::RequestBuilder = client.head(&uri);
    let response = match response_builder.send().await {
        Ok(resp) => resp,
        Err(_) => return,
    };


    let code: reqwest::StatusCode = response.status();
    if code.is_success() || code.is_redirection() { 
           println!("Busted: {dir}:{code}");
    }
}


fn create_client(user_agent: &str, timeout: u64) -> Result<reqwest::Client, Box<dyn Error>> {
    let headers: reqwest::header::HeaderMap = Default::default();
    let client =
        reqwest::ClientBuilder::new()
        .user_agent(user_agent)
        .default_headers(headers)
        .timeout(std::time::Duration::from_millis(timeout))
        .redirect(reqwest::redirect::Policy::none())
        .pool_max_idle_per_host(200)
        .build()?;
    Ok(client)
}

pub async fn bust_dirs(url: String, wordlist: String) -> Result<(), Box<dyn Error>> {
   let mut uri: String = url.clone();
    if !uri.ends_with("/") {
        uri.push('/');
    }

    let client = create_client("gbrute", 750)?;
    let use_cli = Arc::new(client);
    let semaphore = Arc::new(Semaphore::new(200));

    
    let file = File::open(wordlist).await?;
    let reader = BufReader::new(file);
    let mut lines = reader.lines();

    let mut handles: Vec<JoinHandle<_>> = Vec::new();

    while let Some(line) = lines.next_line().await? {
          
        let cli_clone = Arc::clone(&use_cli);
        let uri_clone = uri.clone();
        let dir = line.clone();
        let permit = semaphore.clone().acquire_owned().await.unwrap();

        let handle: JoinHandle<()> = tokio::spawn( async move {
            bust_dir(&uri_clone, &cli_clone, dir.as_str()).await;
            drop(permit);
        });
        handles.push(handle);
    }
    for i in handles {
        let _ = i.await;
    }
    Ok(())
}
