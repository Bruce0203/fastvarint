use derive_more::derive::{Deref, DerefMut, Display, Into};
use nonmax::NonMaxI32;
use num_traits::AsPrimitive;

#[derive(
    Default, Debug, Display, Into, Deref, DerefMut, Clone, Copy, PartialEq, Eq, PartialOrd, Ord,
)]
pub struct NonMaxVarInt(NonMaxI32);

impl NonMaxVarInt {
    pub fn new(value: i32) -> Self {
        Self(unsafe { NonMaxI32::new_unchecked(value) })
    }
}

impl<T: AsPrimitive<i32>> From<T> for NonMaxVarInt {
    fn from(value: T) -> Self {
        Self::new(value.as_())
    }
}
