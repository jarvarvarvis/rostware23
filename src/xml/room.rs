use instant_xml::FromXml;

use super::data::Data;

#[derive(FromXml, Debug, Eq, PartialEq)]
#[xml(rename = "room")]
pub struct Room {
    #[xml(attribute, rename = "roomId")]
    pub room_id: String,

    #[xml(rename = "data")]
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
        let actual = deserialize(welcome_message).unwrap();
        assert_eq!(expected, actual);
    }
}
