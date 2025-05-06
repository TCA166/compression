/// Module providing the Burrows-Wheeler Transform (BWT). This is a somewhat
/// complex transform that reduces the entropy of the data, improving the
/// compression ratio of the data.
pub mod bwt;

/// Module providing the Move-To-Front (MTF) transform. A relatively simple
/// transform that is used to improve the compression ratio of the data,
/// usually in combination with other transforms.
pub mod mtf;
