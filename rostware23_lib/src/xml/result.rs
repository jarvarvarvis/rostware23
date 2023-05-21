use instant_xml::{FromXml, ToXml};

use super::common;


#[derive(FromXml, ToXml, Debug, Eq, PartialEq)]
#[xml(scalar, rename_all = "UPPERCASE")]
pub enum AggregationKind {
    Sum,
    Average
}

#[derive(FromXml, ToXml, Debug, Eq, PartialEq)]
#[xml(rename = "aggregation")]
pub struct Aggregation(pub AggregationKind);

#[derive(FromXml, ToXml, Debug, Eq, PartialEq)]
#[xml(rename = "relevantForRanking")]
pub struct RelevantForRanking(pub bool);

#[derive(FromXml, ToXml, Debug, Eq, PartialEq)]
#[xml(rename = "fragment")]
pub struct Fragment {
    #[xml(attribute)]
    pub name: String,

    pub aggregation: Aggregation,
    pub relevant_for_ranking: RelevantForRanking
}

#[derive(FromXml, ToXml, Debug, Eq, PartialEq)]
#[xml(rename = "definition")]
pub struct Definition {
    #[xml(rename = "fragment")]
    pub fragments: Vec<Fragment>
}

#[derive(FromXml, ToXml, Debug, Eq, PartialEq)]
#[xml(rename = "player")]
pub struct ScoresEntryPlayer {
    #[xml(attribute)]
    pub name: String,

    #[xml(attribute)]
    pub team: common::Team
}

#[derive(FromXml, ToXml, Debug, Eq, PartialEq)]
#[xml(rename = "part")]
pub struct ScorePart(pub u32);

#[derive(FromXml, ToXml, Debug, Eq, PartialEq)]
#[xml(rename = "score")]
pub struct ScoresEntryScore {
    #[xml(attribute)]
    pub cause: String,

    #[xml(attribute)]
    pub reason: String,

    #[xml(rename = "part")]
    pub parts: Vec<ScorePart>
}

#[derive(FromXml, ToXml, Debug, Eq, PartialEq)]
#[xml(rename = "entry")]
pub struct ScoresEntry {
    pub player: ScoresEntryPlayer,
    pub score: ScoresEntryScore
}

#[derive(FromXml, ToXml, Debug, Eq, PartialEq)]
#[xml(rename = "scores")]
pub struct Scores {
    #[xml(rename = "entry")]
    pub entries: Vec<ScoresEntry>
}

#[derive(FromXml, ToXml, Debug, Eq, PartialEq)]
#[xml(rename = "winner")]
pub struct Winner {
    #[xml(attribute)]
    pub team: common::Team
}

#[derive(FromXml, ToXml, Debug, Eq, PartialEq)]
#[xml(transparent)]
pub struct GameResult {
    pub definition: Definition,
    pub scores: Scores,
    pub winner: Option<Winner>
}

#[cfg(test)]
mod tests {
    use crate::xml::*;
    use super::*;

    #[test]
    fn deserialize_aggregation() {
        let aggregation = r#"<aggregation>SUM</aggregation>"#;
        let expected = Aggregation(AggregationKind::Sum);
        let actual = deserialize(aggregation).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn deserialize_relevant_for_ranking() {
        let relevant_for_ranking = r#"<relevantForRanking>false</relevantForRanking>"#;
        let expected = RelevantForRanking(false);
        let actual = deserialize(relevant_for_ranking).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn deserialize_fragment() {
        let fragment = r#"<fragment name="∅ Punkte">
            <aggregation>AVERAGE</aggregation>
            <relevantForRanking>true</relevantForRanking>
        </fragment>"#;
        let expected = Fragment { 
            name: "∅ Punkte".to_string(), 
            aggregation: Aggregation(AggregationKind::Average), 
            relevant_for_ranking: RelevantForRanking(true)
        };
        let actual = deserialize(fragment).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn deserialize_definition() {
        let definition = r#"<definition>
            <fragment name="Siegpunkte">
                <aggregation>SUM</aggregation>
                <relevantForRanking>true</relevantForRanking>
            </fragment>
            <fragment name="∅ Punkte">
                <aggregation>AVERAGE</aggregation>
                <relevantForRanking>true</relevantForRanking>
            </fragment>
        </definition>"#;
        let expected = Definition {
            fragments: vec![
                Fragment {
                    name: "Siegpunkte".to_string(),
                    aggregation: Aggregation(AggregationKind::Sum),
                    relevant_for_ranking: RelevantForRanking(true)
                },
                Fragment {
                    name: "∅ Punkte".to_string(),
                    aggregation: Aggregation(AggregationKind::Average),
                    relevant_for_ranking: RelevantForRanking(true)
                }
            ]
        };
        let actual = deserialize(definition).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn deserialize_score_entry_player() {
        let player = r#"<player name="amogus" team="TWO"/>"#;
        let expected = ScoresEntryPlayer { 
            name: "amogus".to_string(),
            team: common::Team::Two
        };
        let actual = deserialize(player).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn deserialize_score_part() {
        let score_part = r#"<part>42</part>"#;
        let expected = ScorePart(42);
        let actual = deserialize(score_part).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn deserialize_score() {
        let score = r#"<score cause="LEFT" reason="Player left">
            <part>0</part>
            <part>15</part>
        </score>"#;
        let expected = ScoresEntryScore {
            cause: "LEFT".to_string(),
            reason: "Player left".to_string(),
            parts: vec![ ScorePart(0), ScorePart(15) ]
        };
        let actual = deserialize(score).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn deserialize_scores() {
        let scores = r#"<scores>
            <entry>
                <player name="A Team" team="ONE"/>
                <score cause="REGULAR" reason="">
                    <part>2</part>
                    <part>27</part>
                </score>
            </entry>
            <entry>
                <player name="B Team" team="TWO"/>
                <score cause="LEFT" reason="Player left">
                    <part>0</part>
                    <part>15</part>
                </score>
            </entry>
        </scores>"#;
        let expected = Scores {
            entries: vec![
                ScoresEntry {
                    player: ScoresEntryPlayer { name: "A Team".to_string(), team: common::Team::One },
                    score: ScoresEntryScore { 
                        cause: "REGULAR".to_string(), 
                        reason: "".to_string(), 
                        parts: vec![
                            ScorePart(2),
                            ScorePart(27)
                        ]
                    }
                },
                ScoresEntry {
                    player: ScoresEntryPlayer { name: "B Team".to_string(), team: common::Team::Two },
                    score: ScoresEntryScore { 
                        cause: "LEFT".to_string(), 
                        reason: "Player left".to_string(), 
                        parts: vec![
                            ScorePart(0),
                            ScorePart(15)
                        ]
                    }
                },
            ]
        };
        let actual = deserialize(scores).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn deserialize_winner() {
        let winner = r#"<winner team="ONE"/>"#;
        let expected = Winner { team: common::Team::One };
        let actual = deserialize(winner).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn deserialize_result() {
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
                    <player name="A Team" team="ONE"/>
                    <score cause="REGULAR" reason="">
                        <part>2</part>
                        <part>27</part>
                    </score>
                </entry>
                <entry>
                    <player name="B Team" team="TWO"/>
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
            state: None,
            sent_move: None,
            result: Some(GameResult {
                definition: Definition {
                    fragments: vec![
                        Fragment {
                            name: "Siegpunkte".to_string(),
                            aggregation: Aggregation(AggregationKind::Sum),
                            relevant_for_ranking: RelevantForRanking(true)
                        },
                        Fragment {
                            name: "∅ Punkte".to_string(),
                            aggregation: Aggregation(AggregationKind::Average),
                            relevant_for_ranking: RelevantForRanking(true)
                        }
                    ]
                },
                scores: Scores {
                    entries: vec![
                        ScoresEntry {
                            player: ScoresEntryPlayer { name: "A Team".to_string(), team: common::Team::One },
                            score: ScoresEntryScore { 
                                cause: "REGULAR".to_string(), 
                                reason: "".to_string(), 
                                parts: vec![
                                    ScorePart(2),
                                    ScorePart(27)
                                ]
                            }
                        },
                        ScoresEntry {
                            player: ScoresEntryPlayer { name: "B Team".to_string(), team: common::Team::Two },
                            score: ScoresEntryScore { 
                                cause: "LEFT".to_string(), 
                                reason: "Player left".to_string(), 
                                parts: vec![
                                    ScorePart(0),
                                    ScorePart(15)
                                ]
                            }
                        },
                    ]
                },
                winner: Some(Winner { team: common::Team::One })
            }),
        };
        let actual = deserialize(result).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn deserialize_result_wrapped_in_room() {
        let result = r#"<room roomId="aecb7719-c6d5-468c-87b0-506c04a794e0">
        <data class="result">
          <definition>
            <fragment name="Siegpunkte">
              <aggregation>SUM</aggregation>
              <relevantForRanking>true</relevantForRanking>
            </fragment>
            <fragment name="Fische">
              <aggregation>AVERAGE</aggregation>
              <relevantForRanking>true</relevantForRanking>
            </fragment>
          </definition>
          <scores>
            <entry>
              <player name="Spieler 1" team="ONE"/>
              <score cause="REGULAR" reason="">
                <part>2</part>
                <part>21</part>
              </score>
            </entry>
            <entry>
              <player name="Spieler 2" team="TWO"/>
              <score cause="SOFT_TIMEOUT" reason="Der Spieler hat innerhalb von 2 Sekunden nach Aufforderung keinen Zug gesendet">
                <part>0</part>
                <part>19</part>
              </score>
            </entry>
          </scores>
        </data>
      </room>"#;
        let expected = room::Room { 
            room_id: "aecb7719-c6d5-468c-87b0-506c04a794e0".to_string(),
            data: data::Data {
                class: data::DataClass::Result,
                color: None,
                state: None,
                sent_move: None,
                result: Some(GameResult {
                    definition: Definition {
                        fragments: vec![
                            Fragment {
                                name: "Siegpunkte".to_string(),
                                aggregation: Aggregation(AggregationKind::Sum),
                                relevant_for_ranking: RelevantForRanking(true)
                            },
                            Fragment {
                                name: "Fische".to_string(),
                                aggregation: Aggregation(AggregationKind::Average),
                                relevant_for_ranking: RelevantForRanking(true)
                            }
                        ]
                    },
                    scores: Scores {
                        entries: vec![
                            ScoresEntry {
                                player: ScoresEntryPlayer { name: "Spieler 1".to_string(), team: common::Team::One },
                                score: ScoresEntryScore { 
                                    cause: "REGULAR".to_string(), 
                                    reason: "".to_string(), 
                                    parts: vec![
                                        ScorePart(2),
                                        ScorePart(21)
                                    ]
                                }
                            },
                            ScoresEntry {
                                player: ScoresEntryPlayer { name: "Spieler 2".to_string(), team: common::Team::Two },
                                score: ScoresEntryScore { 
                                    cause: "SOFT_TIMEOUT".to_string(), 
                                    reason: "Der Spieler hat innerhalb von 2 Sekunden nach Aufforderung keinen Zug gesendet".to_string(), 
                                    parts: vec![
                                        ScorePart(0),
                                        ScorePart(19)
                                    ]
                                }
                            },
                        ]
                    },
                    winner: None
                }),
            }
        };
        let actual = deserialize(result).unwrap();
        assert_eq!(expected, actual);
    }
}
