use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq)]
pub enum Team {
    #[serde(rename = "ONE")]
    One,

    #[serde(rename = "TWO")]
    Two
}
