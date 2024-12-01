use derive_more::derive::{Deref, DerefMut, Display, Into};
use nonmax::NonMaxI32;
use num_traits::AsPrimitive;

use crate::EncodeVarInt;

#[derive(
    Default, Debug, Display, Into, Deref, DerefMut, Clone, Copy, PartialEq, Eq, PartialOrd, Ord,
)]
pub struct NonMaxI32VarInt(pub NonMaxI32);

impl NonMaxI32VarInt {
    pub const fn new(value: i32) -> Self {
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

#[cfg(feature = "serde")]
impl serde::Serialize for NonMaxI32VarInt {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        crate::serialize(&self.0.get(), serializer)
    }
}
