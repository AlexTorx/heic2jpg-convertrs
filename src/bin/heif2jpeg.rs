use std::path::PathBuf;
use std::process::exit;

use clap::Parser;
use log::LevelFilter;

use heif2jpeg::ThreadPool;


#[derive(Parser,Debug)]
#[command(author = "Alexis Tourneux", version, about = "Convert HEIF pictures to JPEG")]
struct Cli {
    #[arg(
        short,
        long,
        required = true,
        help = "Input target for conversion (either a file or a folder)",
    )]
    input: PathBuf, 
    #[arg(
        short,
        long,
        required = true,
        help = "Target for output of conversion (type must match input option)",
    )]
    output: PathBuf, 
}

fn main() -> () {

    // Init logger for use
    setup_logger();

    // Parse all command parameters
    let args = Cli::parse();

    // Sanity check that input and output both refer same object type (file or folder).
    // It would cause some trouble if the content of a folder is sent to a single file.
    if args.input.is_dir() != args.output.is_dir() {
        println!("Both input and output argument must be of same type");
        exit(1);
    }

    let pool = ThreadPool::new(num_cpus::get());
}

fn setup_logger() {
    env_logger::builder()
        .filter(None, LevelFilter::Info)
        .init();
}
