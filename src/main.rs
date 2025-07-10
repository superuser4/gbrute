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
use fuzz::FuzzBuster;

#[derive(ValueEnum, Clone, Debug)]
enum BusterMode {
    Dir,
    Dns,
    Fuzz,
}

/// GBrute is a web directory, dns, and parameter brute-forcer
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

    #[arg(long,default_value="gbrute 0.1")]
    user_agent: String,

    #[arg(long,default_value="1000")]
    timeout: u64,

    #[arg(value_enum)]
    mode: BusterMode,

    #[arg(long, default_value="404", num_args=1.., value_delimiter=',')]
    status_code: Vec<u16>,
}

fn print_entry(args: &Args, mode: &BusterMode) {
    println!("Starting GBrute {} at {}",version::GBRUTE_VERSION,chrono::Local::now().format("%Y-%m-%d %H:%M:%S") );
    println!("----------------------------------------------------------------------------------");

    let mut menu = format!("\
    [*] Url: {}\n\
    [*] Threads: {}\n\
    [*] Wordlist: {}\n\
    [*] Timeout: {}\n", args.url, args.threads, args.wordlist, args.timeout);

    match mode {
        BusterMode::Dir => {
            menu += format!("[*] Ignored Status Codes: {:?} \n", args.status_code).as_str();
            menu += format!("[*] User-Agent: {}\n", args.user_agent).as_str();
        }
        BusterMode::Dns => {
        }
        BusterMode::Fuzz => {
        }
    }
    println!("{menu}");
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    print_entry(&args, &args.mode);
    match args.mode {
        BusterMode::Dir => {
            let buster = DirBuster::new(args.url, args.wordlist, args.threads, args.timeout, args.user_agent, args.status_code);
            let _ = buster.expect("Failed to run dirbuster").run().await;
        }
        BusterMode::Dns => {
            let mut dnsbuster = DnsBuster::new(args.url, args.wordlist, args.threads, args.timeout, args.user_agent);
            let _ = dnsbuster.run().await;
        }
        BusterMode::Fuzz => {
            let mut fuzzbuster = FuzzBuster::new(args.url, args.wordlist, args.threads, args.timeout, args.user_agent);
            let _ = fuzzbuster.run().await;
        }
    }
}
