use serde_json::{from_str, to_string_pretty};
use std::error::Error;

use crate::model::list::TypedList;
use crate::model::{Object, Pair, ZString, ZUnit};

#[test]
pub fn serialize_list() -> Result<(), Box<dyn Error>> {
    let list = TypedList::<ZString> { inner: vec!["1".into(), "2".into()] };
    let obj = Object::new(list);
    let s = to_string_pretty(&obj)?;
    println!("{s}"); // TODO assert
    Ok(())
}

#[test]
pub fn deserialize_list() -> Result<(), Box<dyn Error>> {
    let json = include_str!("../test_data/list_string.json");

    let o: Object<TypedList<ZString>> = from_str(json)?;
    dbg!(o);

    Ok(())
}

#[test]
pub fn deserialize() -> Result<(), Box<dyn Error>> {
    let json = r#"{
        "Z1K1": {
            "Z1K1": "Z9",
            "Z9K1": "Z22"
        },
        "Z22K1": {
            "Z1K1": "Z6",
            "Z6K1": "13"
        },
        "Z22K2": {
            "Z1K1": "Z9",
            "Z9K1": "Z24"
        }    
    }"#;

    let obj: Object<Pair<ZString, ZUnit>> = from_str(json)?;

    dbg!(obj);

    Ok(())
}
