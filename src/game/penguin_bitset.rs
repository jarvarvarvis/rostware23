use super::common::*;
use super::penguin::*;

const PENGUIN_COUNT_BIT_MASK: u64 = 0b111;
const PENGUIN_INITIAL_OFFSET: u64 = 3;
const PENGUIN_COORDS_OFFSET: u64 = 6;
const PENGUIN_COORD_OFFSET: u64 = 3;
const PENGUIN_COORDS_BIT_MASK: u64 = 0b111111;
const PENGUIN_COORD_BIT_MASK: u64 = 0b111;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PenguinBitset {
    value: u64
}

impl PenguinBitset {
    pub fn empty() -> Self {
        Self {
            value: 0
        }
    }

    pub fn get_penguin_count(&self) -> u64 {
        self.value & PENGUIN_COUNT_BIT_MASK
    }

    fn add_penguin_at_bit_position(&mut self, coordinates: Coordinate, position: u64) {
        let x = coordinates.x() / 2;
        let y = coordinates.y();
        let combined_coords = ((y << PENGUIN_COORD_OFFSET) | x) & PENGUIN_COORDS_BIT_MASK;
        let position_offset = position * PENGUIN_COORDS_OFFSET + PENGUIN_INITIAL_OFFSET;
        let non_coord_bit_mask = !(PENGUIN_COORDS_BIT_MASK << position_offset);
        self.value = (self.value & non_coord_bit_mask) | (combined_coords << position_offset);
    }

    fn add_penguin_at_coordinate(&mut self, coordinate: Coordinate) {
        let team_penguin_count = self.value & PENGUIN_COUNT_BIT_MASK;
        self.value = (self.value & !PENGUIN_COUNT_BIT_MASK) | ((team_penguin_count + 1) & PENGUIN_COUNT_BIT_MASK);
        self.add_penguin_at_bit_position(coordinate, team_penguin_count);
    }

    pub fn add_penguin(&mut self, penguin: Penguin) {
        self.add_penguin_at_coordinate(penguin.coordinate);
    }

    fn get_coords_at_bitset_index(&self, index: u64) -> Coordinate {
        let start_pos = PENGUIN_INITIAL_OFFSET + index * PENGUIN_COORDS_OFFSET;
        let y_start_pos = start_pos + PENGUIN_COORD_OFFSET;
        let x = (self.value & (PENGUIN_COORD_BIT_MASK << start_pos)) >> start_pos;
        let y = (self.value & (PENGUIN_COORD_BIT_MASK << y_start_pos)) >> y_start_pos;
        Coordinate::new(x * 2 + y % 2, y)
    }

    pub fn move_penguin(&mut self, penguin: Penguin, to: Coordinate) -> anyhow::Result<()> {
        let penguin_count = self.get_penguin_count();
        for index in 0..=penguin_count {
            if self.get_coords_at_bitset_index(index) == penguin.coordinate {
                self.add_penguin_at_bit_position(to, index);
                return Ok(());
            }
        }
        anyhow::bail!("Penguin {:?} doesn't exist", penguin)
    }

    pub fn get_penguin(&self, coordinates: Coordinate, team: Team) -> anyhow::Result<Penguin> {
        if !self.has_penguin_at(coordinates.clone()) {
            anyhow::bail!("No penguin at {:?}", coordinates);
        }

        Ok(Penguin {
            coordinate: coordinates,
            team
        })
    }

    pub fn has_penguin_at(&self, coordinates: Coordinate) -> bool {
        let penguin_count = self.get_penguin_count();
        for index in 0..=penguin_count {
            if self.get_coords_at_bitset_index(index) == coordinates {
                return true;
            }
        }
        
        return false;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_penguin_at_position_0_0_and_try_get_it_back() {
        let mut penguin_bitset = PenguinBitset::empty();
        penguin_bitset.add_penguin(Penguin { coordinate: Coordinate::new(0, 0), team: Team::One });
        assert_eq!(1, penguin_bitset.get_penguin_count());
        let penguin = penguin_bitset.get_penguin(Coordinate::new(0, 0), Team::One);
        assert!(penguin.is_ok());
        let penguin_at_wrong_coordinate = penguin_bitset.get_penguin(Coordinate::new(1, 1), Team::One);
        assert!(penguin_at_wrong_coordinate.is_err());
    }

    #[test]
    fn add_penguin_at_position_0_0_then_bitset_has_penguin() {
        let mut penguin_bitset = PenguinBitset::empty();
        penguin_bitset.add_penguin(Penguin { coordinate: Coordinate::new(0, 0), team: Team::One });
        assert_eq!(1, penguin_bitset.get_penguin_count());
        assert!(penguin_bitset.has_penguin_at(Coordinate::new(0, 0)));
    }
    
    #[test]
    fn add_penguin_at_position_then_bitset_doesnt_have_penguin_at_other_position() {
        let mut penguin_bitset = PenguinBitset::empty();
        penguin_bitset.add_penguin(Penguin { coordinate: Coordinate::new(3, 7), team: Team::One });
        assert_eq!(1, penguin_bitset.get_penguin_count());
        assert!(!penguin_bitset.has_penguin_at(Coordinate::new(4, 2)));
    }

    #[test]
    fn adding_penguins_increases_penguin_count() {
        let mut penguin_bitset = PenguinBitset::empty();
        penguin_bitset.add_penguin(Penguin { coordinate: Coordinate::new(2, 4), team: Team::One });
        penguin_bitset.add_penguin(Penguin { coordinate: Coordinate::new(3, 1), team: Team::One });
        penguin_bitset.add_penguin(Penguin { coordinate: Coordinate::new(0, 6), team: Team::One });
        assert_eq!(3, penguin_bitset.get_penguin_count());
    }
    
    #[test]
    fn moving_penguins_doesnt_increase_penguin_count() {
        let mut penguin_bitset = PenguinBitset::empty();
        let penguin = Penguin { coordinate: Coordinate::new(3, 1), team: Team::Two };
        penguin_bitset.add_penguin(penguin.clone());
        penguin_bitset.move_penguin(penguin, Coordinate::new(6, 4)).unwrap();
        assert_eq!(1, penguin_bitset.get_penguin_count());
    }

    #[test]
    fn moved_penguin_doesnt_exist_at_previous_coordinate() {
        let mut penguin_bitset = PenguinBitset::empty();
        let coordinate = Coordinate::new(0, 6);
        let penguin = Penguin { coordinate: coordinate.clone(), team: Team::One };
        penguin_bitset.add_penguin(penguin.clone());
        penguin_bitset.move_penguin(penguin, Coordinate::new(5, 3)).unwrap();
        assert!(penguin_bitset.get_penguin(coordinate, Team::One).is_err());
    }
}
