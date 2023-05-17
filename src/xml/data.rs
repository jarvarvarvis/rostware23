use instant_xml::{FromXml, ToXml};

use super::common;
use super::state;
use super::moves;
use super::result;

#[derive(FromXml, ToXml, Debug, Eq, PartialEq)]
#[xml(scalar, rename_all = "camelCase")]
pub enum DataClass {
    WelcomeMessage,
    MoveRequest,
    Memento,
    Move,
    Result,
}

#[derive(FromXml, ToXml, Debug, Eq, PartialEq)]
#[xml(rename = "data")]
pub struct Data {
    #[xml(attribute, rename = "class")]
    pub class: DataClass,

    #[xml(attribute, rename = "color")]
    pub color: Option<common::Team>,

    pub state: Option<state::State>,

    pub sent_move: Option<moves::Move>,

    pub result: Option<result::GameResult>

}

#[cfg(test)]
mod tests {
    use crate::xml::*;
    use super::*;

    #[test]
    fn deserialize_data_welcome_message() {
        let welcome_message = r#"<data class="welcomeMessage" color="ONE"></data>"#;
        let expected = Data {
            class: DataClass::WelcomeMessage,
            color: Some(common::Team::One),
            state: None,
            sent_move: None,
            result: None
        };
        let actual = deserialize(welcome_message).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn deserialize_data_move() {
        let welcome_message = r#"<data class="move"></data>"#;
        let expected = Data {
            class: DataClass::Move,
            state: None,
            color: None,
            sent_move: None,
            result: None
        };
        let actual = deserialize(welcome_message).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn deserialize_data_memento() {
        let welcome_message = r#"<data class="memento"></data>"#;
        let expected = Data {
            class: DataClass::Memento,
            state: None,
            color: None,
            sent_move: None,
            result: None
        };
        let actual = deserialize(welcome_message).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn deserialize_data_result() {
        let welcome_message = r#"<data class="result"></data>"#;
        let expected = Data {
            class: DataClass::Result,
            state: None,
            color: None,
            sent_move: None,
            result: None
        };
        let actual = deserialize(welcome_message).unwrap();
        assert_eq!(expected, actual);
    }
}
