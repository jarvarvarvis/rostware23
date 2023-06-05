use rostware23_lib::game::board::{Board, FieldState};
use rostware23_lib::game::common::{Coordinate, Team};
use rostware23_lib::game::moves::Move;

pub fn parse_board(board_string: &str) -> Board {
    let mut board = Board::empty();
    let mut x = 0;
    let mut y = 0;
    for c in board_string.chars() {
        let coord = Coordinate::new(x, y);
        match c {
            'G' => board.perform_move(Move::Place(coord), Team::One).unwrap(),
            'P' => board.perform_move(Move::Place(coord), Team::Two).unwrap(),
            '-' => board.set(coord, FieldState::Fish(1)).unwrap(),
            '=' => board.set(coord, FieldState::Fish(2)).unwrap(),
            '3' => board.set(coord, FieldState::Fish(3)).unwrap(),
            '4' => board.set(coord, FieldState::Fish(4)).unwrap(),
            _ => {}
        }
        x += 1;
        if c == '\n' {
            y += 1;
            x = 0;
        }
    }
    board
}

#[test]
fn board_parser_manual_test() {
    let board_string =
        "4 3 3 =   = = 3\n\
         - = P = = G - =\n\
        - = G = - - P -\n\
         = =     = - = -\n\
        - = - =     = =\n\
         - G - - = P = -\n\
        = - - = = - = -\n\
         3 = =   = 3 3 4\n";
    let board = parse_board(board_string);
    println!("{}", board);
}