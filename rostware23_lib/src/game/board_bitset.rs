use std::fmt::Display;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Bitset8x8 {
    pub value: u64,
}

fn get_8x8_coordinate_mask(x: u64, y: u64) -> u64 {
    1 << (y * 8 + x)
}

impl Bitset8x8 {
    pub fn empty() -> Self {
        Self { value: 0 }
    }

    pub fn get(&self, x: u64, y: u64) -> anyhow::Result<bool> {
        if x >= 8 || y >= 8 {
            anyhow::bail!("Coordinates ({x},{y}) are out of bounds");
        }

        Ok(self.value & get_8x8_coordinate_mask(x, y) != 0)
    }

    pub fn set(&mut self, x: u64, y: u64, new_value: bool) -> anyhow::Result<bool> {
        let old_state = self.get(x, y)?;
        if new_value {
            self.value |= get_8x8_coordinate_mask(x, y);
        } else {
            self.value &= !get_8x8_coordinate_mask(x, y);
        }
        Ok(old_state)
    }

    pub fn with_set(&self, x: u64, y: u64, new_value: bool) -> anyhow::Result<Self> {
        let mut new_state = self.clone();
        let _ = new_state.set(x, y, new_value)?;
        Ok(new_state)
    }
}

impl Display for Bitset8x8 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..super::common::BOARD_HEIGHT {
            if y % 2 == 1 {
                write!(f, " ")?;
            }
            
            for x in 0..super::common::BOARD_WIDTH {
                let state = self.get(x, y).unwrap_or(false);
                write!(f, "{} ", state as u8)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_bitset() {
        let bitset = Bitset8x8::empty();
        for y in 0..8 {
            for x in 0..8 {
                assert!(!bitset.get(x, y).unwrap());
            }
        }
    }

    #[test]
    fn set_bitset_diagonal() {
        let mut bitset = Bitset8x8::empty();
        for y in 0..8 {
            for x in 0..8 {
                if x == y {
                    assert!(!bitset.set(x, y, true).unwrap());
                }
            }
        }

        for y in 0..8 {
            for x in 0..8 {
                if x == y {
                    assert!(bitset.get(x, y).unwrap());
                } else {
                    assert!(!bitset.get(x, y).unwrap());
                }
            }
        }
    }

    #[test]
    fn display_simple_bitset() {
        let mut bitset = Bitset8x8::empty();
        bitset.set(2, 1, true).unwrap();
        bitset.set(0, 2, true).unwrap();
        bitset.set(2, 2, true).unwrap();
        bitset.set(6, 2, true).unwrap();
        bitset.set(6, 3, true).unwrap();
        bitset.set(7, 3, true).unwrap();
        bitset.set(1, 4, true).unwrap();
        bitset.set(0, 7, true).unwrap();
        let expected = "0 0 0 0 0 0 0 0 \n 0 0 1 0 0 0 0 0 \n1 0 1 0 0 0 1 0 \n 0 0 0 0 0 0 1 1 \n0 1 0 0 0 0 0 0 \n 0 0 0 0 0 0 0 0 \n0 0 0 0 0 0 0 0 \n 1 0 0 0 0 0 0 0 \n";
        assert_eq!(expected, format!("{}", bitset));
    }
}
