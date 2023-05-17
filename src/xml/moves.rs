use instant_xml::{FromXml, ToXml};

use super::common;

#[derive(FromXml, ToXml, Debug, Eq, PartialEq)]
#[xml(rename = "from")]
pub struct From {
    #[xml(attribute)]
    pub x: u32,

    #[xml(attribute)]
    pub y: u32,
}

#[derive(FromXml, ToXml, Debug, Eq, PartialEq)]
#[xml(rename = "to")]
pub struct To {
    #[xml(attribute)]
    pub x: u32,

    #[xml(attribute)]
    pub y: u32,
}

#[derive(FromXml, ToXml, Debug, Eq, PartialEq)]
#[xml(transparent)]
pub struct Move {
    pub from: Option<From>,
    pub to: To
}

#[cfg(test)]
mod tests {
    use crate::xml::*;
    use super::*;

    #[test]
    fn serialize_normal_move() {
        let r#move = data::Data {
            class: data::DataClass::Move,
            color: None,
            state: None,
            sent_move: Some(Move {
                from: Some(From { x: 0, y: 7 }),
                to: To { x: 4, y: 5 }
            }),
            result: None
        };
        let expected = r#"<data class="move"><from x="0" y="7"></from><to x="4" y="5"></to></data>"#;
        let actual = serialize(r#move).unwrap();
        assert_eq!(expected.to_string(), actual);
    }

    #[test]
    fn serialize_place_move() {
        let r#move = data::Data {
            class: data::DataClass::Move,
            color: None,
            state: None,
            sent_move: Some(Move {
                from: None,
                to: To { x: 3, y: 7 }
            }),
            result: None
        };
        let expected = r#"<data class="move"><to x="3" y="7"></to></data>"#;
        let actual = serialize(r#move).unwrap();
        assert_eq!(expected.to_string(), actual);
    }
}
