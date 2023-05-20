use super::common::*;
use super::board::Board;
use super::move_generator::MoveGenerator;
use super::moves::Move;
use super::possible_moves::PossibleMovesIterator;
use super::result::{GameResult, TeamAndPoints};

use crate::xml;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct State {
    pub turn: u32,
    pub start_team: Team,
    pub team_one_fish: u32,
    pub team_two_fish: u32,
    pub board: Board
}

impl State {
    pub fn from_initial_board_with_start_team_one(board: Board) -> Self {
        Self {
            turn: 0,
            start_team: Team::One,
            team_one_fish: 0,
            team_two_fish: 0,
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

    pub fn has_team_any_moves(&self, team: Team) -> bool {
        PossibleMovesIterator::from_state_and_team(self.clone(), team)
            .next()
            .is_some()
    }

    pub fn is_over(&self) -> bool {
        return !self.has_team_any_moves(Team::One) && !self.has_team_any_moves(Team::Two)
    }

    pub fn current_team(&self) -> anyhow::Result<Team> {
        let assumed_current_team = self.turn_based_current_team();
        let opponent = assumed_current_team.opponent();
        Ok(assumed_current_team)
    }

    pub fn score_of_team(&self, team: Team) -> u32 {
        match team {
            Team::One => self.team_one_fish,
            Team::Two => self.team_two_fish
        }
    }

    fn score_for_team_after_move(&self, score_team: Team, performed_move: &Move) -> anyhow::Result<u32> {
        let target_field = self.board.get(performed_move.get_to())?;
        let added_points = target_field.get_fish_count()?;
        let current_team = self.current_team()?;
        let initial_score = self.score_of_team(score_team);
        if score_team == current_team {
            return Ok(initial_score + added_points);
        }
        Ok(initial_score)
    }

    pub fn perform_move(&mut self, performed_move: Move) -> anyhow::Result<()> {
        let new_team_one_score = self.score_for_team_after_move(Team::One, &performed_move)?;
        let new_team_two_score = self.score_for_team_after_move(Team::Two, &performed_move)?;
        let current_team = self.current_team()?;
        let new_board = self.board.with_move_performed(performed_move, current_team)?;
        self.turn = self.turn + 1;
        self.team_one_fish = new_team_one_score;
        self.team_two_fish = new_team_two_score;
        self.board = new_board;
        Ok(())
    }

    pub fn with_move_performed(&self, performed_move: Move) -> anyhow::Result<Self> {
        let mut self_clone = self.clone();
        self_clone.perform_move(performed_move)?;
        Ok(self_clone)
    }

    pub fn possible_moves(&self) -> impl Iterator<Item = Move> {
        self.possible_moves_by_move_generator::<PossibleMovesIterator>()
    }

    pub fn possible_moves_by_move_generator<Generator: MoveGenerator>(&self) -> impl Iterator<Item = Move> {
        Generator::get_possible_moves(self.clone())
    }

    pub fn with_moveless_player_skipped(&self) -> anyhow::Result<Self> {
        if self.has_team_any_moves(self.current_team()?) {
            return Ok(self.clone());
        }
        if self.has_team_any_moves(self.current_team()?.opponent()) {
            return Ok(Self {
                turn: self.turn + 1,
                start_team: self.start_team,
                team_one_fish: self.team_one_fish,
                team_two_fish: self.team_two_fish,
                board: self.board.clone()
            });
        }
        anyhow::bail!("Can't skip moveless player when nobody has moves");
    }

    pub fn get_result(&self) -> anyhow::Result<GameResult> {
        if !self.is_over() {
            anyhow::bail!("The game state is not over yet");
        }

        let winner;
        if self.team_one_fish > self.team_two_fish {
            winner = Some(Team::One);
        } else if self.team_two_fish > self.team_one_fish {
            winner = Some(Team::Two);
        } else {
            winner = None
        }

        Ok(GameResult {
            winner,
            points: (TeamAndPoints::new(Team::One, self.team_one_fish), TeamAndPoints::new(Team::Two, self.team_two_fish)),
        })
    }
}

impl From<xml::state::State> for State {
    fn from(state: xml::state::State) -> Self {
        Self {
            turn: state.turn,
            start_team: state.start_team,
            team_one_fish: state.fishes.entries[0].0,
            team_two_fish: state.fishes.entries[1].0,
            board: Board::from(state.board)
        }
    }
}

impl std::fmt::Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Board:\n{}\nCurrent turn: {}, Fish: {} / {}",
               self.board,
               self.turn,
               self.score_of_team(Team::One),
               self.score_of_team(Team::Two))
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
            team_one_fish: 6,
            team_two_fish: 9,
            board: Board::empty()
        };
        let actual = State::from(state);
        assert_eq!(expected, actual);
    }

    #[test]
    fn turn_based_current_team_on_odd_turn() {
        let state = State {
            turn: 7,
            start_team: Team::One,
            team_one_fish: 0,
            team_two_fish: 0,
            board: Board::empty()
        };
        assert_eq!(Team::Two, state.turn_based_current_team());
    }

    #[test]
    fn turn_based_current_team_on_even_turn() {
        let state = State {
            turn: 2,
            start_team: Team::One,
            team_one_fish: 0,
            team_two_fish: 0,
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
    fn team_one_on_empty_state() {
        let state = State::from_initial_board_with_start_team_one(Board::empty());
        assert_eq!(Team::One, state.current_team().unwrap());
    }


    #[test]
    fn one_team_has_moves_and_other_team_has_none_on_test_state() {
        let mut board = Board::empty();
        board.perform_move(Move::Place(Coordinate::new(2, 0)), Team::One).unwrap();
        board.perform_move(Move::Place(Coordinate::new(4, 0)), Team::One).unwrap();
        board.perform_move(Move::Place(Coordinate::new(6, 0)), Team::One).unwrap();
        board.perform_move(Move::Place(Coordinate::new(12, 0)), Team::One).unwrap();

        board.perform_move(Move::Place(Coordinate::new(2, 2)), Team::Two).unwrap();
        board.perform_move(Move::Place(Coordinate::new(4, 2)), Team::Two).unwrap();
        board.perform_move(Move::Place(Coordinate::new(6, 2)), Team::Two).unwrap();
        board.perform_move(Move::Place(Coordinate::new(8, 2)), Team::Two).unwrap();
        board.set(Coordinate::new(10, 2), FieldState::Fish(2)).unwrap();
        
        let state = State::from_initial_board_with_start_team_one(board);
        assert!(!state.has_team_any_moves(Team::One));
        assert!(state.has_team_any_moves(Team::Two));
    }

    #[test]
    fn current_team_on_even_turn_is_same_team_when_start_team_has_no_moves() {
        let mut board = Board::empty();
        board.perform_move(Move::Place(Coordinate::new(2, 0)), Team::One).unwrap();
        board.perform_move(Move::Place(Coordinate::new(4, 0)), Team::One).unwrap();
        board.perform_move(Move::Place(Coordinate::new(6, 0)), Team::One).unwrap();
        board.perform_move(Move::Place(Coordinate::new(12, 0)), Team::One).unwrap();
        
        board.perform_move(Move::Place(Coordinate::new(2, 2)), Team::Two).unwrap();
        board.perform_move(Move::Place(Coordinate::new(4, 2)), Team::Two).unwrap();
        board.perform_move(Move::Place(Coordinate::new(6, 2)), Team::Two).unwrap();
        board.perform_move(Move::Place(Coordinate::new(8, 2)), Team::Two).unwrap();
        board.set(Coordinate::new(10, 2), FieldState::Fish(2)).unwrap();
        let mut state = State::from_initial_board_with_start_team_one(board);
        state.turn = 8;
        println!("{}", state);
        assert_eq!(Team::One, state.current_team().unwrap());
    }

    #[test]
    fn perform_move_on_empty_state_does_work_due_to_no_error_checking() {
        let state = State::from_initial_board_with_start_team_one(Board::empty());
        let result = state.with_move_performed(Move::Place(Coordinate::new(5, 7)));
        assert!(result.is_ok());
    }

    #[test]
    fn empty_state_is_over() {
        let state = State::from_initial_board_with_start_team_one(Board::empty());
        assert!(state.is_over());
    }

    #[test]
    fn given_game_state_with_possible_moves_when_skipping_moveless_player_then_do_nothing() {
        let mut board = Board::empty();
        board.set(Coordinate::new(4, 2), FieldState::Fish(1)).unwrap();
        let state_before = State::from_initial_board_with_start_team_one(board);
        let state_after = state_before.with_moveless_player_skipped().unwrap();
        assert_eq!(state_before, state_after);
    }

    #[test]
    fn empty_state_with_moveless_player_skipped_fails() {
        let empty_state = State::from_initial_board_with_start_team_one(Board::empty());
        let state_after = empty_state.with_moveless_player_skipped();
        assert!(state_after.is_err());
    }

    #[test]
    fn given_game_state_without_possible_moves_when_skipping_moveless_player_then_change_team_by_only_changing_turn_count() {
        let mut board = Board::empty();
        board.perform_move(Move::Place(Coordinate::new(2, 2)), Team::Two).unwrap();
        board.perform_move(Move::Place(Coordinate::new(4, 2)), Team::Two).unwrap();
        board.perform_move(Move::Place(Coordinate::new(6, 2)), Team::Two).unwrap();
        board.perform_move(Move::Place(Coordinate::new(8, 2)), Team::Two).unwrap();
        board.set(Coordinate::new(10, 2), FieldState::Fish(2)).unwrap();
        let state_before = State::from_initial_board_with_start_team_one(board.clone());
        let state_expected = State{
            turn: 1,
            board,
            team_one_fish: 0,
            team_two_fish: 0,
            start_team: Team::One
        };
        let state_after = state_before.with_moveless_player_skipped().unwrap();
        assert_eq!(state_expected, state_after);
    }

    #[test]
    fn result_of_empty_state_has_no_winner() {
        let state = State::from_initial_board_with_start_team_one(Board::empty());
        let result = state.get_result().unwrap();
        assert_eq!(None, result.winner);
        assert_eq!(TeamAndPoints::new(Team::One, 0), result.points.0);
        assert_eq!(TeamAndPoints::new(Team::Two, 0), result.points.1);
    }

    #[test]
    fn result_of_empty_state_with_more_team_one_points_has_team_one_as_winner() {
        let state = State {
            turn: 0,
            start_team: Team::One,
            team_one_fish: 20,
            team_two_fish: 0,
            board: Board::empty()
        };
        let result = state.get_result().unwrap();
        assert_eq!(Some(Team::One), result.winner);
        assert_eq!(TeamAndPoints::new(Team::One, 20), result.points.0);
        assert_eq!(TeamAndPoints::new(Team::Two, 0), result.points.1);
    }

    #[test]
    fn result_of_empty_state_with_more_team_two_points_has_team_two_as_winner() {
        let state = State {
            turn: 0,
            start_team: Team::One,
            team_one_fish: 9,
            team_two_fish: 21,
            board: Board::empty()
        };
        let result = state.get_result().unwrap();
        assert_eq!(Some(Team::Two), result.winner);
        assert_eq!(TeamAndPoints::new(Team::One, 9), result.points.0);
        assert_eq!(TeamAndPoints::new(Team::Two, 21), result.points.1);
    }

    #[test]
    fn result_of_not_over_state_is_err() {
        let mut board = Board::empty();
        board.perform_move(Move::Place(Coordinate::new(1, 1)), Team::One).unwrap();
        board.perform_move(Move::Place(Coordinate::new(3, 1)), Team::One).unwrap();
        board.perform_move(Move::Place(Coordinate::new(5, 1)), Team::One).unwrap();
        board.perform_move(Move::Place(Coordinate::new(7, 1)), Team::One).unwrap();
        
        board.perform_move(Move::Place(Coordinate::new(2, 2)), Team::Two).unwrap();
        board.perform_move(Move::Place(Coordinate::new(4, 2)), Team::Two).unwrap();
        board.perform_move(Move::Place(Coordinate::new(6, 2)), Team::Two).unwrap();
        board.set(Coordinate::new(8, 2), FieldState::Fish(2)).unwrap();
        let mut state = State::from_initial_board_with_start_team_one(board);

        let result = state.get_result();
        assert!(result.is_err());
    }
}
