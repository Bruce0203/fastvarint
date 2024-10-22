use std::{convert::Infallible, marker::PhantomData, usize};

use serde::{
    de::{Unexpected, Visitor},
    Deserialize, Serialize,
};

use crate::{DecodeVarInt, DecodeVarIntError, EncodeVarInt, VarInt};

impl Serialize for VarInt {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serialize(&**self, serializer)
    }
}

pub fn serialize<T: EncodeVarInt, S>(value: &T, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    value.encode_var_int(|bytes| serializer.serialize_bytes(bytes))
}

impl<'de> Deserialize<'de> for VarInt {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserialize(deserializer)
    }
}

fn deserialize<'de, T: DecodeVarInt, D>(deserializer: D) -> Result<T, D::Error>
where
    D: serde::Deserializer<'de>,
{
    deserializer.deserialize_bytes(VarIntVisitor::<T>(PhantomData))
}

struct VarIntVisitor<T: DecodeVarInt>(PhantomData<T>);

impl<'de, T: DecodeVarInt> Visitor<'de> for VarIntVisitor<T> {
    type Value = T;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("failed to decode varint")
    }

    fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        T::decode_var_int(|i| -> Result<_, Infallible> { Ok(v.get(i).copied()) }).map_err(|err| {
            match err {
                DecodeVarIntError::NotEnoughBytesInTheBuffer => {
                    E::invalid_length(usize::MAX, &"NotEnoughBytesInTheBuffer")
                }
                DecodeVarIntError::TooLarge => {
                    E::invalid_value(Unexpected::Unsigned(1), &"TooLarge")
                }
                DecodeVarIntError::Custom(_) => E::custom(""),
            }
        })
    }
}
