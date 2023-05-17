mod xml;
mod game;

use game::server::Connection;
use game::protocol::{Protocol, JoinKind};

fn main() -> anyhow::Result<()> {
    let connection = Connection::connect("127.0.0.1:13050")?;
    println!("Connection with game server established on port 13050");
    let mut protocol = Protocol::from_connection(connection);
    protocol.join_game(JoinKind::Any)?;
    protocol.read_message_after_join()?;
    protocol.read_welcome_message()?;

    Ok(())
}
