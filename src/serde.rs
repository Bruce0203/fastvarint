use std::marker::PhantomData;

use serde::{
    de::{Error, SeqAccess, Visitor},
    Serializer,
};

use crate::impls::{DecodeVarInt, DecodeVarIntError, EncodeVarInt};

pub fn serialize<T, S>(t: &T, serializer: S) -> Result<S::Ok, S::Error>
where
    T: EncodeVarInt,
    S: Serializer,
{
    t.encode_var_int(|v| serializer.serialize_bytes(v))
}

pub fn deserialize<'de, D, T>(deserializer: D) -> Result<T, D::Error>
where
    T: DecodeVarInt,
    D: serde::Deserializer<'de>,
{
    deserializer.deserialize_seq(VarIntVisitor(PhantomData))
}

struct VarIntVisitor<T: DecodeVarInt>(PhantomData<T>);

impl<'de, T: DecodeVarInt> Visitor<'de> for VarIntVisitor<T> {
    type Value = T;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("failed to read vaarint")
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: SeqAccess<'de>,
    {
        match DecodeVarInt::decode_var_int(|_ind| seq.next_element()) {
            Ok(v) => Ok(v),
            Err(DecodeVarIntError::NotEnoughBytesInTheBuffer | DecodeVarIntError::TooLarge) => {
                return Err(Error::custom("fail to read varint"));
            }
            Err(DecodeVarIntError::Custom(err)) => {
                return Err(err);
            }
        }
    }
}
