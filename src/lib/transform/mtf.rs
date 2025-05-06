/// Encodes a sequence of elements using the Move-to-Front (MTF) algorithm.
///
/// ## Arguments
///
/// - `input`: A slice of elements to be encoded.
/// - `ordering`: A mutable reference to a vector representing the current ordering of elements.
///
/// ## Returns
///
/// A vector of indices representing the encoded elements.
///
/// ## Example
///
/// ```
/// use generic_compression::transform::mtf::encode_move_to_front;
/// let input = vec!['h', 'e', 'l', 'l', 'o'];
/// let mut ordering = vec!['e', 'h', 'l', 'o'];
/// let encoded = encode_move_to_front(&input, &mut ordering);
/// assert_eq!(encoded, vec![1, 1, 2, 0, 3]);
/// ```
pub fn encode_move_to_front<T: Eq + Clone>(input: &[T], ordering: &mut Vec<T>) -> Vec<usize> {
    let mut result = Vec::with_capacity(input.len());
    for el in input {
        let idx = ordering
            .iter()
            .position(|x| x == el)
            .expect("Element not found in ordering");
        result.push(idx);
        ordering.remove(idx);
        ordering.insert(0, el.clone());
    }
    return result;
}

/// Decodes a sequence of indices using the Move-to-Front (MTF) algorithm.
///
/// ## Arguments
///
/// - `input`: A slice of indices to be decoded.
/// - `ordering`: A mutable reference to a vector representing the current ordering of elements.
///
/// ## Returns
///
/// A vector of elements representing the decoded data.
///
/// ## Example
///
/// ```
/// use generic_compression::transform::mtf::{decode_move_to_front, encode_move_to_front};
/// let input = vec!['h', 'e', 'l', 'l', 'o'];
/// let mut ordering = vec!['e', 'h', 'l', 'o'];
/// let encoded = encode_move_to_front(&input, &mut ordering.clone());
/// let decoded = decode_move_to_front(&encoded, &mut ordering);
/// assert_eq!(decoded, input);
/// ```
pub fn decode_move_to_front<T: Eq + Clone>(input: &[usize], ordering: &mut Vec<T>) -> Vec<T> {
    let mut result = Vec::with_capacity(input.len());
    for idx in input {
        let el = ordering[*idx].clone();
        result.push(el.clone());
        ordering.remove(*idx);
        ordering.insert(0, el);
    }
    return result;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hello() {
        let mut ordering = vec!['e', 'h', 'l', 'o'];
        let input = vec!['h', 'e', 'l', 'l', 'o'];
        let encoded = encode_move_to_front(&input, &mut ordering);
        assert_eq!(encoded, vec![1, 1, 2, 0, 3]);
    }

    #[test]
    fn test_hello_decode() {
        let mut ordering = vec!['e', 'h', 'l', 'o'];
        let input = vec![1, 1, 2, 0, 3];
        let decoded = decode_move_to_front(&input, &mut ordering);
        assert_eq!(decoded, vec!['h', 'e', 'l', 'l', 'o']);
    }
}
