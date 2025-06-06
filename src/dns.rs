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
