use core::panic;

use bits_io::{
    bit_types::BitVec,
    bits,
    prelude::{BitRead, BitSlice, Bits},
};
use num_traits::{FromBytes, ToBytes};

/// Encodes a value using Elias gamma encoding.
///
/// ## Arguments
///
/// - `value`: The value to be encoded.
/// - `out`: The output buffer to store the encoded bits.
///
///
/// ## Example
///
/// ```
/// use generic_compression::elias::gamma_encode;
/// use bits_io::{bits, bit_types::BitVec};
///
/// let mut buffer = BitVec::new();
/// gamma_encode(8, &mut buffer);
/// assert_eq!(buffer, bits![0, 0, 0, 1, 0, 0, 0]);
/// ```
pub fn gamma_encode<I: ToBytes<Bytes: Send + 'static>>(value: I, out: &mut BitVec) {
    let bytes = value.to_be_bytes();
    // we get a slice of big endian bytes
    let bits = Bits::from_owner_bytes(bytes);
    if let Some(first_one) = bits.first_one() {
        // write the number of bits in the value
        let num_bits = bits.len() - first_one;
        for _ in 0..num_bits - 1 {
            out.push(false);
        }
        out.extend_from_bitslice(&bits[first_one..]);
    } else {
        panic!("Cannot encode zero");
    }
}

/// Decodes a value using Elias gamma encoding.
///
/// ## Arguments
///
/// - `state`: The input stream to read the encoded bits from.
///
/// ## Returns
///
/// - `Result<I, Box<dyn std::error::Error>>` - The decoded value or an error.
///
/// ## Example
///
/// ```
/// use generic_compression::elias::gamma_decode;
/// use bits_io::{bits, bit_types::BitVec};
///
/// let mut buffer = bits![0, 0, 0, 1, 0, 0, 1];
/// let decoded_value: u32 = gamma_decode(&mut buffer).unwrap();
/// assert_eq!(decoded_value, 9);
/// ```
pub fn gamma_decode<const N: usize, I: FromBytes<Bytes = [u8; N]>, R: BitRead>(
    state: &mut R,
) -> Result<I, Box<dyn std::error::Error>> {
    let mut num_zeros = 0;
    let buff = bits![mut 0; 1];
    loop {
        state.read_bits_exact(buff)?;
        if buff[0] {
            break;
        }
        num_zeros += 1;
    }
    let mut buff = [0u8; N];
    let slice = BitSlice::from_slice_mut(&mut buff);
    let slice_len = slice.len();
    slice.set(slice_len - num_zeros - 1, true);
    state.read_bits_exact(&mut slice[slice_len - num_zeros..slice_len])?;
    Ok(I::from_be_bytes(&buff))
}

#[cfg(test)]
mod tests {
    use super::*;
    use bits_io::bits;

    #[test]
    fn test_gamma_encode() {
        let mut buffer = BitVec::new();
        gamma_encode(42, &mut buffer);
        assert_eq!(buffer, bits![0, 0, 0, 0, 0, 1, 0, 1, 0, 1, 0]);
    }
}
