use std::fmt;

use serde::de::DeserializeOwned;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

pub fn serialize<T: ZObject + Serialize, S>(value: &T, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    Object {
        ty: Default::default(),
        value,
    }
    .serialize(serializer)
}

pub fn deserialize<'de, D, T: ZObject + Deserialize<'de>>(deserializer: D) -> Result<T, D::Error>
where
    D: Deserializer<'de>,
{
    let obj: Object<T> = Object::deserialize(deserializer)?;
    Ok(obj.value)
}

/// Represents a Z1/object. This should not be used for fields.
/// Instead, use the [`serialize`] and [`deserialize`] functions,
/// with the `#[serde(with = "wikifunctions::object")]` attribute.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Object<T: ZObject> {
    #[serde(rename = "Z1K1")]
    pub ty: T::ZType,
    #[serde(flatten)] // TODO avoid flatten
    pub value: T,
}

impl<T: ZObject> Object<T> {
    pub fn new(value: T) -> Self {
        Self {
            ty: Default::default(),
            value,
        }
    }
}

pub trait ZObject {
    type ZType: DeserializeOwned + Serialize + fmt::Debug + Default;
}

impl<T: ZObject> ZObject for &'_ T {
    type ZType = T::ZType;
}
