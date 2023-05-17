use super::common::*;

use crate::xml;

use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq)]
pub struct State {
    pub turn: u32,
    pub fish_map: HashMap<Team, u32>
}

impl From<xml::state::State> for State {
    fn from(state: xml::state::State) -> Self {
        let fish_map = HashMap::from([
            (Team::One, state.fishes.entries[0].0),
            (Team::Two, state.fishes.entries[1].0)
        ]);
        Self {
            turn: state.turn,
            fish_map 
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_state_from_xml_state() {
        let state = xml::state::State {
            turn: 5,
            start_team: xml::common::Team::One,
            board: xml::state::Board {
                rows: vec![],
            },
            fishes: xml::state::Fishes {
                entries: vec![
                    xml::state::FishEntry(6),
                    xml::state::FishEntry(9)
                ]
            },
        };
        let expected = State {
            turn: 5,
            fish_map: HashMap::from([
                (Team::One, 6),
                (Team::Two, 9)
            ])
        };
        let actual = State::from(state);
        assert_eq!(expected, actual);
    }
}
