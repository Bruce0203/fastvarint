use std::{convert::Infallible, marker::PhantomData, usize};

use serde::de::{Unexpected, Visitor};

use crate::{DecodeVarInt, DecodeVarIntError, EncodeVarInt};

pub fn serialize<T: EncodeVarInt, S>(value: &T, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    value.encode_var_int(|bytes| serializer.serialize_bytes(bytes))
}
