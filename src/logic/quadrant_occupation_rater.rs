use rostware23_lib::game::state::State;
use super::Rater;
use rostware23_lib::game::common::Coordinate;
use rostware23_lib::game::board::Board;
use rostware23_lib::game::moves::Move;
use rostware23_lib::game::common::Team;

pub struct QuadrantOccupationRater {}

fn get_quadrant_index(coord: &Coordinate) -> usize {
    let mut result = 0;
    if coord.x() > 7 {
        result += 1;
    }
    if coord.y() > 4 {
        result += 2;
    }
    result
}

fn get_rating_for_team(state: &State, team: Team) -> i32 {
    let mut visited_quadrants = [false; 4];
    let penguins = state.board.get_penguin_iterator(team);
    for penguin in penguins {
        let index = get_quadrant_index(&penguin.coordinate);
        visited_quadrants[index] = true;
    }
    let mut result: i32 = 0;
    for quadrant_visited in visited_quadrants {
        if quadrant_visited {
            result += 1;
        }
    }
    result
}

impl Rater for QuadrantOccupationRater {
    fn rate(state: &State) -> i32 {
        get_rating_for_team(state, state.current_team()) - get_rating_for_team(state, state.current_team().opponent())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_quadrant_index_top_left() {
        let top_left_coord = Coordinate::new(3, 1);
        let index = get_quadrant_index(&top_left_coord);
        assert_eq!(0, index);
    }

    #[test]
    fn get_quadrant_index_bottom_left() {
        let bottom_left_coord = Coordinate::new(4, 6);
        let index = get_quadrant_index(&bottom_left_coord);
        assert_eq!(2, index);
    }

    #[test]
    fn get_quadrant_index_bottom_right() {
        let bottom_left_coord = Coordinate::new(13, 5);
        let index = get_quadrant_index(&bottom_left_coord);
        assert_eq!(3, index);
    }

    #[test]
    fn get_quadrant_index_top_right() {
        let bottom_left_coord = Coordinate::new(12, 2);
        let index = get_quadrant_index(&bottom_left_coord);
        assert_eq!(1, index);
    }

    fn board_with_three_to_one_occupied_quadrants() -> Board {
        let mut board = Board::empty();
        board.perform_move(Move::Place(Coordinate::new(3, 1)), Team::One).unwrap();
        board.perform_move(Move::Place(Coordinate::new(4, 6)), Team::One).unwrap();
        board.perform_move(Move::Place(Coordinate::new(12, 2)), Team::One).unwrap();
        board.perform_move(Move::Place(Coordinate::new(13, 5)), Team::Two).unwrap();
        return board;
    }

    #[test]
    fn get_rating_for_team_one() {
        let state = State::from_initial_board_with_start_team_one(board_with_three_to_one_occupied_quadrants());
        assert_eq!(3, get_rating_for_team(&state, Team::One));
    }

    #[test]
    fn get_rating_for_team_two() {
        let state = State::from_initial_board_with_start_team_one(board_with_three_to_one_occupied_quadrants());
        assert_eq!(1, get_rating_for_team(&state, Team::Two));
    }

    #[test]
    fn rate_for_team_one() {
        let state = State::from_initial_board_with_start_team_one(board_with_three_to_one_occupied_quadrants());
        assert_eq!(2, QuadrantOccupationRater::rate(&state));
    }
}
