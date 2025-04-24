use compress_lib::{LZ77entry, LZ77tuple, LZ78entry, LZ78tuple};

use std::io::{self, Write};

fn serialize_usize<W: Write>(value: usize, state: &mut W) -> io::Result<()> {
    state.write_all(&value.to_le_bytes())
}

// FIXME another trait for getting the bin value
pub fn serialize_lz77<T: AsRef<[u8]>, W: Write>(
    value: Vec<LZ77entry<T>>,
    state: &mut W,
) -> Result<(), Box<dyn std::error::Error>> {
    serialize_usize(value.len(), state)?;
    for entry in value {
        let tp: LZ77tuple<T> = entry.into();
        serialize_usize(tp.0, state)?;
        serialize_usize(tp.1, state)?;
        state.write_all(tp.2.as_ref())?;
    }
    Ok(())
}

pub fn serialize_lz78<T: AsRef<[u8]>, W: Write>(
    value: Vec<LZ78entry<T>>,
    state: &mut W,
) -> Result<(), Box<dyn std::error::Error>> {
    serialize_usize(value.len(), state)?;
    for entry in value {
        let tp: LZ78tuple<T> = entry.into();
        let mut tp_len: i8 = 0;
        if let Some(_) = tp.0 {
            tp_len += 1;
        }
        if let Some(_) = tp.1 {
            tp_len += 1;
        }
        state.write_all(&tp_len.to_le_bytes())?;
        if let Some(offset) = tp.0 {
            serialize_usize(offset, state)?;
        }
        if let Some(el) = tp.1 {
            state.write_all(el.as_ref())?;
        }
    }
    Ok(())
}

pub fn serialize_lzw<W: Write>(
    value: Vec<usize>,
    state: &mut W,
) -> Result<(), Box<dyn std::error::Error>> {
    serialize_usize(value.len(), state)?;
    for entry in value {
        serialize_usize(entry, state)?;
    }
    Ok(())
}
