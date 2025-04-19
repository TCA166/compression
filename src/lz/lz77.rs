use serde::{Deserialize, Serialize};

/// A struct to represent an LZ77 entry
/// Traditionally a LZ77 entry is represented as a tuple of (offset, length, next_char)
/// where offset is the distance to the last occurrence of the string, length is the length of the
/// string, and next_char is the next character in the string.
///
/// In this implementation, we use a struct to represent the entry.
/// This is more Rust-idiomatic and allows us to use the `serde` crate for serialization and deserialization.
#[derive(Debug)]
pub struct LZ77entry<T> {
    offset: usize,
    length: usize,
    next_char: Option<T>,
}

/// A tuple to represent an LZ77 entry
pub type LZ77tuple<T> = (usize, usize, Option<T>);

impl<T> From<LZ77tuple<T>> for LZ77entry<T> {
    fn from(tuple: (usize, usize, Option<T>)) -> Self {
        LZ77entry {
            offset: tuple.0,
            length: tuple.1,
            next_char: tuple.2,
        }
    }
}

impl<T> Into<LZ77tuple<T>> for LZ77entry<T> {
    fn into(self) -> (usize, usize, Option<T>) {
        (self.offset, self.length, self.next_char)
    }
}

impl Serialize for LZ77entry<u8> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let tuple = (self.offset, self.length, self.next_char);
        tuple.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for LZ77entry<u8> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let tuple = <(usize, usize, Option<u8>)>::deserialize(deserializer)?;
        Ok(LZ77entry::from(tuple))
    }
}

/// A function to encode a slice of data using the LZ77 algorithm
/// The function takes a slice of data, a maximum offset, and a maximum length.
/// It returns a vector of LZ77 entries.
///
/// ## Arguments
///
/// - `input`: A slice of data to be encoded.
/// - `max_offset`: The maximum offset to search for matches.
/// - `max_length`: The maximum length of matches.
///
/// ## Returns
///
/// A vector of LZ77 entries.
///
/// ## Example
///
/// ```
/// use compress_lib::lz77_encode;
/// let input = b"ABABABABA";
/// let encoded = lz77_encode(input, 4, 4);
/// assert!(encoded.len() < input.len());
/// ```
///
pub fn lz77_encode<T: PartialEq + Clone>(
    input: &[T],
    max_offset: usize,
    max_length: usize,
) -> Vec<LZ77entry<T>> {
    /// A struct to represent a match in the input data
    struct Match {
        pub offset: usize,
        pub length: usize,
    }

    let mut output = Vec::new();
    let mut i = 0; // our position in the input

    while i < input.len() {
        let mut m: Option<Match> = None; // the longest match

        // Find the longest match
        for j in (i.saturating_sub(max_offset)..i).rev() {
            let mut k = 0;
            // as long as we are within bounds, and the characters match
            while k < max_length && i + k < input.len() && input[j + k] == input[i + k] {
                k += 1; // increment the length of the match
            }
            if k > m.as_ref().map_or(0, |m| m.length) {
                // update the longest match
                m = Some(Match {
                    offset: i - j,
                    length: k,
                });
            }
        }

        // If no match found, just output the next character
        if let Some(m) = m {
            // check if we can get the next character
            let next_char = if i + m.length < input.len() {
                Some(input[i + m.length].clone())
            } else {
                None // if not, then we set it to None
            };
            output.push(LZ77entry {
                offset: m.offset,
                length: m.length,
                next_char,
            });
            i += m.length + 1;
        } else {
            // we found nothing, so we just output the next character
            output.push(LZ77entry {
                offset: 0,
                length: 0,
                next_char: Some(input[i].clone()),
            });
            i += 1;
        }
    }

    output
}

/// A function to decode a vector of LZ77 entries
/// The function takes a vector of LZ77 entries and returns a vector of data.
///
/// ## Arguments
///
/// - `input`: A vector of LZ77 entries to be decoded.
///
/// ## Returns
///
/// A vector of data.
///
/// ## Example
///
/// ```
/// use compress_lib::{lz77_decode, lz77_encode};
/// let input = b"ABABABABA";
/// let encoded = lz77_encode(input, 4, 4);
/// assert!(encoded.len() < input.len());
/// let decoded = lz77_decode(&encoded);
/// assert_eq!(input.to_vec(), decoded);
/// ```
pub fn lz77_decode<T: Clone>(input: &[LZ77entry<T>]) -> Vec<T> {
    let mut output: Vec<T> = Vec::new();

    for entry in input {
        // foreach entry
        let start = output.len() - entry.offset;
        for i in 0..entry.length {
            // copy the match
            output.push(output[start + i].clone());
        }
        // if we have a next character, we push it to the output
        if let Some(next_char) = &entry.next_char {
            output.push(next_char.clone());
        }
    }

    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lz77() {
        let input = b"RATABARBARATABARBARAT";

        let encoded = lz77_encode(input, 4, 4);
        let decoded = lz77_decode(&encoded);

        assert_eq!(input.to_vec(), decoded);
    }

    #[test]
    fn test_lz77_empty() {
        let input: Vec<u8> = vec![];

        let encoded = lz77_encode(&input, 4, 4);
        let decoded = lz77_decode(&encoded);

        assert_eq!(input, decoded);
    }

    #[test]
    fn test_nasty_decode() {
        let input = vec![
            LZ77entry {
                offset: 0,
                length: 0,
                next_char: Some(1),
            },
            LZ77entry {
                offset: 1,
                length: 5,
                next_char: Some(2),
            },
        ];
        let decoded = lz77_decode(&input);
        assert_eq!(decoded, vec![1, 1, 1, 1, 1, 1, 2]);
    }
}
