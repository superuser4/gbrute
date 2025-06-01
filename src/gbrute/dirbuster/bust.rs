use reqwest::blocking::get;
use std::path::Path;
use std::fs::File;
use std::io::{prelude::*, BufReader};

fn bust_dir(url: &String){ 
       let response = match get(url) {
           Ok(response) => response,
           Err(_) => todo!(),
       };
       if response.status() == 200 {
           println!("{url}: 200 OK")
       }
}


pub fn bust_dirs(url: String, wordlist: String) {
    let path = Path::new(&wordlist);

    let file = match File::open(path) {
        Ok(file) => file,
        Err(why) => panic!("Could not open wordlist: {}",why),
    };

    let reader = BufReader::new(file);
    for line in reader.lines() {
        let mut uri: String = url.clone();
        if !uri.ends_with("/") {
            uri += "/";
        }
        let st: String = match line {
            Ok(line) => line,
            Err(_) => todo!(),
        };
        
        uri += &st;

        bust_dir(&uri);
    }
}
