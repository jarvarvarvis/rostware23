use instant_xml::{FromXml, ToXml};

#[derive(FromXml, ToXml, Clone, Copy, Debug, Eq, PartialEq, Hash)]
#[xml(scalar, rename_all = "UPPERCASE")]
pub enum Team {
    One,
    Two,
}

impl Team {
    #[inline] pub fn opponent(&self) -> Self {
        match self {
            Team::One => Team::Two,
            Team::Two => Team::One,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::xml::*;
    use super::*;

    #[test]
    fn deserialize_team_inside_other_type() {
        #[derive(FromXml, Debug, Eq, PartialEq)]
        #[xml(rename = "test")]
        struct TestStruct {
            #[xml(direct)]
            team: Team
        }

        let test_data = "<test>ONE</test>";
        let expected = TestStruct { team: Team::One };
        let actual = deserialize(test_data).unwrap();
        assert_eq!(expected, actual);
    }
}
