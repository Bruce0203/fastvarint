use num_traits::AsPrimitive;

pub trait EncodeVarInt {
    fn encode_var_int<F: FnOnce(&[u8]) -> R, R>(&self, write: F) -> R;
}

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

pub trait DecodeVarInt: Sized {
    fn decode_var_int<F: FnMut(usize) -> Result<Option<u8>, E>, E>(
        reader: F,
    ) -> Result<Self, DecodeVarIntError<E>>;
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

#[cfg(test)]
mod test {
    use super::*;

    fn verify_encode_var_int<T: EncodeVarInt + AsPrimitive<u64> + Copy>(value: T, expected: &[u8]) {
        let mut encoded = Vec::new();
        value.encode_var_int(|bytes| {
            encoded.extend_from_slice(bytes);
        });
        assert_eq!(encoded, expected);
    }

    fn verify_decode_var_int<T: DecodeVarInt + PartialEq + std::fmt::Debug>(
        expected: T,
        bytes: &[u8],
    ) {
        let mut i = 0;
        let result: Result<T, DecodeVarIntError<()>> = T::decode_var_int(|_| {
            if i < bytes.len() {
                let byte = bytes[i];
                i += 1;
                Ok(Some(byte))
            } else {
                Ok(None)
            }
        });
        assert_eq!(result.unwrap(), expected);
    }

    #[test]
    fn test_encode_var_int() {
        // Encoding tests
        verify_encode_var_int(0, &[0x00]);
        verify_encode_var_int(1, &[0x01]);
        verify_encode_var_int(2, &[0x02]);
        verify_encode_var_int(127, &[0x7f]);
        verify_encode_var_int(128, &[0x80, 0x01]);
        verify_encode_var_int(255, &[0xff, 0x01]);
        verify_encode_var_int(25565, &[0xdd, 0xc7, 0x01]);
        verify_encode_var_int(2097151, &[0xff, 0xff, 0x7f]);
        verify_encode_var_int(2147483647, &[0xff, 0xff, 0xff, 0xff, 0x07]);
        verify_encode_var_int(-1, &[0xff, 0xff, 0xff, 0xff, 0x0f]);
        verify_encode_var_int(-2147483648, &[0x80, 0x80, 0x80, 0x80, 0x08]);
    }

    #[test]
    fn test_decode_var_int() {
        // Decoding tests
        verify_decode_var_int(0, &[0x00]);
        verify_decode_var_int(1, &[0x01]);
        verify_decode_var_int(2, &[0x02]);
        verify_decode_var_int(127, &[0x7f]);
        verify_decode_var_int(128, &[0x80, 0x01]);
        verify_decode_var_int(255, &[0xff, 0x01]);
        verify_decode_var_int(25565, &[0xdd, 0xc7, 0x01]);
        verify_decode_var_int(2097151, &[0xff, 0xff, 0x7f]);
        verify_decode_var_int(2147483647, &[0xff, 0xff, 0xff, 0xff, 0x07]);
        verify_decode_var_int(-1, &[0xff, 0xff, 0xff, 0xff, 0x0f]);
        verify_decode_var_int(-2147483648, &[0x80, 0x80, 0x80, 0x80, 0x08]);
    }
}
