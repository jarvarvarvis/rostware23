mod xml;
mod game;
mod logic;

use game::server::Connection;
use game::protocol::{Protocol, JoinKind};
use game::server_side_message::*;

use crate::logic::*;
use crate::logic::random_getter::RandomGetter;

fn main() -> anyhow::Result<()> {
    let connection = Connection::connect("127.0.0.1:13050")?;
    println!("Connection with game server established on port 13050");
    let mut protocol = Protocol::from_connection(connection);
    protocol.join_game(JoinKind::Any)?;
    protocol.read_message_after_join()?;
    protocol.read_welcome_message()?;

    let mut current_state = None;
    let move_getter = RandomGetter::new();
    loop {
        let current_room_message = protocol.read_room_message()?;
        let server_side_message = ServerSideMessage::try_from(current_room_message)?;

        if let ServerSideMessage::Memento(state) = &server_side_message {
            println!("Current state:\n{}", state);
            current_state = Some(state.clone());
        }

        if let ServerSideMessage::MoveRequest = &server_side_message {
            println!("Got move request");
            if let Some(current_state) = &current_state {
                let chosen_move = move_getter.get_move(current_state)?;
                println!("Sending move: {:?}", chosen_move);
                protocol.send_move(chosen_move)?;
            }
        }

        if let ServerSideMessage::Result(result) = &server_side_message {
            println!("Result: {:?}", result);
            break;
        }
    }

    Ok(())
}
