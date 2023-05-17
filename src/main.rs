mod xml;
mod game;

use game::server::Connection;
use game::protocol::{Protocol, JoinKind};
use game::server_side_message::*;

fn main() -> anyhow::Result<()> {
    let connection = Connection::connect("127.0.0.1:13050")?;
    println!("Connection with game server established on port 13050");
    let mut protocol = Protocol::from_connection(connection);
    protocol.join_game(JoinKind::Any)?;
    protocol.read_message_after_join()?;
    protocol.read_welcome_message()?;

    loop {
        let current_room_message = protocol.read_room_message()?;
        let server_side_message = ServerSideMessage::try_from(current_room_message)?;

        if let ServerSideMessage::Memento(state) = &server_side_message {
            println!("Got current state:\n{}", state);
        }

        if let ServerSideMessage::Result(result) = &server_side_message {
            println!("Result: {:?}", result);
            protocol.read_protocol_close_tag()?;
            break;
        }
    }

    Ok(())
}
