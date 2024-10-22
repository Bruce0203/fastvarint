use derive_more::derive::{Deref, DerefMut, Display, Into};
use num_traits::AsPrimitive;

#[derive(
    Default, Debug, Display, Into, Deref, DerefMut, Clone, Copy, PartialEq, Eq, PartialOrd, Ord,
)]
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

#[cfg(feature = "serde")]
impl serde::Serialize for VarInt {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        crate::serialize(&**self, serializer)
    }
}

#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for VarInt {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        crate::deserialize(deserializer)
    }
}
