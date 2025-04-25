use compress_lib::{LZ77entry, LZ77tuple, LZ78entry, LZ78tuple};
use num_traits::FromBytes;

use std::{error, io::Read};

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

fn deserialize_byte<R: Read>(state: &mut R) -> Result<u8, Box<dyn error::Error>> {
    let mut buffer = [0; 1];
    state.read_exact(&mut buffer)?;
    Ok(buffer[0])
}

pub fn deserialize_lz77<R: Read, T: FromBytes<Bytes = [u8]> + Sized>(
    state: &mut R,
) -> Result<Vec<LZ77entry<T>>, Box<dyn error::Error>> {
    let len = deserialize_usize(state, 8)?;
    let mut result = Vec::with_capacity(len);
    let window_size = deserialize_byte(state)?;
    let lookahead_size = deserialize_byte(state)?;
    for _ in 0..len {
        let offset = deserialize_usize(state, window_size)?;
        let length = deserialize_usize(state, lookahead_size)?;
        let mut buffer = vec![0; size_of::<T>()];
        state.read_exact(&mut buffer)?;
        let value = T::from_le_bytes(buffer.as_slice());
        result.push(LZ77entry::from((offset, length, value)));
    }
    return Ok(result);
}
