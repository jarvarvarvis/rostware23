use instant_xml::{FromXml, ToXml};

use super::data::Data;

#[derive(FromXml, ToXml, Debug, Eq, PartialEq)]
#[xml(rename = "room")]
pub struct Room {
    #[xml(attribute, rename = "roomId")]
    pub room_id: String,

    #[xml(rename = "data")]
    pub data: Data,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::xml::*;

    #[test]
    fn deserialize_welcome_message() {
        let welcome_message = r#"<room roomId="TEST_ROOM_ID">
            <data class="welcomeMessage" color="ONE"></data>
        </room>"#;
        let expected = Room {
            room_id: "TEST_ROOM_ID".to_string(),
            data: data::Data {
                class: data::DataClass::WelcomeMessage,
                color: Some(common::Team::One),
                state: None,
                sent_move: None,
                result: None,
            },
        };
        let actual = deserialize(welcome_message).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn deserialize_move_request() {
        let welcome_message = r#"<room roomId="TEST_ROOM_ID">
            <data class="moveRequest"></data>
        </room>"#;
        let expected = Room {
            room_id: "TEST_ROOM_ID".to_string(),
            data: data::Data {
                class: data::DataClass::MoveRequest,
                color: None,
                state: None,
                sent_move: None,
                result: None,
            },
        };
        let actual = deserialize(welcome_message).unwrap();
        assert_eq!(expected, actual);
    }
}
