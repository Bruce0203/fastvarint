use derive_more::derive::{Deref, DerefMut, Display, Into};
use nonmax::NonMaxI32;
use num_traits::AsPrimitive;

use crate::{DecodeVarInt, EncodeVarInt};

#[derive(
    Default, Debug, Display, Into, Deref, DerefMut, Clone, Copy, PartialEq, Eq, PartialOrd, Ord,
)]
pub struct NonMaxI32VarInt(NonMaxI32);

impl NonMaxI32VarInt {
    pub fn new(value: i32) -> Self {
        Self(unsafe { NonMaxI32::new_unchecked(value) })
    }
}

impl<T: AsPrimitive<i32>> From<T> for NonMaxI32VarInt {
    fn from(value: T) -> Self {
        Self::new(value.as_())
    }
}

impl EncodeVarInt for NonMaxI32VarInt {
    fn encode_var_int<F: FnOnce(&[u8]) -> R, R>(&self, write: F) -> R {
        self.0.get().encode_var_int(write)
    }
}

impl DecodeVarInt for NonMaxI32VarInt {
    fn decode_var_int<F: FnMut(usize) -> Result<Option<u8>, E>, E: std::fmt::Display>(
        mut reader: F,
    ) -> Result<Self, crate::DecodeVarIntError<E>> {
        i32::decode_var_int::<_, E>(|i| reader(i)).map(|v| NonMaxI32VarInt::new(v))
    }
}
