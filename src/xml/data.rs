use instant_xml::{FromXml, ToXml};

use super::common;
use super::moves;

#[derive(FromXml, ToXml, Debug, Eq, PartialEq)]
#[xml(scalar, rename_all = "camelCase")]
pub enum DataClass {
    WelcomeMessage,
    MoveRequest,
    Move,
}

#[derive(FromXml, ToXml, Debug, Eq, PartialEq)]
#[xml(rename = "data")]
pub struct Data {
    #[xml(attribute, rename = "class")]
    pub class: DataClass,

    #[xml(attribute, rename = "color")]
    pub color: Option<common::Team>,

    pub sent_move: Option<moves::Move>
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
            color: Some(common::Team::One),
            sent_move: None
        };
        let actual = deserialize(welcome_message).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_deserialize_inner_move_request() {
        let welcome_message = r#"<data class="moveRequest"></data>"#;
        let expected = Data {
            class: DataClass::MoveRequest,
            color: None,
            sent_move: None
        };
        let actual = deserialize(welcome_message).unwrap();
        assert_eq!(expected, actual);
    }
}
