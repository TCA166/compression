use std::ops::Deref;

use bits_io::bit_types::BitVec;
use num::Integer;

#[derive(Clone)]
enum NodeInternal<T, W: Integer + Clone> {
    Value((T, W)),
    Pointer(Box<Node<T, W>>),
}

impl<T, W: Integer + Clone> NodeInternal<T, W> {
    fn frequency(&self) -> W {
        match self {
            NodeInternal::Value((_, frequency)) => frequency.clone(),
            NodeInternal::Pointer(node) => node.frequency.clone(),
        }
    }
}

#[derive(Clone)]
struct Node<T, W: Integer + Clone> {
    frequency: W,
    left: NodeInternal<T, W>,
    right: Option<NodeInternal<T, W>>,
}

/// A tree structure for the huffman encoding
pub struct HuffmanEncoding<T: Clone + PartialEq, W: Integer + Clone> {
    root: Option<Box<Node<T, W>>>,
}

impl<T: Clone + PartialEq, W: Integer + Clone> HuffmanEncoding<T, W> {
    /// Creates a new empty HuffmanEncoding
    pub fn new() -> Self {
        HuffmanEncoding { root: None }
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
        // sort the weights
        let mut sorted_weights = weights.to_vec();
        sorted_weights.sort_by(|a, b| a.1.cmp(&b.1));
        // create the huffman tree
        let mut nodes: Vec<NodeInternal<T, W>> = sorted_weights
            .iter()
            .map(|(value, frequency)| NodeInternal::Value((value.clone(), frequency.clone())))
            .collect();
        while nodes.len() > 2 {
            // pop the two smallest nodes
            let left = nodes.remove(0);
            let right = nodes.remove(0);
            // create a new node with the sum of the frequencies
            let new_node = Node {
                frequency: left.frequency() + right.frequency(),
                left: left.clone(),
                right: Some(right.clone()),
            };
            // insert the new node back into the list
            nodes.push(NodeInternal::Pointer(new_node.into()));
            // sort the list again
            nodes.sort_by(|a, b| a.frequency().cmp(&b.frequency()));
        }
        // pop the last two nodes
        let left = nodes.remove(0);
        let right = nodes.remove(0);
        // create the root node
        let root = Node {
            frequency: left.frequency() + right.frequency(),
            left: left.clone(),
            right: Some(right.clone()),
        };
        HuffmanEncoding {
            root: Some(Box::new(root)),
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
    /// use compress_lib::HuffmanEncoding;
    /// use bits_io::bits;
    /// let codec = HuffmanEncoding::with_weights(&[(b'a', 5), (b'b', 9)]);
    /// let encoded = codec.encode_value(&b'a').unwrap();
    /// assert_eq!(encoded.as_bitslice(), bits![0]);
    /// let encoded = codec.encode_value(&b'b').unwrap();
    /// assert_eq!(encoded.as_bitslice(), bits![1]);
    /// ```
    pub fn encode_value(&self, value: &T) -> Option<BitVec> {
        if let Some(root) = &self.root {
            let mut stack = vec![(root, BitVec::new())];
            while let Some((node, mut encoding)) = stack.pop() {
                match &node.left {
                    NodeInternal::Value((v, _)) => {
                        if v == value {
                            encoding.push(false);
                            return Some(encoding);
                        }
                    }
                    NodeInternal::Pointer(left_node) => {
                        let mut new_encoding = encoding.clone();
                        new_encoding.push(false);
                        stack.push((left_node, new_encoding));
                    }
                }
                if let Some(ref right) = node.right {
                    encoding.push(true);
                    match right {
                        NodeInternal::Value((v, _)) => {
                            if v == value {
                                return Some(encoding);
                            }
                        }
                        NodeInternal::Pointer(right_node) => {
                            stack.push((right_node, encoding));
                        }
                    }
                }
            }
            None
        } else {
            return None;
        }
    }

    /*
    /// Encodes a value into a bit vector, while changing the weights and rebalancing the tree
    pub fn encode_value_mut(&mut self, value: &T) -> BitVec {
        if let Some(root) = &mut self.root {
            let mut stack = vec![(root, BitVec::new())];
            let mut out_encoding = None;
            while let Some((node, mut encoding)) = stack.pop() {
                match &mut node.left {
                    NodeInternal::Value((v, weight)) => {
                        if v == value {
                            encoding.push(false);
                            *weight = weight.clone() + W::one();
                            out_encoding = Some(encoding.clone());
                            break;
                        }
                    }
                    NodeInternal::Pointer(left_node) => {
                        let mut new_encoding = encoding.clone();
                        new_encoding.push(false);
                        stack.push((left_node, new_encoding));
                    }
                }
                if let Some(ref mut right) = node.right {
                    encoding.push(true);
                    match right {
                        NodeInternal::Value((v, weight)) => {
                            if v == value {
                                *weight = weight.clone() + W::one();
                                out_encoding = Some(encoding.clone());
                                break;
                            }
                        }
                        NodeInternal::Pointer(right_node) => {
                            stack.push((right_node, encoding));
                        }
                    }
                }
            }
            if let Some(encoding) = out_encoding {
                // increase the weights
                let mut node = root;
                for bit in encoding.iter() {
                    match bit.deref() {
                        false => match &mut node.left {
                            NodeInternal::Value((_, weight)) => *weight = weight.clone() + W::one(),
                            NodeInternal::Pointer(left_node) => node = left_node,
                        },
                        true => match &mut node.right.unwrap() {
                            NodeInternal::Value((_, weight)) => *weight = weight.clone() + W::one(),
                            NodeInternal::Pointer(right_node) => node = right_node,
                        },
                    }
                }
                return encoding;
            } else {
                // find a node to insert the new value and balance the tree
                let mut stack = vec![(root, BitVec::new())];
            }
        } else {
            // create a root node if it doesn't exist
            let new_node = Node {
                frequency: W::one(),
                left: NodeInternal::Value((value.clone(), W::one())),
                right: None,
            };
            self.root = Some(Box::new(new_node));
            // return the encoding for the new node
            let mut encoding = BitVec::new();
            encoding.push(false);
            return encoding;
        }
    }
    */

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
    /// use compress_lib::HuffmanEncoding;
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
        if let Some(root) = &self.root {
            let mut node = root;
            for bit in input {
                match bit.deref() {
                    false => match &node.left {
                        NodeInternal::Value((v, _)) => return Some(v.clone()),
                        NodeInternal::Pointer(left_node) => node = left_node,
                    },
                    true => {
                        if let Some(ref right) = node.right {
                            match right {
                                NodeInternal::Value((v, _)) => return Some(v.clone()),
                                NodeInternal::Pointer(right_node) => node = right_node,
                            }
                        } else {
                            return None;
                        }
                    }
                }
            }
            None
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
            (b'a', 5),
            (b'b', 9),
            (b'c', 12),
            (b'd', 13),
            (b'e', 16),
            (b'f', 45),
        ];
        let huffman = HuffmanEncoding::with_weights(&weights);
        assert_eq!(huffman.encode_value(&b'f').unwrap().as_bitslice(), bits![0]);
        assert_eq!(
            huffman.encode_value(&b'a').unwrap().as_bitslice(),
            bits![1, 1, 0, 0]
        );
    }

    #[test]
    fn test_huffman_decoding() {
        let weights = [
            (b'a', 5),
            (b'b', 9),
            (b'c', 12),
            (b'd', 13),
            (b'e', 16),
            (b'f', 45),
        ];
        let huffman = HuffmanEncoding::with_weights(&weights);
        let encoded = huffman.encode_value(&b'a').unwrap();
        let decoded = huffman.decode_value(encoded.as_bitslice().iter()).unwrap();
        assert_eq!(decoded, b'a');
    }
}
