use serde::Deserialize;

use super::data::Data;

#[derive(Debug, Deserialize, Eq, PartialEq)]
#[serde(rename = "room")]
pub struct Room {
    #[serde(rename = "@roomId")]
    pub room_id: String,

    pub data: Data
}

#[cfg(test)]
mod tests {
    use crate::xml::*;
    use super::*;

    #[test]
    fn test_deserialize_welcome_message() {
        let welcome_message = r#"<room roomId="TEST_ROOM_ID">
            <data class="welcomeMessage" color="ONE"></data>
        </room>"#;
        let expected = Room {
            room_id: "TEST_ROOM_ID".to_string(),
            data: data::Data {
                class: data::DataClass::WelcomeMessage,
                color: common::Team::One
            }
        };
        let actual = deserialize(welcome_message.to_string()).unwrap();
        assert_eq!(expected, actual);
    }
}
