use generic_compression::{LZ77entry, LZ78entry};
use num_traits::FromBytes;

use std::{error, io::Read};

/// Deserializes a `usize` value from a specified number of bytes.
///
/// ## Arguments
/// - `state` - The input stream to read the serialized data from.
/// - `num_bytes` - The number of bytes to deserialize the value from.
///
/// ## Returns
/// - `Result<usize, Box<dyn error::Error>>` - The deserialized `usize` value or an error.
fn deserialize_usize<R: Read>(
    state: &mut R,
    num_bytes: u8,
) -> Result<usize, Box<dyn error::Error>> {
    let mut buffer = vec![0; num_bytes as usize];
    state.read_exact(&mut buffer)?;
    Ok(match num_bytes {
        1 => buffer[0] as usize,
        2 => u16::from_le_bytes(buffer.as_slice().try_into()?) as usize,
        4 => u32::from_le_bytes(buffer.as_slice().try_into()?) as usize,
        _ => u64::from_le_bytes(buffer.as_slice().try_into()?) as usize,
    })
}

/// Deserializes a single byte from the input stream.
///
/// ## Arguments
/// - `state` - The input stream to read the byte from.
///
/// ## Returns
/// - `Result<u8, Box<dyn error::Error>>` - The deserialized byte value or an error.
fn deserialize_byte<R: Read>(state: &mut R) -> Result<u8, Box<dyn error::Error>> {
    let mut buffer = [0; 1];
    state.read_exact(&mut buffer)?;
    Ok(buffer[0])
}

/// Deserializes a vector of `LZ77entry` values from the input stream.
///
/// ## Arguments
/// - `state` - The input stream to read the serialized data from.
///
/// ## Returns
/// - `Result<Vec<LZ77entry<T>>, Box<dyn error::Error>>` - The deserialized vector of `LZ77entry` values or an error.
pub fn deserialize_lz77<R: Read, const N: usize, T: FromBytes<Bytes = [u8; N]>>(
    state: &mut R,
) -> Result<Vec<LZ77entry<T>>, Box<dyn error::Error>> {
    let len = deserialize_usize(state, 8)?;
    let mut result = Vec::with_capacity(len);
    let window_size = deserialize_byte(state)?;
    let lookahead_size = deserialize_byte(state)?;
    for _ in 0..len {
        let offset = deserialize_usize(state, window_size)?;
        let length = deserialize_usize(state, lookahead_size)?;
        let mut buffer = [0; N];
        state.read_exact(&mut buffer)?;
        let value = T::from_le_bytes(&buffer);
        result.push(LZ77entry::from((offset, length, value)));
    }
    return Ok(result);
}

/// Deserializes a vector of `LZ78entry` values from the input stream.
///
/// ## Arguments
/// - `state` - The input stream to read the serialized data from.
///
/// ## Returns
/// - `Result<Vec<LZ78entry<T>>, Box<dyn error::Error>>` - The deserialized vector of `LZ78entry` values or an error.
pub fn deserialize_lz78<R: Read, const N: usize, T: FromBytes<Bytes = [u8; N]>>(
    state: &mut R,
) -> Result<Vec<LZ78entry<T>>, Box<dyn error::Error>> {
    let len = deserialize_usize(state, 8)?;
    let mut result = Vec::with_capacity(len);
    let dict_width = deserialize_byte(state)?;
    for _ in 0..len {
        let index = deserialize_usize(state, dict_width)?;
        let index = if index == 0 { None } else { Some(index - 1) };
        let mut buffer = [0; N];
        state.read_exact(&mut buffer)?;
        let value = T::from_le_bytes(&buffer);
        result.push(LZ78entry::from((index, value)));
    }
    return Ok(result);
}

/// Deserializes a vector of `usize` values from the input stream.
///
/// ## Arguments
/// - `state` - The input stream to read the serialized data from.
///
/// ## Returns
/// - `Result<Vec<usize>, Box<dyn error::Error>>` - The deserialized vector of `usize` values or an error.
pub fn deserialize_lzw<R: Read>(state: &mut R) -> Result<Vec<usize>, Box<dyn error::Error>> {
    let len = deserialize_usize(state, 8)?;
    let mut result = Vec::with_capacity(len);
    let width = deserialize_byte(state)?;
    for _ in 0..len {
        let value = deserialize_usize(state, width)?;
        result.push(value);
    }
    return Ok(result);
}
