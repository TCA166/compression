/// Module providing the Burrows-Wheeler Transform (BWT)
mod bwt;
pub use bwt::{decode_bwt, encode_bwt};

mod mtf;
pub use mtf::{decode_move_to_front, encode_move_to_front};
