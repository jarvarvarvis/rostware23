use crate::xml;

use super::common;
use super::moves::Move;
use super::server::Connection;
use super::protocol_error::*;

pub enum JoinKind {
    Any,
    Room(String),
    Reservation(String)
}

pub struct Protocol {
    pub connection: Connection,
    pub room_id: String,
    pub own_team: Option<common::Team>
}

impl Protocol {
    pub fn from_connection(connection: Connection) -> Self {
        Self {
            connection,
            room_id: String::new(),
            own_team: None,
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

        let joined = xml::deserialize::<xml::connection::Joined>(&initial_message);
        if joined.is_err() {
            let error = Self::deserialize_error(&initial_message)?;
            anyhow::bail!(ProtocolError::from(error))
        }

        let joined = joined.unwrap();
        println!("Joined room with id {}", joined.room_id);

        self.room_id = joined.room_id;
        Ok(())
    }

    pub fn read_room_message(&mut self) -> anyhow::Result<xml::room::Room> {
        let room_message = self.connection.read_string_until_condition(&|text: &str| {
            return text.ends_with("</room>");
        })?;
        let room = xml::deserialize::<xml::room::Room>(&room_message);
        if room.is_err() {
            let error = Self::deserialize_error(&room_message)?;
            anyhow::bail!(ProtocolError::from(error))
        }
        Ok(room.unwrap())
    }

    pub fn read_welcome_message(&mut self) -> anyhow::Result<()> {
        let room = self.read_room_message()?;
        if room.room_id != self.room_id {
            anyhow::bail!("Expected room id {}, got {}", self.room_id, room.room_id);
        }

        if room.data.class != xml::data::DataClass::WelcomeMessage {
            anyhow::bail!("Expected data class {:?}, got {:?}", xml::data::DataClass::WelcomeMessage, room.data.class);
        }

        if room.data.color.is_none() {
            anyhow::bail!("Expected color attribute on welcome message");
        }

        self.own_team = room.data.color;
        println!("Own team: {:?}", self.own_team);

        Ok(())
    }

    pub fn send_move(&mut self, sent_move: Move) -> anyhow::Result<()> {
        let xml_move: xml::moves::Move = sent_move.into();
        let sent_room = xml::room::Room {
            room_id: self.room_id.clone(),
            data: xml::data::Data {
                class: xml::data::DataClass::Move,
                color: None,
                state: None,
                sent_move: Some(xml_move),
                result: None,
            },
        };
        let text = xml::serialize(sent_room)?;
        self.connection.write_string(text)?;
        self.connection.flush_writer()
    }
}
