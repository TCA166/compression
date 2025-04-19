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
/// use compress_lib::encode_bwt;
/// let input = b"banana";
/// let (encoded, index) = encode_bwt(input);
/// assert_eq!(encoded, vec![b'b', b'n', b'n', b'a', b'a', b'a']);
/// assert_eq!(index, 3);
/// ```
pub fn encode_bwt<T: Clone + PartialEq + Ord>(input: &[T]) -> (Vec<T>, usize) {
    let mut table = input
        .iter()
        .zip((0..input.len()).map(|i| {
            input
                .get(i.checked_sub(1).unwrap_or(input.len() - 1))
                .unwrap()
        }))
        .enumerate()
        .collect::<Vec<_>>();
    table.sort_by(|a, b| a.1.0.cmp(&b.1.0).then_with(|| a.0.cmp(&b.0)));
    let index = table
        .iter()
        .position(|x| x.0 == 0)
        .expect("Failed to find the original index");
    return (
        table.iter().map(|x| x.1.1.clone()).collect::<Vec<_>>(),
        index,
    );
}

/// Decodes a Burrows-Wheeler Transform (BWT) encoded data.
///
/// ## Arguments
///
/// - `input`: A slice of data to be decoded.
///
/// ## Returns
///
/// A vector of data.
///
/// ## Example
///
/// ```
/// use compress_lib::{decode_bwt, encode_bwt};
/// let input = b"banana";
/// let (encoded, index) = encode_bwt(input);
/// let decoded = decode_bwt(&encoded, index);
/// assert_eq!(decoded, vec![b'b', b'a', b'n', b'a', b'n', b'a']);
/// ```
pub fn decode_bwt<T: Clone + PartialEq + Ord>(input: &[T], index: usize) -> Vec<T> {
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
