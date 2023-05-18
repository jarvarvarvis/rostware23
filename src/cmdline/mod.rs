extern crate args;
extern crate getopts;

use std::env;

use getopts::Occur;
use args::Args;

use crate::game::protocol::*;
use crate::game::server::*;

#[derive(Debug, Clone)]
pub struct ClientArgs {
    pub host: String,
    pub port: i32,
    pub reservation: Option<String>,
    pub room: Option<String>
}

impl ClientArgs {
    fn setup_args(program: &str) -> Args {
        let mut args = Args::new(
            program,
            "A client for Hey, Danke für den Fisch (Software Challenge 2023)",
        );
        args.option(
            "h",
            "host",
            "The IP address of the host to connect to",
            "HOST",
            Occur::Req,
            Some("localhost".to_string()),
        );
        args.option(
            "p",
            "port",
            "The port used for the connection",
            "PORT",
            Occur::Req,
            Some("13050".to_string()),
        );
        args.option(
            "r",
            "reservation",
            "The reservation code required for joining a prepared game.",
            "CODE",
            Occur::Optional,
            None,
        );
        args.option(
            "",
            "room",
            "The room id required for joining a specific room.",
            "ROOM",
            Occur::Optional,
            None
        );

        args
    }

    fn create_client_args(args: Args) -> anyhow::Result<Self> {
        let host = args.value_of::<String>("host")?;
        let port_string = args.value_of::<String>("port")?;
        let reservation = args.optional_value_of::<String>("reservation")?;
        let room = args.optional_value_of::<String>("room")?;

        let port = port_string.parse::<i32>()?;
        Ok(Self {
            host,
            port,
            reservation,
            room,
        })
    }

    pub fn parse() -> anyhow::Result<Self> {
        let env_args: Vec<String> = env::args().collect();
        let program = env_args.first().unwrap();

        let mut args = Self::setup_args(program);

        let parse_result = args.parse(env_args);

        if parse_result.is_err() {
            let usage = args.full_usage();
            println!("{}", usage);

            let error = parse_result.unwrap_err();
            anyhow::bail!(error);
        }

        Self::create_client_args(args)
    }
}

impl TryInto<Protocol> for ClientArgs {
    type Error = anyhow::Error;

    fn try_into(self) -> Result<Protocol, Self::Error> {
        println!("Got args: {:?}", self);
        let address = format!("{}:{}", self.host, self.port);
        let connection = Connection::connect(&address)?;
        let mut protocol = Protocol::from_connection(connection);

        match (self.room, self.reservation) {
            (None, None) => protocol.join_game(JoinKind::Any)?,
            (None, Some(reservation)) => protocol.join_game(JoinKind::Reservation(reservation))?,
            (Some(room), None) => protocol.join_game(JoinKind::Room(room))?,
            (Some(_), Some(_)) => anyhow::bail!("Can't join reserved game and room at the same time"),
        }

        protocol.read_message_after_join()?;
        println!("Joined game at {}", address);

        Ok(protocol)
    }
}
