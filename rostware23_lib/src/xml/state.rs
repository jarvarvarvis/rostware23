use instant_xml::{FromXml, ToXml};

use super::common;

#[derive(FromXml, ToXml, Debug, Eq, PartialEq)]
#[xml(rename = "startTeam")]
pub struct StartTeam {
    #[xml(direct)]
    pub team: common::Team
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum FieldState {
    Empty,
    Fish(u32),
    Team(common::Team)
}

impl FieldState {
    #[inline] pub fn get_fish_count(&self) -> anyhow::Result<u32> {
        match self {
            FieldState::Empty => Ok(0),
            FieldState::Fish(fish_count) => Ok(*fish_count),
            FieldState::Team(_) => anyhow::bail!("Can't get fish count of player field"),
        }
    }
}

impl<'xml> FromXml<'xml> for FieldState {
    fn matches(id: instant_xml::Id<'_>, field: Option<instant_xml::Id<'_>>) -> bool {
        match field {
            Some(field) => id == field,
            None => false
        }
    }

    fn deserialize<'cx>(
        into: &mut Self::Accumulator,
        field: &'static str,
        deserializer: &mut instant_xml::Deserializer<'cx, 'xml>,
    ) -> Result<(), instant_xml::Error> {
        if into.is_some() {
            return Err(instant_xml::Error::DuplicateValue);
        }

        let value = match deserializer.take_str()? {
            Some(value) => value,
            None => return Ok(()),
        };

        let value = match value {
            "0" => FieldState::Empty,
            "1" => FieldState::Fish(1),
            "2" => FieldState::Fish(2),
            "3" => FieldState::Fish(3),
            "4" => FieldState::Fish(4),
            "ONE" => FieldState::Team(common::Team::One),
            "TWO" => FieldState::Team(common::Team::Two),
            other =>
                return Err(instant_xml::Error::UnexpectedValue(
                        format!("Unable to parse field state from '{other}' for {field}")
                ))
        };
        *into = Some(value);
        Ok(())
    }

    type Accumulator = Option<FieldState>;
    const KIND: instant_xml::Kind = instant_xml::Kind::Scalar;
}

impl ToXml for FieldState {
    fn serialize<W: std::fmt::Write + ?Sized>(
        &self,
        _: Option<instant_xml::Id<'_>>,
        _: &mut instant_xml::Serializer<W>,
    ) -> Result<(), instant_xml::Error> {
        Err(instant_xml::Error::Other(
                format!("FieldState is not supposed to be serialized")
        ))
    }
}

#[derive(FromXml, ToXml, Clone, Debug, Eq, PartialEq)]
#[xml(rename = "field")]
pub struct Field(pub FieldState);

#[derive(FromXml, ToXml, Debug, Eq, PartialEq)]
#[xml(rename = "list")]
pub struct FieldRow {
    pub fields: Vec<Field>
}

#[derive(FromXml, ToXml, Debug, Eq, PartialEq)]
#[xml(rename = "board")]
pub struct Board {
    pub rows: Vec<FieldRow>
}

#[derive(FromXml, ToXml, Debug, Eq, PartialEq)]
#[xml(rename = "int")]
pub struct FishEntry(pub u32);

#[derive(FromXml, ToXml, Debug, Eq, PartialEq)]
#[xml(rename = "fishes")]
pub struct Fishes {
    pub entries: Vec<FishEntry>
}

#[derive(FromXml, ToXml, Debug, Eq, PartialEq)]
#[xml(rename = "state")]
pub struct State {
    #[xml(attribute)]
    pub turn: u32,

    #[xml(rename = "startTeam")]
    pub start_team: common::Team,
    pub board: Board,
    pub fishes: Fishes,
}

#[cfg(test)]
mod tests {
    use crate::xml::*;
    use super::*;

    #[test]
    fn deserialize_start_team() {
        let start_team = r#"<startTeam>ONE</startTeam>"#;
        let expected = StartTeam { team: common::Team::One };
        let actual = deserialize(start_team).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn deserialize_field_row_entry_with_0_value() {
        let field_state = "<field>0</field>";
        let expected = Field(FieldState::Empty);
        let actual = deserialize(field_state).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn deserialize_field_row_entry_with_non_0_value() {
        let field_state = "<field>3</field>";
        let expected = Field(FieldState::Fish(3));
        let actual = deserialize(field_state).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn deserialize_field_row_entry_with_team_one() {
        let field_state = "<field>ONE</field>";
        let expected = Field(FieldState::Team(common::Team::One));
        let actual = deserialize(field_state).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn deserialize_field_row() {
        let field_row = r#"<list>
            <field>1</field>
            <field>0</field>
            <field>2</field>
            <field>TWO</field>
            <field>2</field>
            <field>3</field>
            <field>TWO</field>
            <field>ONE</field>
    	</list>"#;
        let expected = FieldRow {
            fields: vec![
                Field(FieldState::Fish(1)),
                Field(FieldState::Empty),
                Field(FieldState::Fish(2)),
                Field(FieldState::Team(common::Team::Two)),
                Field(FieldState::Fish(2)),
                Field(FieldState::Fish(3)),
                Field(FieldState::Team(common::Team::Two)),
                Field(FieldState::Team(common::Team::One)),
            ]
        };
        let actual = deserialize(field_row).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn deserialize_board() {
        let board = r#"<board>
            <list>
                <field>1</field>
                <field>2</field>
                <field>2</field>
            </list>
            <list>
                <field>2</field>
                <field>ONE</field>
                <field>2</field>
            </list>
            <list>
                <field>TWO</field>
                <field>4</field>
                <field>1</field>
            </list>
        </board>"#;
        let expected = Board {
            rows: vec![
                FieldRow {
                    fields: vec![
                        Field(FieldState::Fish(1)),
                        Field(FieldState::Fish(2)),
                        Field(FieldState::Fish(2)),
                    ]
                },
                FieldRow {
                    fields: vec![
                        Field(FieldState::Fish(2)),
                        Field(FieldState::Team(common::Team::One)),
                        Field(FieldState::Fish(2)),
                    ]
                },
                FieldRow {
                    fields: vec![
                        Field(FieldState::Team(common::Team::Two)),
                        Field(FieldState::Fish(4)),
                        Field(FieldState::Fish(1)),
                    ]
                }
            ]
        };
        let actual = deserialize(board).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn deserialize_state() {
        let board = r#"<state turn="6">
            <startTeam>ONE</startTeam>
            <board>
                <list>
                    <field>1</field>
                    <field>2</field>
                    <field>2</field>
                </list>
                <list>
                    <field>2</field>
                    <field>ONE</field>
                    <field>2</field>
                </list>
                <list>
                    <field>TWO</field>
                    <field>4</field>
                    <field>1</field>
                </list>
            </board>
            <fishes>
                <int>4</int>
                <int>31</int>
            </fishes>
        </state>"#;
        let expected = State {
            turn: 6,
            start_team: common::Team::One,
            board: Board {
                rows: vec![
                    FieldRow {
                        fields: vec![
                            Field(FieldState::Fish(1)),
                            Field(FieldState::Fish(2)),
                            Field(FieldState::Fish(2)),
                        ]
                    },
                    FieldRow {
                        fields: vec![
                            Field(FieldState::Fish(2)),
                            Field(FieldState::Team(common::Team::One)),
                            Field(FieldState::Fish(2)),
                        ]
                    },
                    FieldRow {
                        fields: vec![
                            Field(FieldState::Team(common::Team::Two)),
                            Field(FieldState::Fish(4)),
                            Field(FieldState::Fish(1)),
                        ]
                    }
                ]
            },
            fishes: Fishes {
                entries: vec![
                    FishEntry(4), 
                    FishEntry(31)
                ]
            }
        };
        let actual = deserialize(board).unwrap();
        assert_eq!(expected, actual);
    }
}
