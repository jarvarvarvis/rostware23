use crate::xml;

use super::server::Connection;
use super::protocol_error::*;

pub enum JoinKind {
    Any,
    Room(String),
    Reservation(String)
}

pub struct Protocol {
    connection: Connection,
    pub room_id: String
}

impl Protocol {
    pub fn from_connection(connection: Connection) -> Self {
        Self {
            connection,
            room_id: String::new()
        }
    }

    pub fn join_game(&mut self, join_kind: JoinKind) -> anyhow::Result<()> {
        self.connection.write_string_slice(xml::connection::PROTOCOL_START)?;
        let join_data = match join_kind {
            JoinKind::Any => xml::serialize(xml::connection::Join)?,
            JoinKind::Room(room_id) => xml::serialize(xml::connection::JoinRoom { room_id })?,
            JoinKind::Reservation(reservation_code) => xml::serialize(xml::connection::JoinPrepared { reservation_code })?,
        };
        self.connection.write_string(join_data)?;
        self.connection.flush_writer()?;
        Ok(())
    }

    pub fn deserialize_error(message: &str) -> anyhow::Result<xml::error::ErrorPacket> {
        xml::deserialize(message)
    }

    pub fn read_message_after_join(&mut self) -> anyhow::Result<()> {
        let mut initial_message = self.connection.read_fully_into_string()?;
        
        // Remove <protocol>\n prefix
        initial_message = initial_message.replace("<protocol>\n", "");

        println!("Received message: {initial_message}");
        let joined = xml::deserialize::<xml::connection::Joined>(&initial_message);
        if joined.is_err() {
            let error = Self::deserialize_error(&initial_message)?;
            anyhow::bail!(ProtocolError::from(error))
        }

        let joined = joined.unwrap();

        self.room_id = joined.room_id;
        Ok(())
    }
}
