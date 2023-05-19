use super::common::*;

use crate::xml::result::GameResult as XmlGameResult;

#[derive(Debug, Eq, PartialEq)]
pub struct TeamAndPoints(Team, u32);

#[derive(Debug, Eq, PartialEq)]
pub struct GameResult {
    pub winner: Option<Team>,
    pub points: (TeamAndPoints, TeamAndPoints)
}

impl From<XmlGameResult> for GameResult {
    fn from(result: XmlGameResult) -> Self {
        let entries = result.scores.entries;
        let first_entry = &entries[0];
        let second_entry = &entries[1];
        Self {
            winner: result.winner.team,
            points: (
                TeamAndPoints(first_entry.player.team, first_entry.score.parts[1].0),
                TeamAndPoints(second_entry.player.team, second_entry.score.parts[1].0)
            )
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::xml::result::{
        Aggregation, AggregationKind, 
        Definition, Fragment, 
        Scores, ScoresEntry, ScoresEntryPlayer, ScoresEntryScore, ScorePart,
        RelevantForRanking,
        Winner
    };

    #[test]
    fn game_result_from_xml_game_result() {
        let game_result = XmlGameResult {
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
                        player: ScoresEntryPlayer { name: "A Team".to_string(), team: Team::One },
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
                        player: ScoresEntryPlayer { name: "B Team".to_string(), team: Team::Two },
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
            winner: Winner { team: Some(Team::One) }
        };
        let expected = GameResult {
            winner: Some(Team::One),
            points: (TeamAndPoints(Team::One, 27), TeamAndPoints(Team::Two, 15)),
        };
        let actual = GameResult::from(game_result);
        assert_eq!(expected, actual);
    }
}
