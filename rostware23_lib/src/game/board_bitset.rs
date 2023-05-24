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
}
