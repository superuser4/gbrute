use clap::Parser;
mod bust;
mod dirbuster;
mod dns;
mod fuzz;
mod version;
use clap::ValueEnum;
use bust::Buster;
use dirbuster::DirBuster;
use dns::DnsBuster;

#[derive(ValueEnum, Clone, Debug)]
enum BusterMode {
    Dir,
    Dns,
    Fuzz,
}

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

    #[arg(long,default_value="100")]
    threads: u64,

    #[arg(long,default_value="gbrute")]
    user_agent: String,

    #[arg(long,default_value="1000")]
    timeout: u64,

    #[arg(value_enum)]
    mode: BusterMode,
}

fn print_entry(args: &Args) {
    println!("Starting GBrute {} at {}",version::GBRUTE_VERSION,chrono::Local::now().format("%Y-%m-%d %H:%M:%S") );
    println!("----------------------------------------------------------------------------------");
    let menu = format!("\
    [*] Url: {}\n\
    [*] Threads: {}\n\
    [*] Wordlist: {}\n\
    [*] Ignored Statuscodes: 404\n\
    [*] User-Agent: {}\n\
    [*] Timeout: {}\n", args.url, args.threads, args.wordlist, args.user_agent, args.timeout);
    println!("{menu}");
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    print_entry(&args);
    match args.mode {
        BusterMode::Dir => {
            let buster = DirBuster::new(args.url, args.wordlist, args.threads, args.timeout, args.user_agent); 
            let _ = buster.expect("Failed to run dirbuster").run().await;
        }
        BusterMode::Dns => {
            let mut dnsbuster = DnsBuster::new(args.url, args.wordlist, args.threads, args.timeout, args.user_agent);
            let _ = dnsbuster.run().await;
        }
        BusterMode::Fuzz => todo!(),
    }
}
