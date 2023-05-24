use super::common::*;
use crate::xml;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Move {
    Place(Coordinate),
    Normal { from: Coordinate, to: Coordinate },
}

impl Move {
    pub fn get_to(&self) -> Coordinate {
        match self {
            Move::Place(to) => to.clone(),
            Move::Normal { from: _, to } => to.clone(),
        }
    }
}

impl From<xml::moves::Move> for Move {
    fn from(xml_move: xml::moves::Move) -> Self {
        if let Some(from) = xml_move.from {
            Self::Normal {
                from: Coordinate::new(from.x, from.y),
                to: Coordinate::new(xml_move.to.x, xml_move.to.y),
            }
        } else {
            Self::Place(Coordinate::new(xml_move.to.x, xml_move.to.y))
        }
    }
}

impl Into<xml::moves::Move> for Move {
    fn into(self) -> xml::moves::Move {
        match self {
            Move::Place(to) => xml::moves::Move {
                from: None,
                to: xml::moves::To {
                    x: to.x(),
                    y: to.y(),
                },
            },
            Move::Normal { from, to } => xml::moves::Move {
                from: Some(xml::moves::From {
                    x: from.x(),
                    y: from.y(),
                }),
                to: xml::moves::To {
                    x: to.x(),
                    y: to.y(),
                },
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::xml::moves::Move as XmlMove;
    use crate::xml::moves::{From, To};

    #[test]
    fn normal_move_from_xml_deserialized() {
        let normal_move = XmlMove {
            from: Some(From { x: 0, y: 7 }),
            to: To { x: 4, y: 5 },
        };
        let expected = Move::Normal {
            from: Coordinate::new(0, 7),
            to: Coordinate::new(4, 5),
        };
        let actual = Move::from(normal_move);
        assert_eq!(expected, actual);
    }

    #[test]
    fn place_move_from_xml_deserialized() {
        let place_move = XmlMove {
            from: None,
            to: To { x: 4, y: 3 },
        };
        let expected = Move::Place(Coordinate::new(4, 3));
        let actual = Move::from(place_move);
        assert_eq!(expected, actual);
    }
}
