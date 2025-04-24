use num::rational::Ratio;

type Rational32 = Ratio<u32>;

use std::{collections::HashMap, hash::Hash};

fn weights_to_ranges<T: Hash + Eq>(weights: &[(T, u32)]) -> HashMap<&T, (Rational32, Rational32)> {
    let mut ranges = HashMap::with_capacity(weights.len());
    let sum = weights.iter().map(|(_, weight)| *weight).sum::<u32>();
    let mut total_weight = Rational32::from_integer(0);
    for (key, weight) in weights.iter() {
        let l_weight = total_weight;
        total_weight += Rational32::new(*weight, sum);
        ranges.insert(key, (l_weight, total_weight));
    }
    if total_weight != Rational32::from_integer(1) {
        panic!("Weights do not sum to 1");
    }
    ranges
}

/// Encode a sequence of symbols using arithmetic encoding.
/// The input symbols must be in the range [0, 1).
///
/// ## Arguments
///
/// - `input` - A slice of symbols to be encoded.
/// - `weights` - A map of symbols to their weights.
///
/// ## Returns
///
/// A Rational32 representing the encoded value.
///
/// ## Example
///
/// ```
/// use compress_lib::arithmetic_encode;
/// use num::rational::Ratio;
///
/// let input = vec![0, 1, 0, 1];
/// let weights = [(0, 1), (1, 3)];
///
/// let encoded = arithmetic_encode(&input, &weights);
/// assert_eq!(encoded, Ratio::<u32>::new(47, 512));
/// ```
pub fn arithmetic_encode<T: Hash + Eq>(input: &[T], weights: &[(T, u32)]) -> Rational32 {
    let ranges = weights_to_ranges(weights);
    let mut l = Rational32::from_integer(0);
    let mut r = Rational32::from_integer(1);
    for symbol in input {
        let (l_weight, r_weight) = ranges.get(symbol).unwrap();
        let range = r - l;
        r = l + range * r_weight;
        l = l + range * l_weight;
    }
    return (r + l) / Rational32::from_integer(2);
}

/// Decode a sequence of symbols using arithmetic decoding.
/// The input value must be in the range [0, 1).
///
/// ## Arguments
///
/// - `input` - A Rational32 representing the encoded value.
/// - `weights` - A map of symbols to their weights.
/// - `length` - The length of the output sequence.
///
/// ## Returns
///
/// A vector of symbols representing the decoded sequence.
///
/// ## Example
///
/// ```
/// use compress_lib::{arithmetic_decode, arithmetic_encode};
/// use num::Rational32;
///
/// let input = vec![0, 1, 0, 1];
/// let weights = [(0, 1), (1, 3)];
/// let encoded = arithmetic_encode(&input, &weights);
/// let decoded = arithmetic_decode(encoded, &weights, input.len());
/// assert_eq!(decoded, input);
///
pub fn arithmetic_decode<T: Hash + Eq + Clone>(
    input: Rational32,
    weights: &[(T, u32)],
    length: usize,
) -> Vec<T> {
    let ranges = weights_to_ranges(weights);
    let mut l = Rational32::from_integer(0);
    let mut r = Rational32::from_integer(1);
    let mut output: Vec<T> = Vec::with_capacity(length);
    for _ in 0..length {
        let d = r - l;
        let x = (input - l) / d;
        for (key, (l_weight, r_weight)) in ranges.iter() {
            if x >= *l_weight && x < *r_weight {
                output.push((*key).clone());
                r = l + d * r_weight;
                l = l + d * l_weight;
                break;
            }
        }
    }
    return output;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_arithmetic_encode() {
        let input = b"abcd";
        let weights = [(b'a', 1), (b'b', 1), (b'c', 1), (b'd', 1)];

        let encoded = arithmetic_encode(input, &weights);
        assert_eq!(encoded, Rational32::new(55, 512));
    }

    #[test]
    fn test_arithmetic_decode() {
        let input = Rational32::new(55, 512);
        let weights = [(b'a', 1), (b'b', 1), (b'c', 1), (b'd', 1)];
        let length = 4;

        let decoded = arithmetic_decode(input, &weights, length);
        assert_eq!(decoded, b"abcd");
    }
}
