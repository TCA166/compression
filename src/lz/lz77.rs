use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub struct LZ77entry<T> {
    offset: usize,
    length: usize,
    next_char: Option<T>,
}

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
            let next_char = if i + m.length < input.len() {
                Some(input[i + m.length].clone())
            } else {
                None
            };
            output.push(LZ77entry {
                offset: m.offset,
                length: m.length,
                next_char,
            });
            i += m.length + 1;
        } else {
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

pub fn lz77_decode<T: Clone>(input: &[LZ77entry<T>]) -> Vec<T> {
    let mut output: Vec<T> = Vec::new();

    for entry in input {
        let start = output.len() - entry.offset;
        for i in 0..entry.length {
            output.push(output[start + i].clone());
        }
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
        let input = b"ABABABABA";

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
