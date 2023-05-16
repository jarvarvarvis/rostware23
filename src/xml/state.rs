use instant_xml::FromXml;

use super::common;

#[derive(FromXml, Debug, Eq, PartialEq)]
#[xml(rename = "startTeam")]
pub struct StartTeam {
    #[xml(direct)]
    pub team: common::Team
}

#[derive(Debug, Eq, PartialEq)]
pub enum FieldState {
    Empty,
    Fish(u32),
    Team(common::Team)
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

#[derive(FromXml, Debug, Eq, PartialEq)]
#[xml(rename = "field")]
pub struct Field {
    #[xml(direct)]
    field_state: FieldState
}

#[cfg(test)]
mod tests {
    use crate::xml::*;
    use super::*;

    #[test]
    fn test_deserialize_start_team() {
        let start_team = r#"<startTeam>ONE</startTeam>"#;
        let expected = StartTeam { team: common::Team::One };
        let actual = deserialize(start_team).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_deserialize_field_row_entry_with_0_value() {
        let field_state = "<field>0</field>";
        let expected = Field {
            field_state: FieldState::Empty
        };
        let actual = deserialize(field_state).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_deserialize_field_row_entry_with_non_0_value() {
        let field_state = "<field>3</field>";
        let expected = Field {
            field_state: FieldState::Fish(3)
        };
        let actual = deserialize(field_state).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_deserialize_field_row_entry_with_team_one() {
        let field_state = "<field>ONE</field>";
        let expected = Field {
            field_state: FieldState::Team(common::Team::One)
        };
        let actual = deserialize(field_state).unwrap();
        assert_eq!(expected, actual);
    }
}
