use crate::xml;
pub use crate::xml::state::FieldState;

use super::moves::*;
use super::common::*;
use super::board_bitset::*;
use super::penguin::*;
use super::penguin_collection::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Board {
    if_fish_field_then_fish_count_higher_than_two_otherwise_penguin_team: Bitset8x8,
    if_fish_field_then_fish_modulo_2_otherwise_penguin_count: Bitset8x8,
    non_zero_fish_count: Bitset8x8,
    penguin_collection: PenguinCollection
}

fn get_fish_higher_than_two_or_penguin_team_for_field(field_state: &FieldState) -> bool {
    if let FieldState::Fish(fish_count) = field_state {
        *fish_count > 2
    } else {
        field_state == &FieldState::Team(Team::Two)
    }
}

fn get_fish_modulo_2_equals_1_or_is_penguin(field_state: &FieldState) -> bool {
    match field_state {
        FieldState::Empty => false,
        FieldState::Fish(fish_count) => *fish_count % 2 == 1,
        FieldState::Team(_) => true,
    }
}

fn get_non_zero_fish_count(field_state: &FieldState) -> bool {
    if let FieldState::Fish(fish_count) = field_state {
        *fish_count > 0
    } else {
        false 
    }
}

impl Board {
    pub fn empty() -> Self {
        Self {
            if_fish_field_then_fish_count_higher_than_two_otherwise_penguin_team: Bitset8x8::empty(),
            if_fish_field_then_fish_modulo_2_otherwise_penguin_count: Bitset8x8::empty(),
            non_zero_fish_count: Bitset8x8::empty(),
            penguin_collection: PenguinCollection::empty()
        }
    }

    pub fn fill(field_state: FieldState) -> Self {
        let mut board = Board::empty();

        for y in 0..BOARD_HEIGHT {
            for x in 0..BOARD_WIDTH {
                board.set(Coordinate::new(x, y).odd_r_to_doubled(), field_state.clone()).unwrap();
            }
        }

        board
    }

    pub fn get(&self, at: Coordinate) -> anyhow::Result<FieldState> {
        let x = at.x() / 2;
        let y = at.y();
        let is_fish_field = self.non_zero_fish_count.get(x.into(), y.into())?;
        if is_fish_field {
            let greater_than_two = self.if_fish_field_then_fish_count_higher_than_two_otherwise_penguin_team.get(x.into(), y.into())?;
            let odd = self.if_fish_field_then_fish_modulo_2_otherwise_penguin_count.get(x.into(), y.into())?;
            let fish_count = if greater_than_two { 3 } else { 1 } + if odd { 0 } else { 1 };
            return Ok(FieldState::Fish(fish_count));
        }
        let is_penguin = self.if_fish_field_then_fish_modulo_2_otherwise_penguin_count.get(x.into(), y.into())?;
        if is_penguin {
            let penguin_in_team_two = self.if_fish_field_then_fish_count_higher_than_two_otherwise_penguin_team.get(x.into(), y.into())?;
            return Ok(FieldState::Team(
                if penguin_in_team_two { Team::Two } else { Team::One }
            ))
        }
        Ok(FieldState::Empty)
    }

    pub fn can_move_to(&self, to: Coordinate) -> anyhow::Result<bool> {
        match self.get(to.clone())? {
            FieldState::Fish(_) => Ok(true),
            _ => Ok(false)
        }
    }

    pub fn set(&mut self, at: Coordinate, field_state: FieldState) -> anyhow::Result<()> {
        let x = at.x() / 2;
        let y = at.y();
        self.if_fish_field_then_fish_count_higher_than_two_otherwise_penguin_team
            .set(x.into(), y.into(), get_fish_higher_than_two_or_penguin_team_for_field(&field_state))?;
        self.if_fish_field_then_fish_modulo_2_otherwise_penguin_count
            .set(x.into(), y.into(), get_fish_modulo_2_equals_1_or_is_penguin(&field_state))?;
        self.non_zero_fish_count
            .set(x.into(), y.into(), get_non_zero_fish_count(&field_state))?;
        Ok(())
    }
    
    fn perform_place_move(&mut self, to: Coordinate, team: Team) -> anyhow::Result<()> {
        self.set(to.clone(), FieldState::Team(team.clone()))?;
        self.penguin_collection.add_penguin(Penguin {
            coordinate: to,
            team
        });
        Ok(())
    }

    fn perform_normal_move(&mut self, from: Coordinate, to: Coordinate, team: Team) -> anyhow::Result<()> {
        self.set(from.clone(), FieldState::Empty)?;
        self.set(to.clone(), FieldState::Team(team.clone()))?;
        self.penguin_collection.move_penguin(Penguin {
            coordinate: from,
            team
        }, to)?;
        Ok(())
    }

    pub fn perform_move(&mut self, performed_move: Move, team: Team) -> anyhow::Result<()> {
        match performed_move {
            Move::Place(to) => self.perform_place_move(to, team),
            Move::Normal { from, to } => self.perform_normal_move(from, to, team),
        }
    }

    pub fn with_move_performed(&self, performed_move: Move, team: Team) -> anyhow::Result<Self> {
        let mut new_state = self.clone();
        new_state.perform_move(performed_move, team)?;
        Ok(new_state)
    }

    pub fn get_penguin_iterator(&self, team: Team) -> impl Iterator<Item = Penguin> {
        self.penguin_collection.get_iter_for_team(team)
    }
}

fn convert_field_to_string(field_state: &FieldState) -> String {
    match field_state {
        FieldState::Empty => "\u{001B}[46m ",
        FieldState::Fish(fish_count) => match fish_count {
            1 => "-",
            2 => "=",
            3 => "3",
            4 => "4",
            _ => unreachable!()
        },
        FieldState::Team(team) => match team {
            Team::One => "\u{001B}[32m1",
            Team::Two => "\u{001B}[35m2",
        },
    }.to_string()
}

impl std::fmt::Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..BOARD_HEIGHT {
            let no_field_to_string = "\u{001b}[0m ";
            if y % 2 == 1 {
                write!(f, "{no_field_to_string}")?;
            }

            for x in 0..BOARD_WIDTH {
                let field = self.get(Coordinate::new(x, y).odd_r_to_doubled()).unwrap();
                write!(f, "{}", convert_field_to_string(&field))?;
                write!(f, "{}", no_field_to_string)?;
            }
            if y != BOARD_HEIGHT - 1 {
                writeln!(f)?;
            }
        }
        Ok(())
    }
}

impl From<xml::state::Board> for Board {
    fn from(xml_board: xml::state::Board) -> Self {
        let mut board = Board::empty();

        for y in 0..BOARD_HEIGHT {
            for x in 0..BOARD_WIDTH {
                let board_row = &xml_board.rows[y as usize];
                let field = &board_row.fields[x as usize];
                let field_state = &field.0;

                board.set(Coordinate::new(x, y).odd_r_to_doubled(), field_state.clone()).unwrap();
                if let FieldState::Team(team) = field_state {
                    board.penguin_collection.add_penguin(Penguin {
                        coordinate: Coordinate::new(x, y).odd_r_to_doubled(),
                        team: team.clone()
                    });
                }
            }
        }

        board
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn place_moves_update_correct_fields() {
        let mut board = Board::fill(FieldState::Fish(2));
        let move_1 = Move::Place(Coordinate::new(2, 4));
        board.perform_move(move_1, Team::One).unwrap();
        let move_2 = Move::Place(Coordinate::new(7, 1));
        board.perform_move(move_2, Team::Two).unwrap();

        let mut expected_penguin_collection = PenguinCollection::empty();
        expected_penguin_collection.add_penguin(Penguin {
            coordinate: Coordinate::new(2, 4),
            team: Team::One,
        }); 
        expected_penguin_collection.add_penguin(Penguin {
            coordinate: Coordinate::new(7, 1),
            team: Team::Two,
        }); 

        assert_eq!(board.get(Coordinate::new(2, 4)).unwrap(), FieldState::Team(Team::One));
        assert_eq!(board.get(Coordinate::new(7, 1)).unwrap(), FieldState::Team(Team::Two));

        assert_eq!(board.get(Coordinate::new(3, 5)).unwrap(), FieldState::Fish(2));

        assert_eq!(expected_penguin_collection, board.penguin_collection);
    }

    #[test]
    fn penguin_collection_iterator_on_updated_board_is_correct() {
        let mut board = Board::fill(FieldState::Fish(2));
        let coord_1 = Coordinate::new(2, 4);
        board.perform_move(Move::Place(coord_1.clone()), Team::One).unwrap();
        let coord_2 = Coordinate::new(7, 1);
        board.perform_move(Move::Place(coord_2.clone()), Team::One).unwrap();
        let move_2 = Move::Place(Coordinate::new(11, 7));
        board.perform_move(move_2, Team::Two).unwrap();

        let mut penguin_iterator = board.penguin_collection.get_iter_for_team(Team::One);
        assert_eq!(Penguin { coordinate: coord_1.clone(), team: Team::One }, penguin_iterator.next().unwrap());
        assert_eq!(Penguin { coordinate: coord_2.clone(), team: Team::One }, penguin_iterator.next().unwrap());
        assert_eq!(None, penguin_iterator.next());
    }

    #[test]
    fn penguin_collection_iterator_on_updated_board_has_correct_size() {
        let mut board = Board::fill(FieldState::Fish(2));
        let coord_1 = Coordinate::new(2, 4);
        board.perform_move(Move::Place(coord_1.clone()), Team::One).unwrap();
        let coord_2 = Coordinate::new(7, 1);
        board.perform_move(Move::Place(coord_2.clone()), Team::One).unwrap();
        let move_2 = Move::Place(Coordinate::new(11, 7));
        board.perform_move(move_2, Team::Two).unwrap();

        let penguin_iterator = board.penguin_collection.get_iter_for_team(Team::One);
        assert_eq!(2, penguin_iterator.count());
    }

    #[test]
    fn penguin_collection_iterator_on_board_with_all_penguins_is_correct() {
        let mut board = Board::empty();
        board.set(Coordinate::new(0, 0), FieldState::Fish(2)).unwrap();
        board.perform_move(Move::Place(Coordinate::new(2, 0)), Team::One).unwrap();
        board.perform_move(Move::Place(Coordinate::new(4, 0)), Team::One).unwrap();
        board.perform_move(Move::Place(Coordinate::new(6, 0)), Team::One).unwrap();
        board.set(Coordinate::new(8, 0), FieldState::Fish(2)).unwrap();
        board.perform_move(Move::Place(Coordinate::new(12, 0)), Team::One).unwrap();
        
        board.perform_move(Move::Place(Coordinate::new(2, 2)), Team::Two).unwrap();
        board.perform_move(Move::Place(Coordinate::new(4, 2)), Team::Two).unwrap();
        board.perform_move(Move::Place(Coordinate::new(6, 2)), Team::Two).unwrap();
        board.perform_move(Move::Place(Coordinate::new(8, 2)), Team::Two).unwrap();
        
        let mut team_one_penguins = board.penguin_collection.get_iter_for_team(Team::One);
        assert_eq!(Penguin { coordinate: Coordinate::new(2, 0), team: Team::One }, team_one_penguins.next().unwrap());
        assert_eq!(Penguin { coordinate: Coordinate::new(4, 0), team: Team::One }, team_one_penguins.next().unwrap());
        assert_eq!(Penguin { coordinate: Coordinate::new(6, 0), team: Team::One }, team_one_penguins.next().unwrap());
        assert_eq!(Penguin { coordinate: Coordinate::new(12, 0), team: Team::One }, team_one_penguins.next().unwrap());
        assert_eq!(None, team_one_penguins.next());

        let mut team_two_penguins = board.penguin_collection.get_iter_for_team(Team::Two);
        assert_eq!(Penguin { coordinate: Coordinate::new(2, 2), team: Team::Two }, team_two_penguins.next().unwrap());
        assert_eq!(Penguin { coordinate: Coordinate::new(4, 2), team: Team::Two }, team_two_penguins.next().unwrap());
        assert_eq!(Penguin { coordinate: Coordinate::new(6, 2), team: Team::Two }, team_two_penguins.next().unwrap());
        assert_eq!(Penguin { coordinate: Coordinate::new(8, 2), team: Team::Two }, team_two_penguins.next().unwrap());
        assert_eq!(None, team_two_penguins.next());
    }

    #[test]
    fn can_move_to_non_player_field() {
        let mut board = Board::fill(FieldState::Fish(2));
        board.perform_move(Move::Place(Coordinate::new(0, 0)), Team::One).unwrap();
        assert!(board.can_move_to(Coordinate::new(1, 3)).unwrap());
    }

    #[test]
    fn can_not_move_to_player_field() {
        let mut board = Board::fill(FieldState::Fish(2));
        board.perform_move(Move::Place(Coordinate::new(0, 0)), Team::One).unwrap();
        board.perform_move(Move::Place(Coordinate::new(1, 3)), Team::Two).unwrap();
        assert!(!board.can_move_to(Coordinate::new(1, 3)).unwrap());
    }
}
