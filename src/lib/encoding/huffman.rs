use std::ops::Deref;

use bits_io::{bit_types::BitVec, bitvec};
use num::Integer;

#[derive(Clone, PartialEq, Eq)]
struct HeapValue<T: Clone + Eq, W: Integer + Clone> {
    value: T,
    frequency: W,
}

impl<T: Clone + Eq, W: Integer + Clone> PartialOrd for HeapValue<T, W> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<T: Clone + Eq, W: Integer + Clone> Ord for HeapValue<T, W> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.frequency.cmp(&other.frequency)
    }
}

/// A tree structure for the huffman encoding.
/// Under the hood, it is a binary heap.
pub struct HuffmanEncoding<T: Clone + Eq, W: Integer + Clone> {
    root: Vec<HeapValue<T, W>>,
}

#[inline(always)]
fn left_child_index(index: usize) -> usize {
    index * 2 + 1
}

#[inline(always)]
fn right_child_index(index: usize) -> usize {
    index * 2 + 2
}

impl<T: Clone + Eq, W: Integer + Clone> HuffmanEncoding<T, W> {
    /// Creates a new empty HuffmanEncoding
    pub fn new() -> Self {
        HuffmanEncoding { root: Vec::new() }
    }

    /// Creates a new HuffmanEncoding with the given weights
    ///
    /// ## Arguments
    ///
    /// - `weights`: A slice of tuples containing the value and its frequency.
    ///
    /// ## Returns
    ///
    /// A new HuffmanEncoding instance.
    pub fn with_weights(weights: &[(T, W)]) -> Self {
        let mut heap = weights
            .iter()
            .map(|(value, frequency)| HeapValue {
                value: value.clone(),
                frequency: frequency.clone(),
            })
            .collect::<Vec<_>>();
        heap.sort_by(|a, b| b.cmp(a));
        HuffmanEncoding { root: heap }
    }

    fn encode_and_index(&self, value: &T) -> Option<(BitVec, usize)> {
        let mut stack = vec![(0, bitvec![0; 1]), (1, bitvec!(1; 1))];
        while let Some((index, bits)) = stack.pop() {
            if index < self.root.len() {
                if self.root[index].value == *value {
                    return Some((bits, index));
                } else {
                    let mut left = bits.clone();
                    left.push(false);
                    stack.push((left_child_index(index), left));
                    let mut right = bits.clone();
                    right.push(true);
                    stack.push((right_child_index(index), right));
                }
            }
        }
        None
    }

    fn reorder_index(&mut self, index: usize) {
        let mut move_to_ptr = index;
        while move_to_ptr > 0 {
            let parent_index = (move_to_ptr - 1) / 2;
            if self.root[move_to_ptr] > self.root[parent_index] {
                self.root.swap(move_to_ptr, parent_index);
                move_to_ptr = parent_index;
            } else {
                break;
            }
        }
    }

    /// Encodes a value into a bit vector
    ///
    /// ## Arguments
    ///
    /// - `value`: The value to be encoded.
    ///
    /// ## Returns
    ///
    /// A BitVec representing the encoded value.
    ///
    /// ## Example
    ///
    /// ```
    /// use generic_compression::encoding::HuffmanEncoding;
    /// use bits_io::bits;
    /// let codec = HuffmanEncoding::with_weights(&[(b'a', 5), (b'b', 9)]);
    /// let encoded = codec.encode_value(&b'a').unwrap();
    /// assert_eq!(encoded.as_bitslice(), bits![1]);
    /// let encoded = codec.encode_value(&b'b').unwrap();
    /// assert_eq!(encoded.as_bitslice(), bits![0]);
    /// ```
    pub fn encode_value(&self, value: &T) -> Option<BitVec> {
        if let Some((bits, _)) = self.encode_and_index(value) {
            Some(bits)
        } else {
            None
        }
    }

    /// Encodes a value into a bit vector, ensuring the value is present in the heap
    /// Also increases the frequency of the value in the heap, ensuring future encodings are better
    ///
    /// ## Arguments
    ///
    /// - `value`: The value to be encoded.
    ///
    /// ## Returns
    ///
    /// A BitVec representing the encoded value.
    ///
    /// ## Example
    ///
    /// ```
    /// use generic_compression::encoding::HuffmanEncoding;
    /// use bits_io::bits;
    /// let mut codec = HuffmanEncoding::with_weights(&[(b'a', 5), (b'b', 5), (b'c', 5)]);
    /// assert_eq!(codec.encode_value_mut(&b'c').as_bitslice(), bits![0, 1]);
    /// assert_eq!(codec.encode_value_mut(&b'c').as_bitslice(), bits![0]);
    /// ```
    pub fn encode_value_mut(&mut self, value: &T) -> BitVec {
        if let Some((bits, index)) = self.encode_and_index(value) {
            // Increase the frequency of the value in the heap
            self.root[index].frequency = self.root[index].frequency.clone() + W::one();
            // Reorder the heap to maintain the heap property
            self.reorder_index(index);
            // Return the bits
            return bits;
        } else {
            // if the value is not found, we need to add it to the heap
            let new_value = HeapValue {
                value: value.clone(),
                frequency: W::one(),
            };
            self.root.push(new_value);
            return self.encode_value(value).unwrap();
        }
    }

    fn decode_index<B: Deref<Target = bool>, I: Iterator<Item = B>>(
        &self,
        input: I,
    ) -> Option<usize> {
        let mut input = input.into_iter();
        let mut index = if let Some(index) = input.next() {
            if *index { 1 } else { 0 }
        } else {
            return None;
        };
        for bit in input {
            index = if *bit {
                right_child_index(index)
            } else {
                left_child_index(index)
            };
            if index >= self.root.len() {
                return None;
            }
        }
        Some(index)
    }

    /// Decodes a bit vector into a value
    ///
    /// ## Arguments
    ///
    /// - `input`: An iterator over bits representing the encoded value.
    ///
    /// ## Returns
    ///
    /// A value of type T if the decoding is successful, otherwise None.
    ///
    /// ## Example
    ///
    /// ```
    /// use generic_compression::encoding::HuffmanEncoding;
    /// use bits_io::bits;
    /// let codec = HuffmanEncoding::with_weights(&[(b'a', 5), (b'b', 9)]);
    /// let encoded = codec.encode_value(&b'a').unwrap();
    /// let decoded = codec.decode_value(encoded.as_bitslice().iter()).unwrap();
    /// assert_eq!(decoded, b'a');
    /// ```
    pub fn decode_value<B: Deref<Target = bool>, I: Iterator<Item = B>>(
        &self,
        input: I,
    ) -> Option<T> {
        if let Some(index) = self.decode_index(input) {
            return Some(self.root[index].value.clone());
        } else {
            return None;
        }
    }

    /// Decodes a bit vector into a value
    /// Also increases the frequency of the value in the heap, ensuring future encodings are better
    ///
    /// ## Arguments
    ///
    /// - `input`: An iterator over bits representing the encoded value.
    ///
    /// ## Returns
    ///
    /// A value of type T if the decoding is successful, otherwise None.
    pub fn decode_value_mut<B: Deref<Target = bool>, I: Iterator<Item = B>>(
        &mut self,
        input: I,
    ) -> Option<T> {
        if let Some(index) = self.decode_index(input) {
            // Increase the frequency of the value in the heap
            self.root[index].frequency = self.root[index].frequency.clone() + W::one();
            // Reorder the heap to maintain the heap property
            self.reorder_index(index);
            return Some(self.root[index].value.clone());
        } else {
            return None;
        }
    }
}

#[cfg(test)]
mod tests {
    use bits_io::bits;

    use super::*;

    #[test]
    fn test_huffman_encoding() {
        let weights = [
            ('a', 5),
            ('b', 9),
            ('c', 12),
            ('d', 13),
            ('e', 16),
            ('f', 45),
        ];
        let huffman = HuffmanEncoding::with_weights(&weights);
        let mut len = huffman.encode_value(&'f').unwrap().as_bitslice().len();
        // assert that the length of the encoding is non-decreasing with decreasing frequency
        for (value, _) in weights.iter().rev() {
            let new_len = huffman.encode_value(value).unwrap().as_bitslice().len();
            assert!(new_len >= len);
            len = new_len;
        }
        assert_eq!(huffman.encode_value(&'f').unwrap().as_bitslice(), bits![0]);
        assert_eq!(
            huffman.encode_value(&'a').unwrap().as_bitslice(),
            bits![0, 1, 0]
        );
        assert_eq!(
            huffman.encode_value(&'b').unwrap().as_bitslice(),
            bits![1, 1]
        );
    }

    #[test]
    fn test_huffman_decoding() {
        let weights = [
            ('a', 5),
            ('b', 9),
            ('c', 12),
            ('d', 13),
            ('e', 16),
            ('f', 45),
        ];
        let huffman = HuffmanEncoding::with_weights(&weights);
        let encoded = huffman.encode_value(&'a').unwrap();
        let decoded = huffman.decode_value(encoded.as_bitslice().iter()).unwrap();
        assert_eq!(decoded, 'a');
    }

    #[test]
    fn test_dynamic_huffman() {
        let mut huffman: HuffmanEncoding<char, u16> = HuffmanEncoding::new();
        let weights = [
            ('a', 5),
            ('b', 9),
            ('c', 12),
            ('d', 13),
            ('e', 16),
            ('f', 45),
        ];
        let mut encoding = Vec::new();
        for (value, _) in weights.iter() {
            encoding.push(huffman.encode_value_mut(value));
        }
        for (encoded, (value, _)) in encoding.iter().zip(weights.iter()) {
            let decoded = huffman
                .decode_value_mut(encoded.as_bitslice().iter())
                .unwrap();
            assert_eq!(decoded, *value);
        }
    }
}
