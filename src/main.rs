use clap::Parser;
mod bust;
mod dirbuster;
mod version;
use clap::ValueEnum;
use bust::Buster;
use dirbuster::DirBuster;

#[derive(ValueEnum, Clone, Debug)]
enum BusterMode {
    Dir,
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

    // Amount of light threads to use
    #[arg(long,default_value="100")]
    threads: u64,

    // User agent for Http headers
    #[arg(long,default_value="gbrute 0.1")]
    user_agent: String,

    // timeout per connection in milliseconds
    #[arg(long,default_value="1000")]
    timeout: u64,

    // Specify mode to bust
    #[arg(value_enum)]
    mode: BusterMode,

    // Blacklisted status codes
    #[arg(long, default_value="404", num_args=1.., value_delimiter=',')]
    status_code: Vec<u16>,

    // Enables recursive busting either for dir or dns mode
    #[arg(long)]
    recursive: bool,
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
  }
    println!("{menu}");
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    print_entry(&args, &args.mode);
    match args.mode {
        BusterMode::Dir => {
            let buster = DirBuster::new(args.url, args.wordlist, args.threads, args.timeout, args.user_agent, args.status_code, args.recursive);
            let _ = buster.expect("Failed to run dirbuster").run().await;
        }
   }
}
