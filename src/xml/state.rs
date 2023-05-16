use serde::Deserialize;

use super::common;

#[derive(Debug, Deserialize, Eq, PartialEq)]
#[serde(rename = "startTeam")]
pub struct StartTeam {
    #[serde(rename = "$text")]
    pub team: common::Team
}

#[derive(Debug, Deserialize, Eq, PartialEq)]
pub enum FieldState {
    #[serde(rename = "0")]
    Empty,
}

#[derive(Debug, Deserialize, Eq, PartialEq)]
#[serde(rename = "field")]
pub struct Field {
    #[serde(rename = "$text")]
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
        let actual = deserialize(start_team.to_string()).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_deserialize_field_row_entry_0() {
        let field_state = "<field>0</field>";
        let expected = Field {
            field_state: FieldState::Empty
        };
        let actual = deserialize(field_state.to_string()).unwrap();
        assert_eq!(expected, actual);
    }
}
