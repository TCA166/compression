use generic_compression::lz::{
    lz77::{LZ77entry, LZ77tuple},
    lz78::{LZ78entry, LZ78tuple},
};
use num_traits::ToBytes;

use std::{
    error,
    io::{self, Write},
};

const U8_MAX: usize = u8::MAX as usize;
const U16_MAX: usize = u16::MAX as usize;
const U32_MAX: usize = u32::MAX as usize;

/// Returns the minimum number of bytes needed to represent a given value.
///
/// ## Arguments
/// - `val` - The value to be represented.
///
/// ## Returns
/// - `u8` - The number of bytes needed to represent the value.
///
/// ## Example
/// ```
/// let bytes_needed = min_size(300);
/// assert_eq!(bytes_needed, 2);
/// ```
fn min_size(val: usize) -> u8 {
    if val <= U8_MAX {
        1
    } else if val <= U16_MAX {
        2
    } else if val <= U32_MAX {
        4
    } else {
        8
    }
}

/// Serializes a `usize` value into a specified number of bytes.
///
/// ## Arguments
/// - `value` - The `usize` value to be serialized.
/// - `state` - The output stream to write the serialized data.
/// - `num_bytes` - The number of bytes to serialize the value into.
///
/// ## Returns
/// - `io::Result<()>` - Indicates success or failure of the operation.
///
/// ## Example
/// ```
/// let mut buffer = Vec::new();
/// serialize_usize(42, &mut buffer, 1).unwrap();
/// assert_eq!(buffer, vec![42]);
/// ```
fn serialize_usize<W: Write>(value: usize, state: &mut W, num_bytes: u8) -> io::Result<()> {
    match num_bytes {
        1 => {
            state.write_all(&[value as u8])?;
        }
        2 => {
            state.write_all(&(value as u16).to_le_bytes())?;
        }
        4 => {
            state.write_all(&(value as u32).to_le_bytes())?;
        }
        8 => {
            state.write_all(&value.to_le_bytes())?;
        }
        _ => unreachable!(),
    }
    Ok(())
}

/// Serializes a vector of LZ77 entries into a specified output stream.
/// Arguments used in compression are necessary, for optimizing integer encoding.
///
/// ## Format
/// - The first eight bytes represent the length of the vector.
/// - The next byte represents the size that the first values in triples will be serialized into.
/// - The next byte represents the size that the second values in triples will be serialized into.
/// - The remaining bytes are the serialized entries, each consisting of three parts:
///     - The first part is the offset into the sliding window.
///     - The second part is the length of the match.
///     - The third part is the value
///
/// ## Arguments
/// - `value` - The vector of LZ77 entries to be serialized.
/// - `window_size` - The size of the sliding window.
/// - `lookahead_buffer_size` - The size of the lookahead buffer.
/// - `state` - The output stream to write the serialized data.
///
/// ## Returns
/// - `Result<(), Box<dyn std::error::Error>>` - Indicates success or failure of the operation.
pub fn serialize_lz77<T: ToBytes, W: Write>(
    value: Vec<LZ77entry<T>>,
    window_size: usize,
    lookahead_buffer_size: usize,
    state: &mut W,
) -> Result<(), Box<dyn error::Error>> {
    serialize_usize(value.len(), state, 8)?;
    let window_size_bytes = min_size(window_size);
    state.write(&[window_size_bytes])?;
    let lookahead_buffer_size_bytes = min_size(lookahead_buffer_size);
    state.write(&[lookahead_buffer_size_bytes])?;
    for entry in value {
        let tp: LZ77tuple<T> = entry.into();
        serialize_usize(tp.0, state, window_size_bytes)?;
        serialize_usize(tp.1, state, lookahead_buffer_size_bytes)?;
        let bytes = tp.2.to_le_bytes();
        state.write_all(bytes.as_ref())?;
    }
    Ok(())
}

/// Serializes a vector of LZ78 entries into a specified output stream.
/// Arguments used in compression are necessary, for optimizing integer encoding.
///
/// ## Format
/// - The first eight bytes represent the length of the vector.
/// - The next byte represents the size that the first values in pairs will be serialized into.
/// - The following bytes represent the serialized entries, each consisting of two parts:
///    - The first part is the index into the dictionary.
///   - The second part is the value.
///
/// ## Arguments
/// - `value` - The vector of LZ78 entries to be serialized.
/// - `dictionary_size` - The size of the dictionary.
/// - `state` - The output stream to write the serialized data.
///
/// ## Returns
/// - `Result<(), Box<dyn std::error::Error>>` - Indicates success or failure of the operation.
pub fn serialize_lz78<T: ToBytes, W: Write>(
    value: Vec<LZ78entry<T>>,
    dictionary_size: usize,
    state: &mut W,
) -> Result<(), Box<dyn error::Error>> {
    serialize_usize(value.len(), state, 8)?;
    let dictionary_size_bytes = min_size(dictionary_size);
    state.write(&[dictionary_size_bytes])?;
    for entry in value {
        let tp: LZ78tuple<T> = entry.into();
        if let Some(idx) = tp.0 {
            serialize_usize(idx + 1, state, dictionary_size_bytes)?;
        } else {
            serialize_usize(0, state, dictionary_size_bytes)?; // 0 for None
        }
        let bytes = tp.1.to_le_bytes();
        state.write_all(bytes.as_ref())?;
    }
    Ok(())
}

/// Serializes a vector of LZW entries into a specified output stream.
///
/// ## Format
/// - The first eight bytes represent the length of the vector.
/// - The next byte represents the size that the values will be serialized into.
/// - The remaining bytes are the serialized entries.
///
/// ## Arguments
/// - `value` - The vector of LZW entries to be serialized.
/// - `state` - The output stream to write the serialized data.
///
/// ## Returns
/// - `Result<(), Box<dyn std::error::Error>>` - Indicates success or failure of the operation.
pub fn serialize_lzw<W: Write>(
    value: Vec<usize>,
    state: &mut W,
) -> Result<(), Box<dyn error::Error>> {
    serialize_usize(value.len(), state, 8)?;
    let width = min_size(value.iter().copied().max().unwrap_or(0));
    state.write(&[width])?;
    for entry in value {
        serialize_usize(entry, state, width)?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serialize_usize() {
        let mut buffer = Vec::new();
        serialize_usize(42, &mut buffer, 1).unwrap();
        assert_eq!(buffer, vec![42]);

        buffer.clear();
        serialize_usize(300, &mut buffer, 2).unwrap();
        assert_eq!(buffer, vec![44, 1]);

        buffer.clear();
        serialize_usize(70000, &mut buffer, 4).unwrap();
        assert_eq!(buffer, vec![112, 17, 1, 0]);

        buffer.clear();
        serialize_usize(7000000000, &mut buffer, 8).unwrap();
        assert_eq!(buffer, vec![0, 134, 59, 161, 1, 0, 0, 0]);
    }
}
