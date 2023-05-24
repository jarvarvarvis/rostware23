use super::common::*;
use super::penguin::*;
use super::penguin_bitset::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PenguinCollection {
    team_one_penguins: PenguinBitset,
    team_two_penguins: PenguinBitset,
}

impl PenguinCollection {
    pub fn empty() -> Self {
        Self {
            team_one_penguins: PenguinBitset::empty(),
            team_two_penguins: PenguinBitset::empty(),
        }
    }

    pub fn add_penguin(&mut self, penguin: Penguin) {
        match penguin.team {
            Team::One => self.team_one_penguins.add_penguin(penguin),
            Team::Two => self.team_two_penguins.add_penguin(penguin),
        }
    }

    pub fn move_penguin(&mut self, penguin: Penguin, to: Coordinate) -> anyhow::Result<()> {
        match penguin.team {
            Team::One => self.team_one_penguins.move_penguin(penguin, to),
            Team::Two => self.team_two_penguins.move_penguin(penguin, to),
        }
    }

    pub fn get_penguin(&self, coordinate: Coordinate) -> anyhow::Result<Penguin> {
        let team_one = self.team_one_penguins.has_penguin_at(coordinate.clone());
        let team_two = self.team_two_penguins.has_penguin_at(coordinate.clone());

        match (team_one, team_two) {
            (true, false) => self.team_one_penguins.get_penguin(coordinate, Team::One),
            (false, true) => self.team_two_penguins.get_penguin(coordinate, Team::Two),
            (false, false) => anyhow::bail!("No penguin exists at {:?}", coordinate),
            (true, true) => anyhow::bail!("Penguin of both teams is present at {:?}", coordinate),
        }
    }

    pub fn get_iter_for_team(&self, team: Team) -> impl Iterator<Item = Penguin> {
        PenguinBitsetIterator::from(
            match team {
                Team::One => self.team_one_penguins.clone(),
                Team::Two => self.team_two_penguins.clone(),
            },
            team,
        )
    }
}

impl std::fmt::Display for PenguinCollection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..BOARD_HEIGHT {
            if y % 2 == 1 {
                write!(f, " ")?;
            }

            for x in 0..BOARD_WIDTH {
                let penguin = self.get_penguin(Coordinate::new(x, y).odd_r_to_doubled());
                match penguin {
                    Ok(penguin) => match penguin.team {
                        Team::One => write!(f, "1 ")?,
                        Team::Two => write!(f, "2 ")?,
                    },
                    Err(_) => write!(f, "_ ")?,
                }
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
    fn adding_penguin_at_position_0_0_doesnt_influence_other_bitset() {
        let mut penguin_collection = PenguinCollection::empty();
        penguin_collection.team_one_penguins.add_penguin(Penguin {
            coordinate: Coordinate::new(0, 0),
            team: Team::One,
        });
        assert!(!penguin_collection
            .team_two_penguins
            .has_penguin_at(Coordinate::new(0, 0)));
    }

    #[test]
    fn adding_penguin_at_other_position_doesnt_influence_other_bitset() {
        let mut penguin_collection = PenguinCollection::empty();
        penguin_collection.team_one_penguins.add_penguin(Penguin {
            coordinate: Coordinate::new(5, 1),
            team: Team::One,
        });
        assert!(!penguin_collection
            .team_two_penguins
            .has_penguin_at(Coordinate::new(5, 1)));
    }

    #[test]
    fn add_penguin_to_collection_at_position_0_0() {
        let mut penguin_collection = PenguinCollection::empty();
        penguin_collection.add_penguin(Penguin {
            coordinate: Coordinate::new(0, 0),
            team: Team::One,
        });
        let penguin = penguin_collection.get_penguin(Coordinate::new(0, 0));
        assert_eq!(
            penguin.unwrap(),
            Penguin {
                coordinate: Coordinate::new(0, 0),
                team: Team::One
            }
        );
    }

    #[test]
    fn add_penguin_to_collection_at_correct_position() {
        let mut penguin_collection = PenguinCollection::empty();
        penguin_collection.add_penguin(Penguin {
            coordinate: Coordinate::new(2, 6),
            team: Team::One,
        });
        assert_eq!(
            penguin_collection
                .get_penguin(Coordinate::new(2, 6))
                .unwrap()
                .team,
            Team::One
        );
    }

    #[test]
    fn add_penguin_to_collection_and_get_at_incorrect_position() {
        let mut penguin_collection = PenguinCollection::empty();
        penguin_collection.add_penguin(Penguin {
            coordinate: Coordinate::new(2, 6),
            team: Team::One,
        });
        assert!(penguin_collection
            .get_penguin(Coordinate::new(3, 3))
            .is_err());
    }

    #[test]
    fn move_penguin_to_correct_position() {
        let mut penguin_collection = PenguinCollection::empty();
        let penguin = Penguin {
            coordinate: Coordinate::new(1, 7),
            team: Team::One,
        };
        penguin_collection.add_penguin(penguin.clone());
        penguin_collection
            .move_penguin(penguin, Coordinate::new(2, 2))
            .unwrap();
        assert_eq!(
            penguin_collection
                .get_penguin(Coordinate::new(2, 2))
                .unwrap()
                .team,
            Team::One
        );
    }

    #[test]
    fn try_move_penguin_with_invalid_team() {
        let mut penguin_collection = PenguinCollection::empty();
        let penguin = Penguin {
            coordinate: Coordinate::new(4, 2),
            team: Team::One,
        };
        penguin_collection.add_penguin(penguin);
        let other_team_penguin = Penguin {
            coordinate: Coordinate::new(4, 2),
            team: Team::Two,
        };
        assert!(penguin_collection
            .move_penguin(other_team_penguin, Coordinate::new(0, 0))
            .is_err());
    }

    #[test]
    fn iterate_penguin_collection_for_team_one_penguins() {
        let mut penguin_collection = PenguinCollection::empty();
        penguin_collection.add_penguin(Penguin {
            coordinate: Coordinate::new(2, 4),
            team: Team::One,
        });
        penguin_collection.add_penguin(Penguin {
            coordinate: Coordinate::new(0, 6),
            team: Team::One,
        });
        penguin_collection.add_penguin(Penguin {
            coordinate: Coordinate::new(2, 0),
            team: Team::Two,
        });
        penguin_collection.add_penguin(Penguin {
            coordinate: Coordinate::new(3, 1),
            team: Team::Two,
        });
        penguin_collection.add_penguin(Penguin {
            coordinate: Coordinate::new(5, 3),
            team: Team::One,
        });
        let mut iterator = penguin_collection.get_iter_for_team(Team::One);

        assert_eq!(
            Some(Penguin {
                coordinate: Coordinate::new(2, 4),
                team: Team::One
            }),
            iterator.next()
        );
        assert_eq!(
            Some(Penguin {
                coordinate: Coordinate::new(0, 6),
                team: Team::One
            }),
            iterator.next()
        );
        assert_eq!(
            Some(Penguin {
                coordinate: Coordinate::new(5, 3),
                team: Team::One
            }),
            iterator.next()
        );
        assert_eq!(None, iterator.next());
    }

    #[test]
    fn iterate_penguin_collection_for_team_two_penguins() {
        let mut penguin_collection = PenguinCollection::empty();
        penguin_collection.add_penguin(Penguin {
            coordinate: Coordinate::new(2, 2),
            team: Team::One,
        });
        penguin_collection.add_penguin(Penguin {
            coordinate: Coordinate::new(2, 6),
            team: Team::One,
        });
        penguin_collection.add_penguin(Penguin {
            coordinate: Coordinate::new(2, 6),
            team: Team::Two,
        });
        penguin_collection.add_penguin(Penguin {
            coordinate: Coordinate::new(3, 5),
            team: Team::Two,
        });
        penguin_collection.add_penguin(Penguin {
            coordinate: Coordinate::new(1, 3),
            team: Team::One,
        });
        let mut iterator = penguin_collection.get_iter_for_team(Team::Two);

        assert_eq!(
            Some(Penguin {
                coordinate: Coordinate::new(2, 6),
                team: Team::Two
            }),
            iterator.next()
        );
        assert_eq!(
            Some(Penguin {
                coordinate: Coordinate::new(3, 5),
                team: Team::Two
            }),
            iterator.next()
        );
        assert_eq!(None, iterator.next());
    }
}
