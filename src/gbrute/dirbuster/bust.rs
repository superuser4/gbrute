use futures::stream::StreamExt;
use std::{error::Error, sync::Arc};
use tokio::fs;

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

    let use_cli = Arc::new(create_client("gbrute", 1000)?);
    let cont = fs::read_to_string(wordlist).await?;
    let dirs = cont.lines().map(String::from);

    futures::stream::iter(dirs)
        .map( |dir| {
           let client = Arc::clone(&use_cli); 
           let uri = uri.clone();
           async move {
               bust_dir(&uri, &client, dir.as_str()).await;
           }
    })
    .buffer_unordered(200)
    .for_each(|_| async {})
    .await;
            
    Ok(())

}
