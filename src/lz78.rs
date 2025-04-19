use std::fmt::Debug;

use serde::{Serialize, de::DeserializeOwned};

#[derive(Serialize, PartialEq)]
pub struct LZ78entry<T: DeserializeOwned + Serialize> {
    index: Option<usize>,
    next_char: Option<T>,
}

impl<T: DeserializeOwned + Serialize + Clone> LZ78entry<T> {
    fn resolve(&self, dictionary: &Vec<Vec<T>>) -> Vec<T> {
        let mut res = if let Some(index) = self.index {
            let target = &dictionary[index];
            target.clone()
        } else {
            Vec::with_capacity(1)
        };
        if let Some(next_char) = &self.next_char {
            res.push(next_char.clone());
        }
        return res;
    }
}

pub fn lz78_encode<T: DeserializeOwned + Serialize + Clone + PartialEq + Debug>(
    input: &[T],
) -> Vec<LZ78entry<T>> {
    let mut output = Vec::new();
    let mut dictionary: Vec<Vec<T>> = Vec::with_capacity(input.len());
    let mut i = 0;
    while i < input.len() {
        // Find the longest prefix in the dictionary
        let mut longest_prefix: Option<usize> = None;
        for (idx, entry) in dictionary.iter().enumerate() {
            for len in 1..=input.len() - i {
                if input[i..i + len] == *entry {
                    longest_prefix = Some(idx);
                    break;
                }
            }
        }
        let new_entry = if let Some(idx) = longest_prefix {
            // If we found a prefix, add it to the output
            i += dictionary[idx].len() + 1;
            if i - 1 >= input.len() {
                LZ78entry {
                    index: Some(idx),
                    next_char: None,
                }
            } else {
                LZ78entry {
                    index: Some(idx),
                    next_char: Some(input[i - 1].clone()),
                }
            }
        } else {
            // If we didn't find a prefix, add the current character to the dictionary
            i += 1;
            LZ78entry {
                index: None,
                next_char: Some(input[i - 1].clone()),
            }
        };
        dictionary.push(new_entry.resolve(&dictionary));
        output.push(new_entry);
    }
    return output;
}

pub fn lz78_decode<T: DeserializeOwned + Serialize + Clone + PartialEq>(
    input: &[LZ78entry<T>],
) -> Vec<T> {
    let mut output = Vec::new();
    let mut dictionary: Vec<Vec<T>> = Vec::with_capacity(input.len());
    for entry in input {
        let resolved = entry.resolve(&dictionary);
        for el in &resolved {
            output.push(el.clone());
        }
        dictionary.push(resolved);
    }
    return output;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_resolve() {
        let other: Vec<char> = "test".chars().collect();
        let dictionary: Vec<Vec<char>> = vec![vec!['t'], vec!['t', 'e'], vec!['t', 'e', 's']];
        let target = LZ78entry {
            index: Some(2),
            next_char: Some('t'),
        };
        assert_eq!(target.resolve(&dictionary), other);
    }

    #[test]
    fn test_lz78_encode_decode() {
        let input: Vec<char> = "rabarbarbar".chars().collect();
        let encoded = lz78_encode(&input);
        assert!(encoded.len() < input.len());
        let decoded = lz78_decode(&encoded);
        assert_eq!(input, decoded);
    }
}
