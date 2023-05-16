use serde::{Serialize, Deserialize};

use super::common;

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum DataClass {
    WelcomeMessage,
    MoveRequest,
    Move,
    Result
}

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Data {
    #[serde(rename = "@class")]
    pub class: DataClass,

    #[serde(rename = "@color")]
    pub color: common::Team,
}
