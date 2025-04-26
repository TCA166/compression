/// A module providing arithmetic encoding and decoding implementations.
mod arit;
pub use arit::{arithmetic_decode, arithmetic_encode};

/// A module providing huffman encoding and decoding implementations.
mod huffman;
pub use huffman::HuffmanEncoding;
