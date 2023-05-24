use super::board::*;
use super::common::*;
use super::direction::*;
use super::moves::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Penguin {
    pub coordinate: Coordinate,
    pub team: Team,
}

pub struct CoordinatesInDirectionIterator {
    current_coordinate: Coordinate,
    direction: Direction,
}

impl CoordinatesInDirectionIterator {
    pub fn from(current_coordinate: Coordinate, direction: Direction) -> Self {
        Self {
            current_coordinate,
            direction,
        }
    }
}

impl Iterator for CoordinatesInDirectionIterator {
    type Item = Coordinate;

    fn next(&mut self) -> Option<Self::Item> {
        // Skip the start coordinate.
        // If we would return the previous coordinate instead, PenguinPossibleMoveIterator would
        // get the start coordinate and fail by generating a possible move from that start
        // coordinate to itself, thus yielding no possible moves at all.
        let next_coordinate = self.current_coordinate.add(self.direction.clone().vector());
        if next_coordinate.is_valid() {
            self.current_coordinate = next_coordinate.clone();
            Some(next_coordinate)
        } else {
            None
        }
    }
}

pub struct PenguinPossibleMoveIterator {
    start_coordinate: Coordinate,
    board: Board,
    direction_iterator: DirectionIterator,
    coordinates_iterator: Option<CoordinatesInDirectionIterator>,
}

impl PenguinPossibleMoveIterator {
    pub fn from(penguin: Penguin, board: Board) -> Self {
        Self {
            start_coordinate: penguin.coordinate.clone(),
            board,
            direction_iterator: DirectionIterator::new(),
            coordinates_iterator: None,
        }
    }

    fn next_direction(&mut self) -> Option<()> {
        let next_direction = self.direction_iterator.next()?;
        self.coordinates_iterator = Some(CoordinatesInDirectionIterator::from(
            self.start_coordinate.clone(),
            next_direction,
        ));
        Some(())
    }
}

impl Iterator for PenguinPossibleMoveIterator {
    type Item = Move;

    fn next(&mut self) -> Option<Self::Item> {
        // Advance the direction iterator and init the coordinates iterator if it is not initialized
        if self.coordinates_iterator.is_none() {
            self.next_direction()?;
        }

        let coordinate_iterator = self.coordinates_iterator.as_mut().unwrap();
        let to_coordinate = coordinate_iterator.next();

        // Advance the direction iterator if the current coordinate iterator is done,
        // and then advance the move iterator
        if to_coordinate.is_none() {
            self.next_direction()?;
            return self.next();
        }
        let to_coordinate = to_coordinate.unwrap();

        // Advance the direction iterator if the next move is not valid on the board,
        // and then advance the move iterator
        let can_move_to = self.board.can_move_to(to_coordinate.clone()).ok()?;
        if !can_move_to {
            self.next_direction()?;
            return self.next();
        }

        Some(Move::Normal {
            from: self.start_coordinate.clone(),
            to: to_coordinate,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn iterator_returns_leftwards_coordinates_when_on_right_side_position() {
        let mut coordinates_iterator =
            CoordinatesInDirectionIterator::from(Coordinate::new(14, 4), Direction::Left);
        assert_eq!(Coordinate::new(12, 4), coordinates_iterator.next().unwrap());
        assert_eq!(Coordinate::new(10, 4), coordinates_iterator.next().unwrap());
        assert_eq!(Coordinate::new(8, 4), coordinates_iterator.next().unwrap());
        assert_eq!(Coordinate::new(6, 4), coordinates_iterator.next().unwrap());
        assert_eq!(Coordinate::new(4, 4), coordinates_iterator.next().unwrap());
        assert_eq!(Coordinate::new(2, 4), coordinates_iterator.next().unwrap());
        assert_eq!(Coordinate::new(0, 4), coordinates_iterator.next().unwrap());
        assert_eq!(None, coordinates_iterator.next());
    }

    #[test]
    fn iterator_returns_up_right_coordinates_when_on_left_side_position() {
        let mut coordinates_iterator =
            CoordinatesInDirectionIterator::from(Coordinate::new(1, 3), Direction::TopRight);
        assert_eq!(Coordinate::new(2, 4), coordinates_iterator.next().unwrap());
        assert_eq!(Coordinate::new(3, 5), coordinates_iterator.next().unwrap());
        assert_eq!(Coordinate::new(4, 6), coordinates_iterator.next().unwrap());
        assert_eq!(Coordinate::new(5, 7), coordinates_iterator.next().unwrap());
        assert_eq!(None, coordinates_iterator.next());
    }

    #[test]
    fn penguin_possible_move_iterator_returns_no_moves_on_empty_board() {
        let board = Board::empty();
        let penguin = Penguin {
            coordinate: Coordinate::new(4, 4),
            team: Team::One,
        };
        let mut possible_move_iterator = PenguinPossibleMoveIterator::from(penguin, board);
        assert_eq!(None, possible_move_iterator.next());
    }

    #[test]
    fn penguin_possible_move_iterator_returns_expected_moves_on_all_1_fish_board() {
        let board = Board::fill(FieldState::Fish(1));
        let penguin_coord = Coordinate::new(4, 4);
        let penguin = Penguin {
            coordinate: penguin_coord.clone(),
            team: Team::One,
        };
        let mut possible_move_iterator = PenguinPossibleMoveIterator::from(penguin, board);

        // Left
        assert_eq!(
            Move::Normal {
                from: penguin_coord.clone(),
                to: Coordinate::new(2, 4)
            },
            possible_move_iterator.next().unwrap()
        );
        assert_eq!(
            Move::Normal {
                from: penguin_coord.clone(),
                to: Coordinate::new(0, 4)
            },
            possible_move_iterator.next().unwrap()
        );

        // TopLeft
        assert_eq!(
            Move::Normal {
                from: penguin_coord.clone(),
                to: Coordinate::new(3, 5)
            },
            possible_move_iterator.next().unwrap()
        );
        assert_eq!(
            Move::Normal {
                from: penguin_coord.clone(),
                to: Coordinate::new(2, 6)
            },
            possible_move_iterator.next().unwrap()
        );
        assert_eq!(
            Move::Normal {
                from: penguin_coord.clone(),
                to: Coordinate::new(1, 7)
            },
            possible_move_iterator.next().unwrap()
        );

        // TopRight
        assert_eq!(
            Move::Normal {
                from: penguin_coord.clone(),
                to: Coordinate::new(5, 5)
            },
            possible_move_iterator.next().unwrap()
        );
        assert_eq!(
            Move::Normal {
                from: penguin_coord.clone(),
                to: Coordinate::new(6, 6)
            },
            possible_move_iterator.next().unwrap()
        );
        assert_eq!(
            Move::Normal {
                from: penguin_coord.clone(),
                to: Coordinate::new(7, 7)
            },
            possible_move_iterator.next().unwrap()
        );

        // Right
        assert_eq!(
            Move::Normal {
                from: penguin_coord.clone(),
                to: Coordinate::new(6, 4)
            },
            possible_move_iterator.next().unwrap()
        );
        assert_eq!(
            Move::Normal {
                from: penguin_coord.clone(),
                to: Coordinate::new(8, 4)
            },
            possible_move_iterator.next().unwrap()
        );
        assert_eq!(
            Move::Normal {
                from: penguin_coord.clone(),
                to: Coordinate::new(10, 4)
            },
            possible_move_iterator.next().unwrap()
        );
        assert_eq!(
            Move::Normal {
                from: penguin_coord.clone(),
                to: Coordinate::new(12, 4)
            },
            possible_move_iterator.next().unwrap()
        );
        assert_eq!(
            Move::Normal {
                from: penguin_coord.clone(),
                to: Coordinate::new(14, 4)
            },
            possible_move_iterator.next().unwrap()
        );

        // BottomRight
        assert_eq!(
            Move::Normal {
                from: penguin_coord.clone(),
                to: Coordinate::new(5, 3)
            },
            possible_move_iterator.next().unwrap()
        );
        assert_eq!(
            Move::Normal {
                from: penguin_coord.clone(),
                to: Coordinate::new(6, 2)
            },
            possible_move_iterator.next().unwrap()
        );
        assert_eq!(
            Move::Normal {
                from: penguin_coord.clone(),
                to: Coordinate::new(7, 1)
            },
            possible_move_iterator.next().unwrap()
        );
        assert_eq!(
            Move::Normal {
                from: penguin_coord.clone(),
                to: Coordinate::new(8, 0)
            },
            possible_move_iterator.next().unwrap()
        );

        // BottomLeft
        assert_eq!(
            Move::Normal {
                from: penguin_coord.clone(),
                to: Coordinate::new(3, 3)
            },
            possible_move_iterator.next().unwrap()
        );
        assert_eq!(
            Move::Normal {
                from: penguin_coord.clone(),
                to: Coordinate::new(2, 2)
            },
            possible_move_iterator.next().unwrap()
        );
        assert_eq!(
            Move::Normal {
                from: penguin_coord.clone(),
                to: Coordinate::new(1, 1)
            },
            possible_move_iterator.next().unwrap()
        );
        assert_eq!(
            Move::Normal {
                from: penguin_coord.clone(),
                to: Coordinate::new(0, 0)
            },
            possible_move_iterator.next().unwrap()
        );

        assert_eq!(None, possible_move_iterator.next());
    }

    #[test]
    fn penguin_possible_move_iterator_returns_only_one_possible_move_when_penguin_is_on_2_tile_island(
    ) {
        let mut board = Board::empty();
        let penguin_coord = Coordinate::new(11, 3);
        board
            .set(penguin_coord.clone(), FieldState::Team(Team::One))
            .unwrap();
        let target_coord = Coordinate::new(9, 3);
        board
            .set(target_coord.clone(), FieldState::Fish(3))
            .unwrap();

        let penguin = Penguin {
            coordinate: penguin_coord.clone(),
            team: Team::One,
        };
        let mut possible_move_iterator = PenguinPossibleMoveIterator::from(penguin, board);

        assert_eq!(
            Move::Normal {
                from: penguin_coord,
                to: target_coord
            },
            possible_move_iterator.next().unwrap()
        );
        assert_eq!(None, possible_move_iterator.next());
    }
}
