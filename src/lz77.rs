use serde::{Serialize, de::DeserializeOwned};

#[derive(Serialize, Debug)]
pub struct LZ77entry<T: DeserializeOwned + Serialize> {
    offset: usize,
    length: usize,
    next_char: Option<T>,
}

impl<T: DeserializeOwned + Serialize> From<(usize, usize, Option<T>)> for LZ77entry<T> {
    fn from(tuple: (usize, usize, Option<T>)) -> Self {
        LZ77entry {
            offset: tuple.0,
            length: tuple.1,
            next_char: tuple.2,
        }
    }
}

pub fn lz77_encode<T: DeserializeOwned + Serialize + PartialEq + Clone>(
    input: &[T],
    max_offset: usize,
    max_length: usize,
) -> Vec<LZ77entry<T>> {
    let mut output = Vec::new();
    let mut i = 0;

    while i < input.len() {
        let mut offset = 0;
        let mut length = 0;

        // Find the longest match
        for j in (i.saturating_sub(max_offset)..i).rev() {
            let mut k = 0;
            while k < max_length && i + k < input.len() && input[j + k] == input[i + k] {
                k += 1;
            }
            if k > length {
                length = k;
                offset = i - j;
            }
        }

        // If no match found, just output the next character
        if length == 0 {
            output.push(LZ77entry {
                offset: 0,
                length: 0,
                next_char: Some(input[i].clone()),
            });
            i += 1;
        } else {
            let next_char = if i + length < input.len() {
                Some(input[i + length].clone())
            } else {
                None
            };
            output.push(LZ77entry {
                offset,
                length,
                next_char,
            });
            i += length + 1;
        }
    }

    output
}

pub fn lz77_decode<T: Serialize + DeserializeOwned + Clone>(input: &[LZ77entry<T>]) -> Vec<T> {
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
