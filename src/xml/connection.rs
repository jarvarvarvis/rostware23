use serde::{Serialize, Deserialize};

pub const PROTOCOL_START : &'static str = "<protocol>";

#[derive(Debug, Serialize, Eq, PartialEq)]
#[serde(rename = "join")]
pub struct Join {}

#[derive(Debug, Serialize, Eq, PartialEq)]
#[serde(rename = "joinRoom")]
pub struct JoinRoom {
    #[serde(rename = "@roomId")]
    pub room_id: String
}

#[derive(Debug, Serialize, Eq, PartialEq)]
#[serde(rename = "joinPrepared")]
pub struct JoinPrepared {
    #[serde(rename = "@reservationCode")]
    pub reservation_code: String
}

#[derive(Debug, Deserialize, Eq, PartialEq)]
#[serde(rename = "joined")]
pub struct Joined {
    #[serde(rename = "roomId")]
    pub room_id: String
}

#[cfg(test)]
mod tests {
    use crate::xml::*;
    use super::*;

    #[test]
    fn test_serialize_join() {
        let join = Join {};
        let expected = "<join />".to_string();
        let actual = serialize(join);
        assert_eq!(expected, actual.unwrap());
    }

    #[test]
    fn test_serialize_join_room() {
        let join_room = JoinRoom { room_id: "TEST_ROOM_ID".to_string() };
        let expected = r#"<joinRoom roomId="TEST_ROOM_ID" />"#.to_string();
        let actual = serialize(join_room);
        assert_eq!(expected, actual.unwrap());
    }

    #[test]
    fn test_serialize_join_prepared() {
        let join_prepared = JoinPrepared { reservation_code: "TEST_RESERVATION_CODE".to_string() };
        let expected = r#"<joinPrepared reservationCode="TEST_RESERVATION_CODE" />"#.to_string();
        let actual = serialize(join_prepared);
        assert_eq!(expected, actual.unwrap());
    }

    #[test]
    fn test_deserialize_joined() {
        let joined = r#"<joined roomId="TEST_ROOM_ID" />"#.to_string();
        let expected = Joined { room_id: "TEST_ROOM_ID".to_string() };
        let actual = deserialize(joined);
        assert_eq!(expected, actual.unwrap());
    }
}
