use compress_lib::*;

use std::{
    fs::{File, read},
    path::PathBuf,
};

use ciborium::{self, into_writer};
use clap::{Parser, Subcommand};

#[derive(Subcommand)]
enum Algorithm {
    /// LZ77 compression algorithm
    LZ77 {
        /// The maximum offset to search for matches
        #[arg(short, long, default_value = "255")]
        window_size: usize,
        /// The maximum length of matches
        #[arg(short, long, default_value = "255")]
        lookahead_buffer_size: usize,
    },
    /// LZ78 compression algorithm
    LZ78 {
        /// The maximum offset to search for matches
        #[arg(short, long, default_value = "255")]
        lookahead_max: usize,
        /// The size of the dictionary
        #[arg(short, long, default_value = "255")]
        dictionary_size: usize,
    },
}

#[derive(Subcommand)]
enum Command {
    /// Compress the input file
    Compress {
        #[command(subcommand)]
        algorithm: Algorithm,
    },
    Decompress,
}

#[derive(Parser)]
#[command(version, about)]
struct Args {
    /// The input file to compress
    input: PathBuf,

    /// The output file to write the compressed data to
    #[arg(short, long, default_value = "compressed.out")]
    output: PathBuf,

    /// The compression algorithm to use (lz77 or lz78)
    #[command(subcommand)]
    command: Command,
}

fn main() {
    let args = Args::parse();

    // Read the input file
    let input_data = read(&args.input).expect("Failed to read input file");

    match args.command {
        Command::Compress { algorithm } => {
            let file = File::create(&args.output).expect("Failed to create output file");
            match algorithm {
                Algorithm::LZ77 {
                    window_size,
                    lookahead_buffer_size,
                } => into_writer(
                    &lz77_encode(&input_data, window_size, lookahead_buffer_size),
                    file,
                ),
                Algorithm::LZ78 {
                    lookahead_max,
                    dictionary_size,
                } => into_writer(
                    &lz78_encode(&input_data, lookahead_max, dictionary_size),
                    file,
                ),
            }
            .unwrap();
        }
        Command::Decompress => {
            // Decompression logic would go here
            println!("Decompression is not implemented yet.");
            return;
        }
    }
}
