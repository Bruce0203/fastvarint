use std::convert::Infallible;

use num_traits::AsPrimitive;

use crate::{DecodeVarInt, DecodeVarIntError, EncodeVarInt};

fn verify_encode_var_int<T: EncodeVarInt + AsPrimitive<u64> + Copy>(value: T, expected: &[u8]) {
    let mut encoded = Vec::new();
    value.encode_var_int(|bytes| {
        encoded.extend_from_slice(bytes);
    });
    assert_eq!(encoded, expected);
}

fn verify_decode_var_int<T: DecodeVarInt + PartialEq + std::fmt::Debug>(expected: T, bytes: &[u8]) {
    let mut i = 0;
    let result: Result<(T, usize), DecodeVarIntError<Infallible>> = T::decode_var_int(|_| {
        if i < bytes.len() {
            let byte = bytes[i];
            i += 1;
            Ok(Some(byte))
        } else {
            Ok(None)
        }
    });
    assert_eq!(result.unwrap().0, expected);
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
