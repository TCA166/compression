/// Module providing LZ77 compression and decompression functions. The lz77
/// compression algorithm transforms a sequence of data into a sequence of
/// triples, using the previous data as a dictionary.
pub mod lz77;

/// Module providing LZ78 compression and decompression functions. The lz78
/// compression algorithm transforms a sequence of data into a sequence of
/// tuples, using the previous data as a dictionary.
pub mod lz78;

/// Module providing LZW compression and decompression functions. The lzw
/// compression algorithm is an iteration on the lz78 algorithm, removing the
/// second value in the tuple, at the cost of requiring an initial dictionary.
pub mod lzw;
