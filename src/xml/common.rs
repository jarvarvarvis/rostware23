use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq)]
pub enum Team {
    #[serde(rename = "ONE")]
    One,

    #[serde(rename = "TWO")]
    Two
}

#[cfg(test)]
mod tests {
    use crate::xml::*;
    use super::*;

    #[test]
    fn deserialize_team_inside_other_type() {
        #[derive(Debug, Deserialize, PartialEq)]
        #[serde(rename = "test")]
        struct TestStruct {
            #[serde(rename = "$value")]
            team: Team
        }

        let test_data = "<test>ONE</test>";
        let expected = TestStruct { team: Team::One };
        let actual = deserialize(test_data.to_string()).unwrap();
        assert_eq!(expected, actual);
    }
}
