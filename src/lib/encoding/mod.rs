/// A module providing arithmetic encoding and decoding implementations.
pub mod arit;

/// A module providing huffman encoding and decoding implementations.
mod huffman;
pub use huffman::HuffmanEncoding;

pub mod elias;
