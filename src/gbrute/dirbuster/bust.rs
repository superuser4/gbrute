use reqwest::Client;
use reqwest::StatusCode;
use tokio::sync::Semaphore;
use std::path::Path;
use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::sync::Arc;

async fn bust_dir(url: &String, client: &Client, dir: &str){
        let uri = url.to_owned() + dir;
    let response_builder: reqwest::RequestBuilder = client.get(&uri);
    let response = match response_builder.send().await {
        Ok(resp) => resp,
        Err(_) => return,
    };


    let code: StatusCode = response.status();
    if code.is_success() {
           println!("{dir}:{code}");
       }
}


pub async fn bust_dirs(url: String, wordlist: String) {
    let path = Path::new(&wordlist);
    let file = match File::open(path) {
        Ok(file) => file,
        Err(why) => panic!("Could not open wordlist: {}",why),
    }; 
    let mut uri: String = url.clone();
    if !uri.ends_with("/") {
        uri += "/";
    }

    let client = Arc::new(Client::new());
    let semaphore = Arc::new(Semaphore::new(200));

    let mut handles: Vec<tokio::task::JoinHandle<_>> = Vec::new();

    let reader = BufReader::new(file);
    for line in reader.lines() {
        let st: String = match line {
            Ok(line) => line,
            Err(_) => continue,
        };
        let cli = Arc::clone(&client);
        let sema = Arc::clone(&semaphore);
        let new_uri = uri.clone();

        let handle = tokio::spawn(async move {
            let _permit = match sema.acquire_owned().await {
                Ok(permit) => permit,
                Err(_e) => return,
            };
            bust_dir(&new_uri, &cli, st.as_str()).await; 
        });
        handles.push(handle);
    }
    for h in handles {
        let _ = h.await;
    }
}
