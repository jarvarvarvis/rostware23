extern crate quick_xml;

pub mod common;
pub mod connection;
pub mod data;
pub mod room;
pub mod state;

use serde::de::DeserializeOwned;

use anyhow::Result;

use quick_xml::se::to_string;
use quick_xml::de::from_str;

pub fn deserialize<D: DeserializeOwned>(data: String) -> Result<D> {
    let result = from_str(&data)?;
    Ok(result)
}

pub fn serialize<S: serde::Serialize>(data: S) -> Result<String> {
    let result = to_string(&data)?;
    Ok(result)
}
