use std::fmt::Display;

use crate::DecodeVarIntError;

pub trait EncodeVarInt {
    fn encode_var_int<F: FnOnce(&[u8]) -> R, R>(&self, write: F) -> R;
}

pub trait DecodeVarInt: Sized {
    fn decode_var_int<F: FnMut(usize) -> Result<Option<u8>, E>, E: Display>(
        reader: F,
    ) -> Result<(Self, usize), DecodeVarIntError<E>>;
}
