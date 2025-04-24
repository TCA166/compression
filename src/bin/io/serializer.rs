use compress_lib::{LZ77entry, LZ77tuple, LZ78entry, LZ78tuple};
use num_traits::ToBytes;

use std::{
    cmp::max,
    io::{self, Write},
};

const U8_MAX: usize = u8::MAX as usize;
const U16_MAX: usize = u16::MAX as usize;
const U32_MAX: usize = u32::MAX as usize;

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
        _ => {
            unreachable!()
        }
    }
    Ok(())
}

pub fn serialize_lz77<const N: usize, T: ToBytes<Bytes = [u8; N]>, W: Write>(
    value: Vec<LZ77entry<T>>,
    state: &mut W,
) -> Result<(), Box<dyn std::error::Error>> {
    let tot_len = value.len();
    serialize_usize(tot_len, state, min_size(tot_len))?;
    for entry in value {
        let tp: LZ77tuple<T> = entry.into();
        // TODO huge compression losses due to number encoding
        let width = min_size(max(tp.0, tp.1));
        state.write(&[width])?;
        serialize_usize(tp.0, state, width)?;
        serialize_usize(tp.1, state, width)?;
        state.write_all(&tp.2.to_le_bytes())?;
    }
    Ok(())
}

pub fn serialize_lz78<const N: usize, T: ToBytes<Bytes = [u8; N]>, W: Write>(
    value: Vec<LZ78entry<T>>,
    state: &mut W,
) -> Result<(), Box<dyn std::error::Error>> {
    // TODO huge compression losses due to number encoding, especially here
    let tot_len = value.len();
    serialize_usize(tot_len, state, min_size(tot_len))?;
    for entry in value {
        let tp: LZ78tuple<T> = entry.into();
        if let Some(idx) = tp.0 {
            let width = min_size(idx);
            state.write(&[width])?;
            serialize_usize(idx, state, width)?;
        } else {
            state.write(&[0])?; // 0 for None
        }
        state.write_all(&tp.1.to_le_bytes())?;
    }
    Ok(())
}

pub fn serialize_lzw<W: Write>(
    value: Vec<usize>,
    state: &mut W,
) -> Result<(), Box<dyn std::error::Error>> {
    let tot_len = value.len();
    serialize_usize(tot_len, state, min_size(tot_len))?;
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
