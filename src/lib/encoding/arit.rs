use num::{Integer, One, Unsigned, Zero, rational::Ratio};
use num_traits::{NumAssignOps, NumOps};

use std::{collections::HashMap, hash::Hash, iter::Sum};

fn weights_to_ranges<T: Hash + Eq, U: Integer + Clone + NumOps + NumAssignOps + Sum>(
    weights: &[(T, U)],
) -> HashMap<&T, (Ratio<U>, Ratio<U>)> {
    let mut ranges = HashMap::with_capacity(weights.len());
    let sum = weights.iter().map(|(_, weight)| weight.clone()).sum::<U>();
    let mut total_weight: Ratio<U> = Ratio::zero();
    for (key, weight) in weights.iter() {
        let l_weight = total_weight.clone();
        total_weight += Ratio::new(weight.clone(), sum.clone());
        ranges.insert(key, (l_weight, total_weight.clone()));
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
/// use generic_compression::encoding::arit::arithmetic_encode;
/// use num::rational::Ratio;
///
/// let input = vec![0, 1, 0, 1];
/// let weights = [(0, 1), (1, 3)];
///
/// let encoded = arithmetic_encode(&input, &weights);
/// assert_eq!(encoded, Ratio::<u32>::new(47, 512));
/// ```
pub fn arithmetic_encode<
    T: Hash + Eq,
    U: Unsigned + Integer + Clone + NumOps + NumAssignOps + Sum,
>(
    input: &[T],
    weights: &[(T, U)],
) -> Ratio<U> {
    let ranges = weights_to_ranges(weights);
    let mut l = Ratio::zero();
    let mut r = Ratio::one();
    for symbol in input {
        let (l_weight, r_weight) = ranges.get(symbol).unwrap();
        let range = r - l.clone();
        r = l.clone() + range.clone() * r_weight;
        l = l + range * l_weight;
    }

    return (r + l) / (U::one() + U::one());
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
/// use generic_compression::encoding::arit::{arithmetic_decode, arithmetic_encode};
/// use num::Rational32;
///
/// let input = vec![0, 1, 0, 1];
/// let weights: &[(u8, u32)] = &[(0, 1), (1, 3)];
/// let encoded = arithmetic_encode(&input, weights);
/// let decoded = arithmetic_decode(encoded, weights, input.len());
/// assert_eq!(decoded, input);
///
pub fn arithmetic_decode<
    T: Hash + Eq + Clone,
    U: Unsigned + Integer + Clone + NumOps + NumAssignOps + Sum,
>(
    input: Ratio<U>,
    weights: &[(T, U)],
    length: usize,
) -> Vec<T> {
    let ranges = weights_to_ranges(weights);
    let mut l = Ratio::zero();
    let mut r = Ratio::one();
    let mut output: Vec<T> = Vec::with_capacity(length);
    for _ in 0..length {
        let d = r.clone() - l.clone();
        let x = (input.clone() - l.clone()) / d.clone();
        for (key, (l_weight, r_weight)) in ranges.iter() {
            if x >= *l_weight && x < *r_weight {
                output.push((*key).clone());
                r = l.clone() + d.clone() * r_weight;
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
        let input = "abcd";
        let weights: &[(u8, u32)] = &[(b'a', 1), (b'b', 1), (b'c', 1), (b'd', 1)];

        let encoded = arithmetic_encode(input.as_ref(), weights);
        assert_eq!(encoded, Ratio::new(55, 512));
    }

    #[test]
    fn test_arithmetic_decode() {
        let input = Ratio::new(55, 512);
        let weights: &[(u8, u32)] = &[(b'a', 1), (b'b', 1), (b'c', 1), (b'd', 1)];
        let length = 4;

        let decoded = arithmetic_decode(input, &weights, length);
        assert_eq!(decoded, b"abcd");
    }
}
