use instant_xml::{FromXml, ToXml};

pub const PROTOCOL_START : &'static str = "<protocol>";

#[derive(Debug, ToXml, Eq, PartialEq)]
#[xml(rename = "join")]
pub struct Join;

#[derive(Debug, ToXml, Eq, PartialEq)]
#[xml(rename = "joinRoom")]
pub struct JoinRoom {
    #[xml(attribute, rename = "roomId")]
    pub room_id: String
}

#[derive(Debug, ToXml, Eq, PartialEq)]
#[xml(rename = "joinPrepared")]
pub struct JoinPrepared {
    #[xml(attribute, rename = "reservationCode")]
    pub reservation_code: String
}

#[derive(Debug, FromXml, Eq, PartialEq)]
#[xml(rename = "joined")]
pub struct Joined {
    #[xml(attribute, rename = "roomId")]
    pub room_id: String
}

#[cfg(test)]
mod tests {
    use crate::xml::*;
    use super::*;

    #[test]
    fn serialize_join() {
        let join = Join {};
        let expected = "<join />";
        let actual = serialize(join);
        assert_eq!(expected, actual.unwrap());
    }

    #[test]
    fn serialize_join_room() {
        let join_room = JoinRoom { room_id: "TEST_ROOM_ID".to_string() };
        let expected = r#"<joinRoom roomId="TEST_ROOM_ID"></joinRoom>"#;
        let actual = serialize(join_room);
        assert_eq!(expected, actual.unwrap());
    }

    #[test]
    fn serialize_join_prepared() {
        let join_prepared = JoinPrepared { reservation_code: "TEST_RESERVATION_CODE".to_string() };
        let expected = r#"<joinPrepared reservationCode="TEST_RESERVATION_CODE"></joinPrepared>"#;
        let actual = serialize(join_prepared);
        assert_eq!(expected, actual.unwrap());
    }

    #[test]
    fn deserialize_joined() {
        let joined = r#"<joined roomId="TEST_ROOM_ID"></joined>"#;
        let expected = Joined { room_id: "TEST_ROOM_ID".to_string() };
        let actual = deserialize(joined);
        assert_eq!(expected, actual.unwrap());
    }
}
