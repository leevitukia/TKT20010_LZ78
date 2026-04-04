use std::{fs::File, path::PathBuf};

mod lz78;
mod huffman;
mod bitwriter;
mod bitreader;
mod bit_traits;

use clap::{Parser, ValueEnum, Subcommand};

#[derive(Debug, Clone, PartialEq, Eq, ValueEnum)]
enum Algorithm {
    LZ78,
    Huffman
}


#[derive(Parser)]
struct Args {
    /// Algorithm to encode the file with
    #[arg(short, long)]
    algorithm: Algorithm,
}

#[derive(Parser)]
struct CliOptions {
    #[command(subcommand)]
    command: Commands,

    #[arg(short, long)]
    input_file: PathBuf,

    #[arg(short, long)]
    output_file: PathBuf,
}
#[derive(Subcommand)]
enum Commands {
    Encode(Args),
    Decode(Args),
}


fn main() {
    let args = CliOptions::parse();
    let input_file = File::open(&args.input_file).unwrap();
    let output_file = File::create(&args.output_file).unwrap();


    match &args.command {
        Commands::Encode(e) => {
            match e.algorithm {
                Algorithm::LZ78 => lz78::encode(input_file, output_file).expect("Failed to encode LZ78 file"),
                Algorithm::Huffman => huffman::encode(input_file, output_file).expect("Failed to encode Huffman file"),
            }
        },
        Commands::Decode(d) => {
            match d.algorithm {
                Algorithm::LZ78 => lz78::decode(input_file, output_file).expect("Failed to decode LZ78 file"),
                Algorithm::Huffman => huffman::decode(input_file, output_file).expect("Failed to decode Huffman file"),
            }
        }
    }

}