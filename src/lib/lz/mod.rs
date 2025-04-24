/// Module providing LZ77 compression and decompression functions
mod lz77;
pub use lz77::{LZ77entry, LZ77tuple, lz77_decode, lz77_encode};

/// Module providing LZ78 compression and decompression functions
mod lz78;
pub use lz78::{LZ78entry, LZ78tuple, lz78_decode, lz78_encode};

/// Module providing LZW compression and decompression functions
mod lzw;
pub use lzw::{lzw_decode, lzw_encode};
