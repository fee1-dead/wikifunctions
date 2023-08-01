use serde_json::{json, Value as JsonValue};

#[derive(Clone, Copy)]
pub enum Type {
    String,
    /// Z21/unit
    Unit,
    Boolean,
    List,
    Pair,
    Map,
}

impl Type {
    pub fn from_json(v: &JsonValue) -> Option<Self> {
        match v {
            JsonValue::String(s) if s == "Z6" => Some(Type::String),
            JsonValue::Object(obj) => match obj.get("Z1K1")? {
                JsonValue::String(ty) if ty == "Z9" => {
                    let Some(JsonValue::String(ref_id)) = obj.get("Z9K1") else {
                        return None;
                    };

                    (ref_id == "Z21").then(|| Type::Unit)
                }
                JsonValue::Object(obj) => {
                    // result of a Z7/function call
                    if obj.get("Z1K1")? != &json!({"Z1K1": "Z9", "Z9K1": "Z7"}) {
                        return None;
                    }

                    match obj.get("Z7K1")?.get("Z9K1")?.as_str()? {
                        "Z881" => Some(Type::List),
                        "Z882" => Some(Type::Pair),
                        _ => None,
                    }
                }
                _ => None,
            },
            _ => None,
        }
    }
    
    pub fn to_value(self, mut v: JsonValue) -> Option<wfrt::Value> {
        match self {
            Type::String => match v.get_mut("Z6K1")?.take() {
                JsonValue::String(s) => Some(wfrt::Value::String(s)),
                _ => None
            },
            _ => None,
        }
    }
}
