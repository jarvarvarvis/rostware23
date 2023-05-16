use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "UPPERCASE")]
pub enum Team {
    One,
    Two
}
