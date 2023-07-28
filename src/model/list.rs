use std::marker::PhantomData;
use std::slice;

use serde::de::{DeserializeSeed, Error, Visitor};
use serde::ser::SerializeMap;
use serde::{Deserialize, Serialize};

use crate::label;

use super::{FunctionCall, Object, Reference, TypeListArgs, ZObject};

/// A Z881/Typed list
#[derive(Debug)] // TODO hand debug
pub struct TypedList<T: ZObject> {
    pub inner: Vec<T>,
}

// (de)serialization for typed lists. typed lists are represented as a linked list T
// with elements E where T is either (E, T1) where T1 is another typed list, or ().

pub struct TypedListVisitor<T> {
    entries: Vec<T>,
    expecting_type: bool,
}

impl<'de, T> DeserializeSeed<'de> for TypedListVisitor<T>
where
    T: ZObject + Deserialize<'de>,
{
    type Value = TypedListVisitor<T>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_map(self)
    }
}

impl<'de, T> Visitor<'de> for TypedListVisitor<T>
where
    T: ZObject + Deserialize<'de>,
{
    type Value = TypedListVisitor<T>;
    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.pad("a map")
    }
    fn visit_map<A>(mut self, mut map: A) -> Result<TypedListVisitor<T>, A::Error>
    where
        A: serde::de::MapAccess<'de>,
    {
        if self.expecting_type {
            let Some((k, _)) = map.next_entry::<String, <TypedList<T> as ZObject>::ZType>()? else {
                return Err(A::Error::custom("expected Z1K1 entry"));
            };

            if k != "Z1K1" {
                return Err(A::Error::custom("expected Z1K1"));
            }
            self.expecting_type = false;
        }
        if let Some((k, v)) = map.next_entry::<String, Object<T>>()? {
            if k != "K1" {
                return Err(A::Error::custom("expected K1"));
            }
            self.entries.push(v.value);

            self.expecting_type = true;
            let Some((k, mut this)) =
                map.next_entry_seed::<PhantomData<String>, Self>(PhantomData, self)?
            else {
                return Err(A::Error::custom("expected K2 entry after K1"));
            };
            this.expecting_type = false;

            if k != "K2" {
                return Err(A::Error::custom("expected K2 after K1"));
            }

            Ok(this)
        } else {
            Ok(self)
        }
    }
}

impl<'de, T: ZObject + Deserialize<'de>> Deserialize<'de> for TypedList<T> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let visitor = deserializer.deserialize_map(TypedListVisitor {
            entries: Vec::new(),
            expecting_type: false,
        })?;
        Ok(TypedList {
            inner: visitor.entries,
        })
    }
}

struct TypedListSerializeImpl<'a, T> {
    iter: slice::Iter<'a, T>,
}

impl<'a, T: ZObject> ZObject for TypedListSerializeImpl<'a, T> {
    type ZType = <TypedList<T> as ZObject>::ZType;
}

impl<T: ZObject + Serialize> Serialize for TypedListSerializeImpl<'_, T> {
    #[inline]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut iter = self.iter.clone();

        if let Some(val) = iter.next() {
            let mut serializer = serializer.serialize_map(Some(2))?;
            serializer.serialize_entry("K1", &Object::new(val))?;
            serializer.serialize_entry("K2", &Object::new(TypedListSerializeImpl { iter }))?;
            serializer.end()
        } else {
            serializer.serialize_map(Some(0))?.end()
        }
    }
}

impl<T: ZObject + Serialize> Serialize for TypedList<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        TypedListSerializeImpl {
            iter: self.inner.iter(),
        }
        .serialize(serializer)
    }
}

impl<T: ZObject> ZObject for TypedList<T> {
    type ZType =
        Object<FunctionCall<Reference<label::Z881>, TypeListArgs<Object<Reference<T::ZType>>>>>;
}
