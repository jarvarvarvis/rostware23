mod xml;
mod game;

use game::server::Connection;

fn main() -> anyhow::Result<()> {
    let mut connection = Connection::connect("127.0.0.1:13050")?;
    println!("Connection established!");
    connection.write_string_slice(xml::connection::PROTOCOL_START)?;
    connection.write_string(xml::serialize(xml::connection::Join)?)?;
    connection.flush_writer()?;

    let initial_message = connection.read_fully_into_string()?;
    println!("Got data: {:?}", initial_message);
    Ok(())
}
