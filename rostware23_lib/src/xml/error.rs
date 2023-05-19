use instant_xml::FromXml;

#[derive(Debug, FromXml, Eq, PartialEq)]
#[xml(rename = "originalRequest")]
pub struct OriginalRequest {
    #[xml(attribute)]
    pub class: String,

    #[xml(attribute, rename = "roomId")]
    pub room_id: Option<String>,

    #[xml(attribute, rename = "reservationCode")]
    pub reservation_code: Option<String>
}

#[derive(Debug, FromXml, Eq, PartialEq)]
#[xml(rename = "errorpacket")]
pub struct ErrorPacket {
    #[xml(attribute)]
    pub message: String,

    pub original_request: Option<OriginalRequest>
}

#[cfg(test)]
mod tests {
    use crate::xml::*;
    use super::*;

    #[test]
    fn deserialize_unknown_reservation() {
        let packet = r#"<errorpacket message="sc.server.gaming.UnknownReservationException: Couldn&apos;t find a reservation for the provided token.">
            <originalRequest class="joinPrepared" reservationCode="amogus"/>
        </errorpacket>"#;
        let expected = ErrorPacket {
            message: "sc.server.gaming.UnknownReservationException: Couldn't find a reservation for the provided token.".to_string(),
            original_request: Some(OriginalRequest { 
                class: "joinPrepared".to_string(),
                room_id: None,
                reservation_code: Some("amogus".to_string()),
            })
        };
        let actual = deserialize(packet).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn deserialize_invalid_room_id() {
        let packet = r#"<errorpacket message="sc.api.plugins.exceptions.GameRoomException: Couldn't find a room with id amogus">
            <originalRequest class="joinRoom" roomId="amogus"/>
        </errorpacket>"#;
        let expected = ErrorPacket {
            message: "sc.api.plugins.exceptions.GameRoomException: Couldn't find a room with id amogus".to_string(),
            original_request: Some(OriginalRequest { 
                class: "joinRoom".to_string(),
                room_id: Some("amogus".to_string()),
                reservation_code: None
            })
        };
        let actual = deserialize(packet).unwrap();
        assert_eq!(expected, actual);
    }
}
