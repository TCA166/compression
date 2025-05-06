/// Burrows-Wheeler Transform (BWT) implementation
/// Transforms a slice of data, in a way that is useful for compression.
///
/// ## Arguments
///
/// - `input`: A slice of data to be transformed.
///
/// ## Returns
///
/// A tuple containing the transformed data and the index of the original data.
///
/// ## Example
///
/// ```
/// use generic_compression::transform::bwt::encode_bwt;
/// let input = b"banana";
/// let (encoded, index) = encode_bwt(input);
/// assert_eq!(encoded, vec![b'n', b'n', b'b', b'a', b'a', b'a']);
/// assert_eq!(index, 3);
/// ```
pub fn encode_bwt<T: Clone + Ord>(input: &[T]) -> (Vec<T>, usize) {
    let n = input.len();
    let mut rotations: Vec<_> = (0..n).collect();
    rotations.sort_by(|&a, &b| {
        input[a..]
            .iter()
            .chain(&input[..a])
            .cmp(input[b..].iter().chain(&input[..b]))
    });
    let result = rotations
        .iter()
        .map(|&i| input[(i + n - 1) % n].clone())
        .collect();
    let original_index = rotations.iter().position(|&i| i == 0).unwrap();
    (result, original_index)
}

/// Decodes a Burrows-Wheeler Transform (BWT) encoded data.
///
/// ## Arguments
///
/// - `input`: A slice of data to be decoded.
/// - `index`: The index of the original data.
///
/// ## Returns
///
/// A vector of data.
///
/// ## Example
///
/// ```
/// use generic_compression::transform::bwt::{decode_bwt, encode_bwt};
/// let input = b"banana";
/// let (encoded, index) = encode_bwt(input);
/// let decoded = decode_bwt(&encoded, index);
/// assert_eq!(decoded, vec![b'b', b'a', b'n', b'a', b'n', b'a']);
/// ```
pub fn decode_bwt<T: Clone + Ord>(input: &[T], index: usize) -> Vec<T> {
    let mut table = input.iter().enumerate().collect::<Vec<_>>();
    table.sort_by(|a, b| a.1.cmp(&b.1));
    let (mut i, el) = table[index];
    let mut result = vec![el.clone()];
    while i != index {
        let (j, el) = table[i];
        result.push(el.clone());
        i = j;
    }
    return result;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bwt() {
        let input = b"hello";
        let (encoded, index) = encode_bwt(input);
        assert_eq!(encoded, vec![b'h', b'o', b'e', b'l', b'l']);
        assert_eq!(index, 1);
    }

    #[test]
    fn test_bwt_decode() {
        let input = vec![b'h', b'o', b'e', b'l', b'l'];
        let index = 1;
        let decoded = decode_bwt(&input, index);
        assert_eq!(decoded, vec![b'h', b'e', b'l', b'l', b'o']);
    }
}
