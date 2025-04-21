use num::Integer;

struct Node<T, W: Integer> {
    value: T,
    frequency: W,
    left: Option<Box<Node<T, W>>>,
    right: Option<Box<Node<T, W>>>,
}

impl<T: PartialEq + Clone, W: Integer> Node<T, W> {
    fn encode_value(&self, value: &T, encoding: &mut Vec<bool>) {
        if self.value == *value {
            return;
        }
        if let Some(ref left) = self.left {
            left.encode_value(value, encoding);
            encoding.push(false);
        }
        if let Some(ref right) = self.right {
            right.encode_value(value, encoding);
            encoding.push(true);
        }
    }

    fn decode_value(&self, encoding: &[bool], index: &mut usize) -> Option<T> {
        if *index == encoding.len() {
            return Some(self.value.clone());
        } else if *index > encoding.len() {
            return None;
        }

        if encoding[*index] {
            *index += 1;
            if let Some(ref right) = self.right {
                return right.decode_value(encoding, index);
            }
        } else {
            *index += 1;
            if let Some(ref left) = self.left {
                return left.decode_value(encoding, index);
            }
        }
        None
    }
}

pub struct HuffmanEncoding<T: Clone + PartialEq, W: Integer + Clone> {
    root: Option<Box<Node<T, W>>>,
}

impl<T: Clone + PartialEq, W: Integer + Clone> HuffmanEncoding<T, W> {
    pub fn new() -> Self {
        HuffmanEncoding { root: None }
    }

    pub fn with_weights(weights: &[(T, W)]) -> Self {
        // sort the weights
        let mut sorted_weights = weights.to_vec();
        sorted_weights.sort_by(|a, b| a.1.cmp(&b.1));
        // create the huffman tree
        let mut nodes: Vec<Node<T, W>> = sorted_weights
            .iter()
            .map(|(value, frequency)| Node {
                value: value.clone(),
                frequency: frequency.clone(),
                left: None,
                right: None,
            })
            .collect();
        while nodes.len() > 1 {
            // pop the two smallest nodes
            let left = nodes.remove(0);
            let right = nodes.remove(0);
            // create a new node with the sum of the frequencies
            let new_node = Node {
                value: left.value.clone(),
                frequency: left.frequency.clone() + right.frequency.clone(),
                left: Some(Box::new(left)),
                right: Some(Box::new(right)),
            };
            // insert the new node back into the list
            nodes.push(new_node);
            // sort the list again
            nodes.sort_by(|a, b| a.frequency.cmp(&b.frequency));
        }
        // the last node is the root of the tree
        let root = nodes.pop().unwrap();
        HuffmanEncoding {
            root: Some(Box::new(root)),
        }
    }

    pub fn encode_value(&self, value: &T, encoding: &mut Vec<bool>) {
        if let Some(ref root) = self.root {
            root.encode_value(value, encoding);
        }
    }

    pub fn decode_value(&self, encoding: &[bool]) -> Option<T> {
        if let Some(ref root) = self.root {
            let mut index = 0;
            return root.decode_value(encoding, &mut index);
        }
        None
    }
}

#[cfg(test)]
mod tests {
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
        let mut encoding = Vec::new();
        huffman.encode_value(&b'f', &mut encoding);
        assert_eq!(encoding, vec![]);
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
        let encoding = vec![];
        let decoded = huffman.decode_value(&encoding);
        assert_eq!(decoded, Some(b'f'));
    }
}
