use std::fmt;


pub mod ffi;

// re-exports for use by compiled programs
pub use postcard::{from_bytes, to_stdvec};

pub fn bytes_to_values(b: ffi::Bytes<'_>) -> postcard::Result<Vec<Value>> {
    postcard::from_bytes(b.as_slice())
}

pub struct ExpectedFound {
    pub expected: &'static str,
    pub found: &'static str,
}

impl fmt::Display for ExpectedFound {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "expected {}, found {}", self.expected, self.found)
    }
}

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
pub enum Value {
    String(String),
}

impl TryFrom<Value> for String {
    type Error = ExpectedFound;
    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::String(s) => Ok(s),
        }
    }
}

pub trait IntoValue {
    fn into_value(self) -> Value;
}

impl IntoValue for String {
    fn into_value(self) -> Value {
        Value::String(self)
    }
}

impl IntoValue for &'_ str {
    fn into_value(self) -> Value {
        Value::String(self.to_owned())
    }
}