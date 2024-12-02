use clap::Parser;
use std::path::PathBuf;

use crate::hybrid_hash_map::HybridMapInterface;
// use crate::type_state;
// use crate::type_state::ColType;
use gperftools::profiler::PROFILER;
use std::{error::Error, process};

mod csv_read;
mod hybrid_hash_map;
// mod type_state;

// mod file_read;
// mod huffman_encode;
// mod stats;

fn parse_input_file(s: &str) -> Result<PathBuf, String> {
    let path = PathBuf::from(s);
    if !path.is_file() {
        return Err(format!("Input path, '{}', is not a regular file", path.display()));
    }
    Ok(PathBuf::from(s))
}

#[derive(Parser)]
#[command(version, about = "Huffman compressor", long_about = None, author= "Mike Hancock")]
struct Cli {
    /// Input file to compress
    #[arg(index=1, value_name = "FILE", value_parser =parse_input_file )]
    input_file: PathBuf,
}

fn run() -> Result<(), Box<dyn Error>> {
    PROFILER.lock().unwrap().start("./prof.prof").unwrap();
    let cli = Cli::parse();

    // let _col = crate::type_state::ColBuilder::new(ColType::U8).build();

    // file_read::process_file(&cli.input_file)?;
    csv_read::process_file(&cli.input_file)?;

    let mut h = crate::hybrid_hash_map::make_map::<u8, u8, fnv_rs::FnvBuildHasher>();
    // let mut h: crate::hybrid_hash_map::HybridHash<u8, u8, fnv_rs::FnvBuildHasher> = Default::default();
    h.bump(0);

    println!("fin");
    PROFILER.lock().unwrap().stop().unwrap();

    Ok(())
}

fn main() {
    if let Err(err) = run() {
        println!("{}", err);
        process::exit(1);
    }
}

#[test]
fn verify_cli() {
    use clap::CommandFactory;
    Cli::command().debug_assert()
}
