extern crate rostware23_lib;

mod cmdline;
mod logic;

use cmdline::ClientArgs;
use rostware23_lib::game::protocol::Protocol;
use rostware23_lib::game::server_side_message::*;

use crate::logic::*;
use crate::logic::pvs_getter::PVSMoveGetter;
use crate::logic::combined_rater::CombinedRater;
use crate::logic::time_measurer::TimeMeasurer;

fn main() -> anyhow::Result<()> {
    let mut protocol: Protocol = ClientArgs::parse()?.try_into()?;
    protocol.read_welcome_message()?;

    let mut current_state = None;
    let move_getter = PVSMoveGetter::<CombinedRater>::new();
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
                let time_measurer = TimeMeasurer::new(1900);
                let chosen_move = move_getter.get_move(&current_state.with_moveless_player_skipped()?, &time_measurer)?;
                println!("Sending move {:?} after {}ms", chosen_move, time_measurer.elapsed_millis_since_start());
                protocol.send_move(chosen_move)?;
            }
        }

        if let ServerSideMessage::Result(result) = &server_side_message {
            println!("Own team: {:?}", protocol.own_team.unwrap());
            println!("Result: {:?}", result);
            break;
        }
    }

    Ok(())
}
