use generic_compression::{
    lz::{lz77::*, lz78::*, lzw::*},
    transform::{bwt::*, mtf::*},
};

use std::{
    fs::{File, read},
    io::{Read, Write},
    path::PathBuf,
};

/// Module providing a simple serialization and deserialization interface, optimized for output size.
mod io;
use io::{
    deserializer::{deserialize_lz77, deserialize_lz78, deserialize_lzw},
    serializer::{serialize_lz77, serialize_lz78, serialize_lzw},
};

use clap::{Parser, Subcommand};

const HEADER_SIZE: usize = 3;
const LZ77_HEADER: &[u8; HEADER_SIZE] = b"l77";
const LZ78_HEADER: &[u8; HEADER_SIZE] = b"l78";
const LZW_HEADER: &[u8; HEADER_SIZE] = b"lzw";
const STACK_HEADER: &[u8; HEADER_SIZE] = b"stk";

const LZW_DICIONARY: &[u8; 256] = &{
    let mut array = [0u8; 256];
    let mut i = 0;
    while i < 256 {
        array[i] = i as u8;
        i += 1;
    }
    array
};

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
    /// LZW compression algorithm
    LZW {
        /// The maximum offset to search for matches
        #[arg(short, long, default_value = "255")]
        lookahead_max: usize,
    },
    /// LZW compression algorithm with move-to-front and Burrows-Wheeler transform
    STACK {
        /// The maximum offset to search for matches
        #[arg(short, long, default_value = "255")]
        lookahead_max: usize,
    },
}

#[derive(Subcommand)]
enum Command {
    /// Compress the input file
    Compress {
        #[command(subcommand)]
        algorithm: Algorithm,
    },
    /// Decompress the input file
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
            let mut file = File::create(&args.output).expect("Failed to create output file");
            match algorithm {
                Algorithm::LZ77 {
                    window_size,
                    lookahead_buffer_size,
                } => {
                    file.write(LZ77_HEADER).unwrap();
                    serialize_lz77(
                        lz77_encode(&input_data, window_size, lookahead_buffer_size),
                        window_size,
                        lookahead_buffer_size,
                        &mut file,
                    )
                }
                Algorithm::LZ78 {
                    lookahead_max,
                    dictionary_size,
                } => {
                    file.write(LZ78_HEADER).unwrap();
                    file.write_all(&dictionary_size.to_le_bytes()).unwrap();
                    serialize_lz78(
                        lz78_encode(&input_data, lookahead_max, dictionary_size),
                        dictionary_size,
                        &mut file,
                    )
                }
                Algorithm::LZW { lookahead_max } => {
                    file.write(LZW_HEADER).unwrap();
                    serialize_lzw(
                        lzw_encode(&input_data, LZW_DICIONARY, lookahead_max),
                        &mut file,
                    )
                }
                Algorithm::STACK { lookahead_max } => {
                    file.write(STACK_HEADER).unwrap();
                    let (bwt, index) = encode_bwt(&input_data);
                    file.write_all(&index.to_le_bytes()).unwrap();
                    let mut ordering = LZW_DICIONARY.to_vec();
                    let mtf = encode_move_to_front(&bwt, &mut ordering);
                    let mtf = mtf.into_iter().map(|x| x as u8).collect::<Vec<_>>();
                    serialize_lzw(
                        lzw_encode(mtf.as_slice(), LZW_DICIONARY, lookahead_max),
                        &mut file,
                    )
                }
            }
            .unwrap();
        }
        Command::Decompress => {
            let mut file = File::open(&args.input).expect("Failed to open input file");
            let mut header = [0; HEADER_SIZE];
            file.read_exact(&mut header)
                .expect("Failed to read header from input file");
            let data = match &header {
                LZ77_HEADER => {
                    let data: Vec<LZ77entry<u8>> =
                        deserialize_lz77(&mut file).expect("Failed to decode LZ77 data");
                    lz77_decode(&data)
                }
                LZ78_HEADER => {
                    let mut dictionary_size_buf = [0; 8];
                    file.read_exact(&mut dictionary_size_buf)
                        .expect("Failed to read dictionary_size from input file");
                    let dictionary_size = usize::from_le_bytes(dictionary_size_buf);

                    let data: Vec<LZ78entry<u8>> =
                        deserialize_lz78(&mut file).expect("Failed to decode LZ78 data");
                    lz78_decode(&data, dictionary_size)
                }
                LZW_HEADER => {
                    let data: Vec<usize> =
                        deserialize_lzw(&mut file).expect("Failed to decode LZW data");
                    lzw_decode(&data, LZW_DICIONARY)
                }
                STACK_HEADER => {
                    let mut index_buf = [0; 8];
                    file.read_exact(&mut index_buf)
                        .expect("Failed to read index from input file");
                    let index = usize::from_le_bytes(index_buf);
                    let data: Vec<usize> =
                        deserialize_lzw(&mut file).expect("Failed to decode LZW data");
                    let mut ordering = LZW_DICIONARY.to_vec();
                    let mtf = lzw_decode(&data, &ordering);
                    let mtf = mtf.into_iter().map(|x| x as usize).collect::<Vec<_>>();
                    let bwt = decode_move_to_front(mtf.as_slice(), &mut ordering);
                    let bwt = bwt.into_iter().map(|x| x as u8).collect::<Vec<_>>();
                    decode_bwt(bwt.as_slice(), index)
                }
                header => panic!("Unknown compression algorithm: {:?}", header),
            };
            let mut output_file = File::create(&args.output).expect("Failed to create output file");
            output_file
                .write_all(&data)
                .expect("Failed to write decompressed data");
            output_file.flush().expect("Failed to flush output file");
        }
    }
}
