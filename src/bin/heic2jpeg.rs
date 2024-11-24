use std::path::PathBuf;
use std::process::exit;

use clap::Parser;
use log::{error,info,LevelFilter};

use heic2jpeg::{ThreadPool,convert_heic_to_jpeg,utils};


#[derive(Parser,Debug)]
#[command(author = "Alexis Tourneux", version, about = "Convert HEIF pictures to JPEG")]
struct Cli {
    #[arg(
        help = "Input target for conversion (either a file or a folder)",
    )]
    input: PathBuf, 
    #[arg(
        help = "Target for output of conversion (type must match input option)",
    )]
    output: PathBuf, 
    #[arg(
        short,
        long,
        help = "Number of workers to spawn for conversion",
        default_value_t = num_cpus::get(),
    )]
    workers: usize,
}

fn main() -> () {

    // Init logger for use
    setup_logger();

    // Parse all command parameters
    let args = Cli::parse();

    // Sanity check to make sure input argument actually exists
    // If input argument does not exist (nor a folder or a file), it means there is
    // no target to start from, and processing cannot start
    if !args.input.exists() {
        error!("Input parameter must be a valid file/directory!");
        exit(1);
    }

    let mut images: Vec<PathBuf> = Vec::new();
    if args.input.is_dir() {
        // Create the list of executions
        images = args.input
            .read_dir()
            .unwrap()
            .filter_map(|f| f.ok())
            .filter(|f| match f.path().extension() {
                None => false,
                Some(ex) => ex.to_str().unwrap().to_lowercase() == "heic",
            })
            .map(|f| f.path())
            .collect::<Vec<PathBuf>>();
    } else {
        images.push(args.input.clone());
    }

    // Create the thread pool for parallel processing
    let pool = ThreadPool::new(args.workers);

    // Send all tasks to the thread pool
    for image in images {
        let input = args.input.clone();
        let output = args.output.clone();
        pool.spawn(move || {
            if input.is_dir() {
                let jpeg_file = utils::generate_jpeg_filename_from_heif(&image, &output);
                convert_heic_to_jpeg(&image, &jpeg_file)
            } else {
                convert_heic_to_jpeg(&image, &output);
            }
        });
    }

    // Wait for all tasks to be executed
    pool.join();
}

fn setup_logger() {
    env_logger::builder()
        .filter(None, LevelFilter::Debug)
        .init();
}
