use instant_xml::{FromXml, ToXml};

use super::common;

#[derive(FromXml, ToXml, Debug, Eq, PartialEq)]
pub struct GameResult {
    
}

#[cfg(test)]
mod tests {
    use crate::xml::*;
    use super::*;

    #[test]
    fn test_deserialize_result() {
        let result = r#"<data class="result">
            <definition>
                <fragment name="Siegpunkte">
                    <aggregation>SUM</aggregation>
                    <relevantForRanking>true</relevantForRanking>
                 </fragment>
                 <fragment name="∅ Punkte">
                     <aggregation>AVERAGE</aggregation>
                     <relevantForRanking>true</relevantForRanking>
                  </fragment>
            </definition>
            <scores>
                <entry>
                    <player name="rad" team="ONE"/>
                    <score cause="REGULAR" reason="">
                        <part>2</part>
                        <part>27</part>
                    </score>
                </entry>
                <entry>
                    <player name="blues" team="TWO"/>
                    <score cause="LEFT" reason="Player left">
                        <part>0</part>
                        <part>15</part>
                    </score>
                </entry>
            </scores>
            <winner team="ONE"/>
        </data>"#;
        let expected = data::Data {
            class: data::DataClass::Result,
            color: None,
            sent_move: None,
            result: None
        };
        let actual = deserialize(result).unwrap();
        assert_eq!(expected, actual);
    }
}
