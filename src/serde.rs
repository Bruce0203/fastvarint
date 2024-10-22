use serde::Serialize;

use crate::{EncodeVarInt, VarInt};

impl Serialize for VarInt {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serialize(&**self, serializer)
    }
}

fn serialize<T: EncodeVarInt, S>(value: &T, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    value.encode_var_int(|bytes| serializer.serialize_bytes(bytes))
}
