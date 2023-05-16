extern crate instant_xml;
extern crate instant_xml_macros;

pub mod common;
pub mod connection;
pub mod data;
pub mod room;
pub mod state;

use anyhow::Result;
use instant_xml::{FromXml, ToXml};

pub fn deserialize<'de, D: FromXml<'de>>(data: &'de str) -> Result<D> {
    let result = instant_xml::from_str(data)?;
    Ok(result)
}

pub fn serialize<S: ToXml>(data: S) -> Result<String> {
    let result = instant_xml::to_string(&data)?;
    Ok(result)
}
