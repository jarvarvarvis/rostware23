use serde::{Serialize, Deserialize};

use super::common;

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum DataClass {
    WelcomeMessage,
}

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq)]
pub struct Data {
    pub class: DataClass,
    pub color: common::Team,
}

#[cfg(test)]
mod tests {
    use crate::xml::*;
    use super::*;

    #[test]
    fn test_deserialize_inner_welcome_message() {
        let welcome_message = r#"<data class="welcomeMessage" color="ONE"></data>"#;
        let expected = Data {
            class: DataClass::WelcomeMessage,
            color: common::Team::One
        };
        let actual = deserialize(welcome_message.to_string()).unwrap();
        assert_eq!(expected, actual);
    }
}
