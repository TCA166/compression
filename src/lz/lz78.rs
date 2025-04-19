use std::fmt::Debug;

use serde::{Deserialize, Serialize};

#[derive(PartialEq)]
pub struct LZ78entry<T> {
    index: Option<usize>,
    next_char: Option<T>,
}

pub type LZ78tuple<T> = (Option<usize>, Option<T>);

impl<T> From<LZ78tuple<T>> for LZ78entry<T> {
    fn from(tuple: (Option<usize>, Option<T>)) -> Self {
        LZ78entry {
            index: tuple.0,
            next_char: tuple.1,
        }
    }
}

impl<T> Into<LZ78tuple<T>> for LZ78entry<T> {
    fn into(self) -> (Option<usize>, Option<T>) {
        (self.index, self.next_char)
    }
}

impl Serialize for LZ78entry<u8> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let tuple = (self.index, self.next_char);
        tuple.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for LZ78entry<u8> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let tuple = <(Option<usize>, Option<u8>)>::deserialize(deserializer)?;
        Ok(LZ78entry::from(tuple))
    }
}

impl<T: Clone> LZ78entry<T> {
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

pub fn lz78_encode<T: Clone + PartialEq + Debug>(
    input: &[T],
    lookahead_max: usize,
    max_dictionary_size: usize,
) -> Vec<LZ78entry<T>> {
    let mut output = Vec::new();
    let mut dictionary: Vec<Vec<T>> = Vec::with_capacity(max_dictionary_size);
    let mut i = 0;
    while i < input.len() {
        // Find the longest prefix in the dictionary
        let mut longest_prefix: Option<usize> = None;
        for (idx, entry) in dictionary.iter().enumerate() {
            for len in 1..=input.len() - i {
                if len > lookahead_max {
                    break;
                }
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
        let new_dict_entry = new_entry.resolve(&dictionary);
        // If the dictionary is full, remove the oldest entry
        if dictionary.len() == max_dictionary_size {
            *dictionary.get_mut(0).unwrap() = new_dict_entry;
        } else {
            dictionary.push(new_dict_entry);
        }
        output.push(new_entry);
    }
    return output;
}

pub fn lz78_decode<T: Clone + PartialEq>(input: &[LZ78entry<T>]) -> Vec<T> {
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
        let encoded = lz78_encode(&input, 4, 4);
        assert!(encoded.len() < input.len());
        let decoded = lz78_decode(&encoded);
        assert_eq!(input, decoded);
    }
}
