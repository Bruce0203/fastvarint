use derive_more::derive::{Deref, DerefMut};
use num_traits::AsPrimitive;

#[derive(Deref, DerefMut)]
pub struct VarInt(i32);

impl VarInt {
    pub fn new(v: impl AsPrimitive<i32>) -> Self {
        Self(v.as_())
    }
}

impl<T: AsPrimitive<i32>> From<T> for VarInt {
    fn from(value: T) -> Self {
        Self(value.as_())
    }
}
