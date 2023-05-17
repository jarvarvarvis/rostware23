use crate::xml;
use crate::game::state::State;
use crate::game::result::GameResult;

#[derive(Debug)]
pub enum ServerSideMessage {
    MoveRequest,
    Memento(super::state::State),
    Result(super::result::GameResult)
}

impl TryFrom<xml::room::Room> for ServerSideMessage {
    type Error = anyhow::Error;

    fn try_from(value: xml::room::Room) -> anyhow::Result<Self> {
        match value.data.class {
            xml::data::DataClass::WelcomeMessage => anyhow::bail!("Welcome message was already handled!"),
            xml::data::DataClass::MoveRequest => Ok(Self::MoveRequest),
            xml::data::DataClass::Memento => Ok(Self::Memento(State::from(value.data.state.unwrap()))),
            xml::data::DataClass::Move => anyhow::bail!("Moves are not server-side messages"),
            xml::data::DataClass::Result => Ok(Self::Result(GameResult::from(value.data.result.unwrap()))),
        }
    }
}
