/// A function to encode a slice of data using the LZW algorithm
/// The function takes a slice of data, an initial dictionary, and a maximum lookahead size.
/// It returns a vector of indices representing the encoded data.
///
/// ## Arguments
///
/// - `input`: A slice of data to be encoded.
/// - `initial`: An initial dictionary to start encoding.
/// - `max_lookahead`: The maximum lookahead size.
///
/// ## Returns
///
/// A vector of indices representing the encoded data.
///
/// ## Example
///
/// ```
/// use generic_compression::lz::lzw::lzw_encode;
/// let input = b"ABABABABA";
/// let initial = b"AB";
/// let encoded = lzw_encode(input, initial, 4);
/// assert_eq!(encoded, vec![0, 1, 2, 4, 3]);
/// ```
pub fn lzw_encode<T: Clone + PartialEq>(
    input: &[T],
    initial: &[T],
    max_lookahead: usize,
) -> Vec<usize> {
    let mut dictionary: Vec<Vec<T>> = Vec::with_capacity(initial.len());
    for i in initial {
        dictionary.push(vec![i.clone()]);
    }
    let mut output: Vec<usize> = Vec::new();

    let mut i = 0;
    while i < input.len() {
        // Find the longest prefix in the dictionary
        let mut longest_prefix: Option<usize> = None;
        for (idx, entry) in dictionary.iter().enumerate() {
            let entry_len = entry.len();
            if entry_len > max_lookahead
                || i + entry_len > input.len()
                || input[i..i + entry_len] != *entry
            {
                continue;
            }
            if let Some(longest) = &mut longest_prefix {
                if entry_len > dictionary[*longest].len() {
                    *longest = idx;
                }
            } else {
                longest_prefix = Some(idx);
            }
        }
        // If we found a prefix, add it to the output
        if let Some(idx) = longest_prefix {
            i += dictionary[idx].len();
            output.push(idx);
            // if it is ok, add the next entry to the dictionary
            if i < input.len() {
                let next_char = input[i].clone();
                let mut new_entry = dictionary[idx].clone();
                new_entry.push(next_char);
                // then add it to the dictionary
                if !dictionary.contains(&new_entry) {
                    dictionary.push(new_entry);
                }
            }
        } else {
            panic!("No match found in dictionary");
        }
    }
    return output;
}

/// A function to decode a vector of indices using the LZW algorithm
/// The function takes a vector of indices and an initial dictionary.
/// It returns a vector of data.
///
/// ## Arguments
///
/// - `input`: A vector of indices to be decoded.
/// - `initial`: An initial dictionary to start decoding.
///
/// ## Returns
///
/// A vector of data.
///
/// ## Example
///
/// ```
/// use generic_compression::lz::lzw::{lzw_decode, lzw_encode};
/// let input = b"ABABABABA";
/// let initial = b"AB";
/// let encoded = lzw_encode(input, initial, 4);
/// let decoded = lzw_decode(&encoded, initial);
/// assert_eq!(input.to_vec(), decoded);
/// ```
pub fn lzw_decode<T: Clone + PartialEq>(input: &[usize], initial: &[T]) -> Vec<T> {
    let mut dictionary: Vec<Vec<T>> = Vec::with_capacity(initial.len());
    for i in initial {
        dictionary.push(vec![i.clone()]);
    }
    let mut output: Vec<T> = Vec::new();

    let mut i = 0;
    while i < input.len() {
        // we get the token
        let idx = input[i];
        let entry = dictionary[idx].clone();
        output.extend(entry.clone()); // decode it
        if i + 1 < input.len() {
            let next_idx = input[i + 1];
            if next_idx < dictionary.len() {
                // if it's a simple token we just add it to the dictionary
                let next_entry = dictionary[next_idx].clone();
                let mut new_entry = entry.clone();
                new_entry.push(next_entry[0].clone());
                if !dictionary.contains(&new_entry) {
                    dictionary.push(new_entry);
                }
            } else {
                // well this is the unique case
                let mut new_entry = entry.clone();
                new_entry.push(entry[0].clone()); // instead of next_entry[0]
                if !dictionary.contains(&new_entry) {
                    dictionary.push(new_entry);
                }
            }
        }
        i += 1;
    }
    return output;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lzw() {
        let input = b"ABABABABA";
        let initial = b"AB";
        let encoded = lzw_encode(input, initial, 4);
        assert_eq!(encoded, vec![0, 1, 2, 4, 3]);
    }

    #[test]
    fn test_lzw_decode() {
        let input = b"ABABABABA";
        let initial = b"AB";
        let encoded = lzw_encode(input, initial, 4);
        let decoded = lzw_decode(&encoded, initial);
        assert_eq!(input.to_vec(), decoded);
    }

    #[test]
    fn test_rabarbar() {
        let input = b"rabarbarbar";
        let initial = b"rab";
        let encoded = lzw_encode(input, initial, 4);
        assert!(encoded.len() < input.len());
        let decoded = lzw_decode(&encoded, initial);
        assert_eq!(input, decoded.as_slice());
    }
}
