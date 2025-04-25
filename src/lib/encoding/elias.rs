use num::Unsigned;
use std::{
    io::{self, Write},
    ops::{Shl, Shr},
};

/*

pub fn omega_encode<I: Unsigned + Shr<Output = I> + Shl<Output = I>, W: Write>(
    n: I,
    state: &mut W,
) -> io::Result<()> {
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_omega_encode() {
        let mut buffer = Vec::new();
        omega_encode(10, &mut buffer).unwrap();
        assert_eq!(buffer, vec![1, 0, 1, 0, 1, 0, 1]);
    }
}
*/
