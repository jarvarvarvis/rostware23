use crate::xml::error::*;

#[derive(Debug, Eq, PartialEq)]
pub enum ProtocolError {
    InvalidJoinRoomId { room_id: String },
    InvalidReservation { reservation: String },
    Other { message: String }
}

impl std::fmt::Display for ProtocolError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ProtocolError::InvalidJoinRoomId { room_id } => write!(f, "Server couldn't find a room with id {room_id}"),
            ProtocolError::InvalidReservation { reservation } => write!(f, "Server couldn't find a reservation for the code {reservation}"),
            ProtocolError::Other { message } => write!(f, "{message}"),
        }
    }
}

impl From<ErrorPacket> for ProtocolError {
    fn from(packet: ErrorPacket) -> Self {
        if let Some(original_request) = packet.original_request {
            match original_request.class.as_str() {
                "joinRoom" if original_request.room_id.is_some() => 
                    return Self::InvalidJoinRoomId { room_id: original_request.room_id.unwrap() },
                "joinPrepared" if original_request.reservation_code.is_some() => 
                    return Self::InvalidReservation { reservation: original_request.reservation_code.unwrap() },
                _ => {}
            }
        }
        Self::Other { message: packet.message }
    }
}

#[cfg(test)]
mod tests {
    use crate::xml::*;
    use super::*;

    #[test]
    fn invalid_join_room_id_error_from_errorpacket() {
        let packet = ErrorPacket {
            message: "sc.api.plugins.exceptions.GameRoomException: Couldn't find a room with id amogus".to_string(),
            original_request: Some(OriginalRequest { 
                class: "joinRoom".to_string(),
                room_id: Some("amogus".to_string()),
                reservation_code: None
            })
        };
        let expected = ProtocolError::InvalidJoinRoomId { room_id: "amogus".to_string() };
        let actual = ProtocolError::from(packet);
        assert_eq!(expected, actual);
    }

    #[test]
    fn invalid_reservation_code_error_from_errorpacket() {
        let packet = ErrorPacket {
            message: "sc.server.gaming.UnknownReservationException: Couldn't find a reservation for the provided token.".to_string(),
            original_request: Some(OriginalRequest { 
                class: "joinPrepared".to_string(),
                room_id: None,
                reservation_code: Some("amogus".to_string()),
            })
        };
        let expected = ProtocolError::InvalidReservation { reservation: "amogus".to_string() };
        let actual = ProtocolError::from(packet);
        assert_eq!(expected, actual);
    }

    #[test]
    fn other_error_from_errorpacket() {
        let packet = ErrorPacket {
            message: "sc.server.gaming.OtherException: This is some other error.".to_string(),
            original_request: None,
        };
        let expected = ProtocolError::Other { message: "sc.server.gaming.OtherException: This is some other error.".to_string() };
        let actual = ProtocolError::from(packet);
        assert_eq!(expected, actual);
    }
}
