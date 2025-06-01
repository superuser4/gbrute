use clap::Parser;

/// GBrute is a directory and web login bruteforcer
#[derive(Parser, Debug)]
#[command(version="0.1.0", about, long_about = None)]
struct Args {
    /// Url of the website
    #[arg(short,long)]
    url: String,
    
    /// Path to worlist
    #[arg(short,long)]
    wordlist: String,
}

fn main() {
    let args = Args::parse();
}
