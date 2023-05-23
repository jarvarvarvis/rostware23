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
    #[inline] pub fn empty() -> Self {
        Self {
            value: 0
        }
    }

    #[inline] pub fn get_penguin_count(&self) -> u64 {
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

    #[inline] pub fn add_penguin(&mut self, penguin: Penguin) {
        self.add_penguin_at_coordinate(penguin.coordinate);
    }

    fn get_coords_at_bitset_index(&self, index: u64) -> Coordinate {
        let start_pos = PENGUIN_INITIAL_OFFSET + index * PENGUIN_COORDS_OFFSET;
        let y_start_pos = start_pos + PENGUIN_COORD_OFFSET;
        let x = (self.value & (PENGUIN_COORD_BIT_MASK << start_pos)) >> start_pos;
        let y = (self.value & (PENGUIN_COORD_BIT_MASK << y_start_pos)) >> y_start_pos;
        Coordinate::new(x * 2 + y % 2, y)
    }

    #[inline] pub fn move_penguin(&mut self, penguin: Penguin, to: Coordinate) -> anyhow::Result<()> {
        let penguin_count = self.get_penguin_count();
        for index in 0..=penguin_count {
            if self.get_coords_at_bitset_index(index) == penguin.coordinate {
                self.add_penguin_at_bit_position(to, index);
                return Ok(());
            }
        }
        anyhow::bail!("Penguin {:?} doesn't exist", penguin)
    }

    #[inline] pub fn get_penguin(&self, coordinates: Coordinate, team: Team) -> anyhow::Result<Penguin> {
        if !self.has_penguin_at(coordinates.clone()) {
            anyhow::bail!("No penguin at {:?}", coordinates);
        }

        Ok(Penguin {
            coordinate: coordinates,
            team
        })
    }

    #[inline] pub fn has_penguin_at(&self, coordinates: Coordinate) -> bool {
        let penguin_count = self.get_penguin_count();
        for index in 0..penguin_count {
            if self.get_coords_at_bitset_index(index) == coordinates {
                return true;
            }
        }
        
        return false;
    }
}

pub struct PenguinBitsetIterator {
    bitset: PenguinBitset,
    penguin_index: u64,
    team: Team
}

impl PenguinBitsetIterator {
    #[inline] pub fn from(bitset: PenguinBitset, team: Team) -> Self {
        Self { bitset, penguin_index: 0, team }
    }
}

impl Iterator for PenguinBitsetIterator {
    type Item = Penguin;

    fn next(&mut self) -> Option<Self::Item> {
        if self.penguin_index < self.bitset.get_penguin_count() {
            let coordinates = self.bitset.get_coords_at_bitset_index(self.penguin_index);
            self.penguin_index += 1;
            self.bitset.get_penguin(coordinates, self.team.clone()).ok()
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn penguin_bitset_doesnt_have_penguin_at_0_0_by_default() {
        let penguin_bitset = PenguinBitset::empty();
        assert!(!penguin_bitset.has_penguin_at(Coordinate::new(0, 0)));
    }

    #[test]
    fn get_coords_at_bitset_index() {
        let test_bitset = PenguinBitset {
            value: 0b001001001
        };
        let expected = Coordinate::new(3, 1);
        let actual = test_bitset.get_coords_at_bitset_index(0);
        assert_eq!(expected, actual);
    }

    #[test]
    fn add_penguin_at_pos_of_bitset() {
        let mut test_bitset = PenguinBitset {
            value: 0b010010010
        };
        let expected = PenguinBitset {
            value: 0b001001010010010
        };
        test_bitset.add_penguin_at_bit_position(Coordinate::new(3, 1), 1);
        assert_eq!(expected, test_bitset);
    }

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
    fn get_penguin_count_of_bitset() {
        let test_bitset = PenguinBitset {
            value: 0b001001001
        };
        assert_eq!(1, test_bitset.get_penguin_count());
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

    #[test]
    fn iterate_empty_penguin_bitset() {
        let penguin_bitset = PenguinBitset::empty();
        let mut iterator = PenguinBitsetIterator::from(penguin_bitset, Team::One);
        assert_eq!(None, iterator.next());
    }

    #[test]
    fn iterate_penguin_bitset() {
        let mut penguin_bitset = PenguinBitset::empty();
        penguin_bitset.add_penguin(Penguin { coordinate: Coordinate::new(2, 4), team: Team::One });
        penguin_bitset.add_penguin(Penguin { coordinate: Coordinate::new(3, 1), team: Team::One });
        penguin_bitset.add_penguin(Penguin { coordinate: Coordinate::new(0, 6), team: Team::One });
        let mut iterator = PenguinBitsetIterator::from(penguin_bitset, Team::One);
        assert_eq!(Some(Penguin { coordinate: Coordinate::new(2, 4), team: Team::One }), iterator.next());
        assert_eq!(Some(Penguin { coordinate: Coordinate::new(3, 1), team: Team::One }), iterator.next());
        assert_eq!(Some(Penguin { coordinate: Coordinate::new(0, 6), team: Team::One }), iterator.next());
        assert_eq!(None, iterator.next());
    }
}
