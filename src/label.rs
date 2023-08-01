use serde::{Deserialize, Serialize};

pub trait ZLabel {
    const LABEL: &'static str;
    const VAL: Self;
}

macro_rules! impl_zlabel {
    ($($ident:ident)*) => {
        $(
            #[derive(Debug, Default)]
            pub struct $ident;

            impl<'de> Deserialize<'de> for $ident {
                fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
                where
                    D: serde::Deserializer<'de>,
                {
                    let label = String::deserialize(deserializer)?;
                    if label == stringify!($ident) {
                        Ok($ident)
                    } else {
                        Err(serde::de::Error::custom(format!(
                            concat!("expected label ", stringify!($ident), ", got {}"),
                            label
                        )))
                    }
                }
            }

            impl Serialize for $ident {
                fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
                where
                    S: serde::Serializer,
                {
                    serializer.serialize_str(stringify!($ident))
                }
            }
        )*
    };
}

impl_zlabel!(Z6 Z7 Z9 Z11 Z12 Z14 Z16 Z17 Z22 Z24 Z60 Z61 Z881);
