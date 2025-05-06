/// A module providing arithmetic encoding and decoding implementations.
/// Arithmetic encoding is a form of encoding where a sequence is assigned a
/// rational number in the range [0, 1) based on the frequency of the symbols
/// in the sequence. When using this implementation, make sure the chosen
/// integer type can hold the range of values you expect to encode.
pub mod arit;

/// A module providing Huffman encoding and decoding implementations.
mod huffman;
pub use huffman::HuffmanEncoding;

/// A module providing Elias encoding algorithms, used for representing
/// arbitrary integers greater than zero. These algorithms all are based on the
/// concept of prefixing the binary representation of a number with unary
/// encoding of its length.
pub mod elias;
