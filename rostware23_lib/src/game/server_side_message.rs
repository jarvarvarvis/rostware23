use crate::game::result::GameResult;
use crate::game::state::State;
use crate::xml::data::DataClass;
use crate::xml::room::Room;

#[derive(Debug, PartialEq, Eq)]
pub enum ServerSideMessage {
    MoveRequest,
    Memento(super::state::State),
    Result(super::result::GameResult),
}

impl TryFrom<Room> for ServerSideMessage {
    type Error = anyhow::Error;

    fn try_from(value: Room) -> anyhow::Result<Self> {
        match value.data.class {
            DataClass::WelcomeMessage => anyhow::bail!("Welcome message was already handled!"),
            DataClass::MoveRequest => Ok(Self::MoveRequest),
            DataClass::Memento => Ok(Self::Memento(State::from(value.data.state.unwrap()))),
            DataClass::Move => anyhow::bail!("Moves are not server-side messages"),
            DataClass::Result => Ok(Self::Result(GameResult::from(value.data.result.unwrap()))),
        }
    }
}
