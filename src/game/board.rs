use crate::xml;
pub use crate::xml::state::FieldState;

use super::moves::*;
use super::common::*;
use super::board_bitset::*;
use super::penguin::*;
use super::penguin_collection::*;

pub const BOARD_WIDTH: u64 = 8;
pub const BOARD_HEIGHT: u64 = 8;

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

    fn perform_move(&mut self, performed_move: Move, team: Team) -> anyhow::Result<()> {
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
            writeln!(f)?;
        }
        Ok(())
    }
}

impl From<xml::state::Board> for Board {
    fn from(xml_board: xml::state::Board) -> Self {
        let mut board = Board::empty();

        for y in 0..BOARD_HEIGHT {
            for x in 0..BOARD_WIDTH {
                let board_row = &xml_board.rows[x as usize];
                let field = &board_row.fields[y as usize];
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
    fn place_move_updates_correct_field() {
        let mut board = Board::fill(FieldState::Fish(2));
        let r#move_1 = Move::Place(Coordinate::new(2, 4));
        board = board.with_move_performed(r#move_1, Team::One).unwrap();
        let r#move_2 = Move::Place(Coordinate::new(7, 1));
        board = board.with_move_performed(r#move_2, Team::Two).unwrap();

        assert_eq!(board.get(Coordinate::new(2, 4)).unwrap(), FieldState::Team(Team::One));
        assert_eq!(board.get(Coordinate::new(7, 1)).unwrap(), FieldState::Team(Team::Two));
    }
}
