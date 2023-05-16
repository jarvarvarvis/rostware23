extern crate serde_xml_rs;

pub mod common;
pub mod connection;
pub mod data;
pub mod room;
pub mod state;

use serde::de::DeserializeOwned;

use anyhow::Result;

use serde_xml_rs::{from_str, to_string};

pub fn deserialize<D: DeserializeOwned>(data: String) -> Result<D> {
    let result = from_str(&data)?;
    Ok(result)
}

pub fn serialize<S: serde::Serialize>(data: S) -> Result<String> {
    let result = to_string(&data)?;
    // serde-xml-rs (or rather the xml-rs crate) always adds an XML prolog to the generated
    // output. I haven't found a way to turn this off yet, so this solution will hopefully
    // suffice for now.
    let sanitized_result = result.replace("<?xml version=\"1.0\" encoding=\"UTF-8\"?>", "");
    Ok(sanitized_result)
}
