use clap::Parser;
use std::path::PathBuf;

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

fn main() {
    let cli = Cli::parse();

    println!("input_file: {}", cli.input_file.display());
}
