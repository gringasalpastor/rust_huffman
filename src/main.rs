use clap::Parser;
use std::path::PathBuf;

mod huffman_encode;
mod stats;

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
    let stats = stats::read_file(&cli.input_file);
    let symbol_table = huffman_encode::make_symbol_table(&stats);

    println!("{:#?}", symbol_table);
    println!("entropy H(x): {:?} bits/symbol(byte)", stats.entropy());
}

#[test]
fn verify_cli() {
    use clap::CommandFactory;
    Cli::command().debug_assert()
}
