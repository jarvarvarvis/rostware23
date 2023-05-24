use rand::prelude::*;

use crate::game::board::*;
use crate::game::common::*;
use crate::game::state::*;
use crate::util::rng::*;

pub const MAX_HOLES_PER_BOARD: u32 = 5;

fn get_inverted_coordinate(coord: &Coordinate) -> Coordinate {
    let x = BOARD_WIDTH * 2 - 1 - coord.x();
    let y = BOARD_HEIGHT - 1 - coord.y();
    Coordinate::new(x, y)
}

// Implemented according to
// https://github.com/software-challenge/backend/blob/ae6b2bd4c0ce2127b29887fa0ce9554769322568/plugin/src/main/kotlin/sc/plugin2023/Board.kt
pub fn create_board_from_seed(seed: u64) -> Board {
    let mut board = Board::empty();

    let mut remaining_fish = (BOARD_WIDTH * BOARD_HEIGHT) as u32;
    let mut rng = XorWow::from_seed(seed);
    let mut current_holes = MAX_HOLES_PER_BOARD;

    for y in 0..BOARD_HEIGHT / 2 {
        for x in 0..BOARD_WIDTH {
            let coordinate = Coordinate::new(x, y).odd_r_to_doubled();
            let inverted_coordinate = get_inverted_coordinate(&coordinate);
            let random = rng.next(0..remaining_fish as u64) as u32;
            if random < current_holes {
                current_holes -= 1;

                // Insert in board's upper and lower half
                board.set(coordinate, FieldState::Empty).unwrap();
                board.set(inverted_coordinate, FieldState::Empty).unwrap();
                continue;
            }

            let fish = (random - current_holes) / 20 + 1;
            remaining_fish -= fish;

            // Insert in board's upper and lower half
            board.set(coordinate, FieldState::Fish(fish)).unwrap();
            board
                .set(inverted_coordinate, FieldState::Fish(fish))
                .unwrap();
        }
    }

    board
}

pub fn create_any_board() -> Board {
    create_board_from_seed(thread_rng().gen::<u64>())
}

pub fn create_any() -> State {
    let board = create_any_board();
    State::from_initial_board_with_start_team_one(board)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_board_from_any_seed_with_logical_fields() {
        let board = create_any_board();

        for y in 0..BOARD_HEIGHT / 2 {
            for x in 0..BOARD_WIDTH {
                let coordinate = Coordinate::new(x, y).odd_r_to_doubled();
                let inverted_coordinate = get_inverted_coordinate(&coordinate);
                assert_eq!(
                    board.get(coordinate).unwrap(),
                    board.get(inverted_coordinate).unwrap()
                );
            }
        }
    }

    #[test]
    fn given_boards_with_seed_0_to_100_then_at_least_one_board_has_field_with_four_fish() {
        let mut boards = vec![];
        for seed in 0..100 {
            boards.push(create_board_from_seed(seed));
        }

        for board in boards {
            for y in 0..BOARD_HEIGHT {
                for x in 0..BOARD_WIDTH {
                    let coordinate = Coordinate::new(x, y).odd_r_to_doubled();
                    let field_state = board.get(coordinate).unwrap();
                    if field_state == FieldState::Fish(4) {
                        assert!(true);
                        return;
                    }
                }
            }
        }
        assert!(false);
    }
}
