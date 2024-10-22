use num_traits::AsPrimitive;

use crate::{DecodeVarInt, EncodeVarInt};

impl<T: AsPrimitive<u64> + Copy> EncodeVarInt for T {
    fn encode_var_int<F: FnOnce(&[u8]) -> R, R>(&self, write: F) -> R {
        let x: u64 = (*self).as_();
        let stage1 = (x & 0x000000000000007f)
            | ((x & 0x0000000000003f80) << 1)
            | ((x & 0x00000000001fc000) << 2)
            | ((x & 0x000000000fe00000) << 3)
            | ((x & 0x00000000f0000000) << 4);

        let leading = stage1.leading_zeros();

        let unused_bytes = (leading - 1) >> 3;
        let bytes_needed = 8 - unused_bytes;

        // set all but the last MSBs
        let msbs = 0x8080808080808080;
        let msbmask = 0xffffffffffffffff >> (((8 - bytes_needed + 1) << 3) - 1);

        let merged = stage1 | (msbs & msbmask);
        let bytes = merged.to_le_bytes();

        let value = unsafe { bytes.get_unchecked(..bytes_needed as usize) };
        write(value)
    }
}

#[derive(Debug)]
pub enum DecodeVarIntError<T> {
    NotEnoughBytesInTheBuffer,
    TooLarge,
    Custom(T),
}

impl<T: From<i32>> DecodeVarInt for T {
    fn decode_var_int<F: FnMut(usize) -> Result<Option<u8>, E>, E>(
        mut reader: F,
    ) -> Result<Self, DecodeVarIntError<E>> {
        let mut val = 0;
        for i in 0..5 as usize {
            let byte: u8 = reader(i)
                .map_err(|err| DecodeVarIntError::Custom(err))?
                .ok_or_else(|| DecodeVarIntError::NotEnoughBytesInTheBuffer)?;
            val |= (byte as i32 & 0b01111111) << (i * 7);
            if byte & 0b10000000 == 0 {
                return Ok(T::from(val));
            }
        }
        Err(DecodeVarIntError::TooLarge)
    }
}
