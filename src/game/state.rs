use super::common::*;
use super::board::Board;
use super::moves::Move;
use super::possible_moves::PossibleMovesIterator;

use crate::xml;

use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct State {
    pub turn: u32,
    pub start_team: Team,
    pub fish_map: HashMap<Team, u32>,
    pub board: Board
}

impl State {
    pub fn from_initial_board_with_start_team_one(board: Board) -> Self {
        let fish_map = HashMap::from([
            (Team::One, 0),
            (Team::Two, 0)
        ]);
        Self {
            turn: 0,
            start_team: Team::One,
            fish_map,
            board
        }
    }

    fn turn_based_current_team(&self) -> Team {
        if self.turn % 2 == 0 { 
            self.start_team.clone()
        } else {
            self.start_team.opponent()
        }
    }

    fn has_team_any_moves(&self, team: Team) -> bool {
        PossibleMovesIterator::from_state_and_team(self.clone(), team).count() > 0
    }

    pub fn current_team(&self) -> Team {
        let assumed_current_team = self.turn_based_current_team();
        if !self.has_team_any_moves(assumed_current_team) {
            assumed_current_team.opponent()
        } else {
            assumed_current_team
        }
    }

    pub fn score_of_team(&self, team: Team) -> u32 {
        self.fish_map[&team]
    }

    pub fn with_move_performed(&self, performed_move: Move) -> anyhow::Result<Self> {
        let target_field = self.board.get(performed_move.get_to())?;
        let added_points = target_field.get_fish_count()?;
        let current_team = self.turn_based_current_team();
        let mut new_fish_map = self.fish_map.clone();
        *new_fish_map.get_mut(&current_team).unwrap() += added_points;
        let new_board = self.board.with_move_performed(performed_move, current_team)?;
        Ok(Self {
            turn: self.turn + 1,
            start_team: self.start_team.clone(),
            fish_map: new_fish_map,
            board: new_board
        })
    }

    pub fn possible_moves(&self) -> PossibleMovesIterator {
        PossibleMovesIterator::from(self.clone())
    }
}

impl From<xml::state::State> for State {
    fn from(state: xml::state::State) -> Self {
        let fish_map = HashMap::from([
            (Team::One, state.fishes.entries[0].0),
            (Team::Two, state.fishes.entries[1].0)
        ]);
        Self {
            turn: state.turn,
            start_team: state.start_team,
            fish_map,
            board: Board::from(state.board)
        }
    }
}

impl std::fmt::Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Board:\n{}\nCurrent turn: {}, Fish: {} / {}",
               self.board,
               self.turn,
               self.fish_map[&Team::One],
               self.fish_map[&Team::Two])
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use xml::state::{Field, FieldState, FieldRow, Board as XmlBoard, State as XmlState, Fishes, FishEntry};

    #[test]
    fn empty_state_from_xml_state() {
        let state = XmlState {
            turn: 5,
            start_team: xml::common::Team::One,
            board: XmlBoard {
                rows: vec![
                    FieldRow { fields: vec![ Field(FieldState::Empty); 8 ] },
                    FieldRow { fields: vec![ Field(FieldState::Empty); 8 ] },
                    FieldRow { fields: vec![ Field(FieldState::Empty); 8 ] },
                    FieldRow { fields: vec![ Field(FieldState::Empty); 8 ] },
                    FieldRow { fields: vec![ Field(FieldState::Empty); 8 ] },
                    FieldRow { fields: vec![ Field(FieldState::Empty); 8 ] },
                    FieldRow { fields: vec![ Field(FieldState::Empty); 8 ] },
                    FieldRow { fields: vec![ Field(FieldState::Empty); 8 ] },
                ],
            },
            fishes: Fishes {
                entries: vec![
                    FishEntry(6),
                    FishEntry(9)
                ]
            },
        };
        let expected = State {
            turn: 5,
            start_team: Team::One,
            fish_map: HashMap::from([
                (Team::One, 6),
                (Team::Two, 9)
            ]),
            board: Board::empty()
        };
        let actual = State::from(state);
        assert_eq!(expected, actual);
    }

    #[test]
    fn get_current_team_on_odd_turn() {
        let state = State {
            turn: 7,
            start_team: Team::One,
            fish_map: HashMap::from([
                (Team::One, 0),
                (Team::Two, 0)
            ]),
            board: Board::empty()
        };
        assert_eq!(Team::Two, state.turn_based_current_team());
    }

    #[test]
    fn get_current_team_on_even_turn() {
        let state = State {
            turn: 2,
            start_team: Team::One,
            fish_map: HashMap::from([
                (Team::One, 0),
                (Team::Two, 0)
            ]),
            board: Board::empty()
        };
        assert_eq!(Team::One, state.turn_based_current_team());
    }

    #[test]
    fn no_team_has_possible_moves_on_empty_state() {
        let state = State::from_initial_board_with_start_team_one(Board::empty());
        assert!(!state.has_team_any_moves(Team::One));
        assert!(!state.has_team_any_moves(Team::Two));
    }

    #[test]
    fn current_team_on_even_turn_is_other_team_when_start_team_has_no_moves() {
        let mut board = Board::empty();
        board.perform_move(Move::Place(Coordinate::new(2, 0)), Team::One).unwrap();
        board.perform_move(Move::Place(Coordinate::new(4, 0)), Team::One).unwrap();
        board.perform_move(Move::Place(Coordinate::new(6, 0)), Team::One).unwrap();
        board.perform_move(Move::Place(Coordinate::new(12, 0)), Team::One).unwrap();
        
        board.perform_move(Move::Place(Coordinate::new(2, 2)), Team::Two).unwrap();
        board.perform_move(Move::Place(Coordinate::new(4, 2)), Team::Two).unwrap();
        board.perform_move(Move::Place(Coordinate::new(6, 2)), Team::Two).unwrap();
        board.perform_move(Move::Place(Coordinate::new(8, 2)), Team::Two).unwrap();
        board.set(Coordinate::new(8, 2), FieldState::Fish(2)).unwrap();
        let mut state = State::from_initial_board_with_start_team_one(board);
        state.turn = 8;
        println!("{}", state);
        assert_eq!(Team::Two, state.current_team());
    }

    #[test]
    fn perform_move_on_empty_state() {
        let state = State {
            turn: 0,
            start_team: Team::One,
            fish_map: HashMap::from([
                (Team::One, 0),
                (Team::Two, 0)
            ]),
            board: Board::empty()
        };

        let expected_board = Board::empty()
            .with_move_performed(Move::Place(Coordinate::new(5, 7)), Team::One).unwrap();
        let expected = State {
            turn: 1,
            start_team: Team::One,
            fish_map: HashMap::from([
                (Team::One, 0),
                (Team::Two, 0)
            ]),
            board: expected_board
        };

        let actual = state.with_move_performed(Move::Place(Coordinate::new(5, 7))).unwrap();
        assert_eq!(expected, actual);
    }
}
