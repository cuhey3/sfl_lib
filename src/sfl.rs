use crate::sfl::GameType::{PlayoffExtra, EXTRA, GENERAL, MID, VAN};
use crate::sfl::SflStage::{
    JP2024AllDivision, JP2024DivisionF, JP2024DivisionS, JP2024GrandFinal, JP2024Playoff,
};
use crate::sfl::SflTeam::*;
use std::cmp::PartialEq;
use std::collections::HashMap;
use std::fmt;

const K: f64 = 16_f64;

#[derive(Clone, Debug)]
pub struct SflRecord {
    pub sfl_match: SflMatch,
    pub set_number: u32,
    pub win_flag: bool,
    pub point: u32,
    pub game_type: GameType,
    // 有効なレコード（延長戦が行われなかったり大将戦が3本で終わると4本目、5本目はfalseになる）
    // TODO
    // is_disabled の方がわかりやすいのでできれば直す
    pub is_valid: bool,
    // 予想か実際かを区別する。予想ならtrue
    // これから予想する場合にもtrue。実績ならfalse
    pub is_prediction: bool,
}

impl SflRecord {
    // pub fn random_result(sfl_match: &SflMatch, rng: &mut StdRng) -> SflRecord {
    //     let required_battle = sfl_match.match_type.get_required_battle();
    //     let mut win_count = 0_u32;
    //     let mut lose_count = 0_u32;
    //     while win_count < required_battle && lose_count < required_battle {
    //         if rng.random() {
    //             win_count = win_count + 1;
    //         } else {
    //             lose_count = lose_count + 1;
    //         }
    //     }
    //     SflRecord {
    //         sfl_match: sfl_match.to_owned(),
    //         set: 0,
    //         win_count,
    //         lose_count,
    //         win_flag: win_count > lose_count,
    //         game_type: sfl_match.match_type.to_owned(),
    //         point: sfl_match.match_type.get_point(),
    //         is_valid: false,
    //         is_prediction: false,
    //     }
    // }
    // pub fn random_extra_result(sfl_match: &SflMatch, rng: &mut StdRng) -> SflRecord {
    //     let win_flag: bool = rng.random();
    //     SflRecord {
    //         sfl_match: SflMatch {
    //             section: sfl_match.section,
    //             branch: sfl_match.branch,
    //             sfl_stage: sfl_match.sfl_stage.to_owned(),
    //             team: sfl_match.team.to_owned(),
    //             opponent_team: sfl_match.opponent_team.to_owned(),
    //             is_home: sfl_match.is_home,
    //             match_type: EXTRA,
    //         },
    //         set: 0,
    //         win_count: if win_flag { 1 } else { 0 },
    //         lose_count: if win_flag { 0 } else { 1 },
    //         win_flag,
    //         point: EXTRA.get_point(),
    //         game_type: EXTRA,
    //         is_valid: false,
    //         is_prediction: false,
    //     }
    // }
    // pub fn update_record_by_simple_result(&self, win_flag: bool, is_valid: bool) -> SflRecord {
    //     let SflRecord { sfl_match, set, game_type, .. } = self.to_owned();
    //     SflRecord {
    //         sfl_match,
    //         set,
    //         win_count: if win_flag { 1 } else { 0 },
    //         lose_count: if win_flag { 0 } else { 1 },
    //         win_flag,
    //         point: 0,
    //         game_type,
    //         is_valid,
    //         is_prediction: false,
    //     }
    // }
}

#[derive(Clone, Debug)]
pub enum GameType {
    VAN,
    MID,
    GENERAL,
    EXTRA,
    PlayoffExtra,
}

impl GameType {
    fn get_point(&self) -> u32 {
        match self {
            VAN => 10,
            MID => 10,
            GENERAL => 20,
            EXTRA => 5,
            PlayoffExtra => 10,
        }
    }
    pub fn is_leader(&self) -> bool {
        match self {
            VAN | MID => false,
            GENERAL | EXTRA | PlayoffExtra => true,
        }
    }
    fn get_games_by_stage(sfl_stage: &SflStage) -> Vec<(u32, GameType)> {
        match sfl_stage {
            JP2024DivisionS | JP2024DivisionF | JP2024AllDivision => vec![
                (1, VAN),
                (2, VAN),
                (3, VAN),
                (1, MID),
                (2, MID),
                (3, MID),
                (1, GENERAL),
                (2, GENERAL),
                (3, GENERAL),
                (4, GENERAL),
                (5, GENERAL),
                (1, EXTRA),
            ],
            JP2024Playoff => vec![
                (1, VAN),
                (2, VAN),
                (3, VAN),
                (1, MID),
                (2, MID),
                (3, MID),
                (1, GENERAL),
                (2, GENERAL),
                (3, GENERAL),
                (4, GENERAL),
                (5, GENERAL),
                (1, VAN),
                (2, VAN),
                (3, VAN),
                (1, MID),
                (2, MID),
                (3, MID),
                (1, GENERAL),
                (2, GENERAL),
                (3, GENERAL),
                (4, GENERAL),
                (5, GENERAL),
                (1, VAN),
                (2, VAN),
                (3, VAN),
                (1, MID),
                (2, MID),
                (3, MID),
                (1, GENERAL),
                (2, GENERAL),
                (3, GENERAL),
                (4, GENERAL),
                (5, GENERAL),
                (1, PlayoffExtra),
                (2, PlayoffExtra),
                (3, PlayoffExtra),
            ],
            JP2024GrandFinal => vec![
                (1, VAN),
                (2, VAN),
                (3, VAN),
                (1, MID),
                (2, MID),
                (3, MID),
                (1, GENERAL),
                (2, GENERAL),
                (3, GENERAL),
                (4, GENERAL),
                (5, GENERAL),
                (1, VAN),
                (2, VAN),
                (3, VAN),
                (1, MID),
                (2, MID),
                (3, MID),
                (1, GENERAL),
                (2, GENERAL),
                (3, GENERAL),
                (4, GENERAL),
                (5, GENERAL),
                (1, VAN),
                (2, VAN),
                (3, VAN),
                (1, MID),
                (2, MID),
                (3, MID),
                (1, GENERAL),
                (2, GENERAL),
                (3, GENERAL),
                (4, GENERAL),
                (5, GENERAL),
                (1, VAN),
                (2, VAN),
                (3, VAN),
                (1, MID),
                (2, MID),
                (3, MID),
                (1, GENERAL),
                (2, GENERAL),
                (3, GENERAL),
                (4, GENERAL),
                (5, GENERAL),
                (1, PlayoffExtra),
                (2, PlayoffExtra),
                (3, PlayoffExtra),
            ],
            _ => vec![],
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum SflStage {
    JP2024DivisionS,
    JP2024DivisionF,
    JP2024AllDivision,
    JP2024Playoff,
    JP2024GrandFinal,
}

impl SflStage {
    pub fn get_teams(&self) -> Vec<SflTeam> {
        match self {
            JP2024DivisionS => vec![G8S, DFM, SOL, IBS, OJA, SNB],
            JP2024DivisionF => vec![CR, CAG, IXA, RC, VAR, FAV],
            JP2024AllDivision => vec![G8S, DFM, SOL, IBS, OJA, SNB, CR, CAG, IXA, RC, VAR, FAV],
            _ => vec![],
        }
    }

    pub fn get_initial_records(&self) -> Vec<Vec<SflRecord>> {
        self.get_matches()
            .iter()
            .map(|sfl_match| self.match_to_records(sfl_match))
            .collect()
    }

    pub fn get_playoff_records(&self, team: &SflTeam, opponent_team: &SflTeam) -> Vec<SflRecord> {
        let sfl_match = self.get_playoff_match(team, opponent_team);
        self.match_to_records(&sfl_match)
    }

    pub fn get_grand_final_records(
        &self,
        team: &SflTeam,
        opponent_team: &SflTeam,
    ) -> Vec<SflRecord> {
        let sfl_match = self.get_grand_final_match(team, opponent_team);
        self.match_to_records(&sfl_match)
    }
    pub fn get_playoff_match(&self, team: &SflTeam, opponent_team: &SflTeam) -> SflMatch {
        SflMatch {
            section: 0,
            branch: 0,
            date_expression: "".to_string(),
            sfl_stage: JP2024Playoff,
            team: team.to_owned(),
            opponent_team: opponent_team.to_owned(),
            is_home: false,
        }
    }

    pub fn get_grand_final_match(&self, team: &SflTeam, opponent_team: &SflTeam) -> SflMatch {
        SflMatch {
            section: 0,
            branch: 0,
            date_expression: "".to_string(),
            sfl_stage: JP2024GrandFinal,
            team: team.to_owned(),
            opponent_team: opponent_team.to_owned(),
            is_home: false,
        }
    }
    pub fn get_matches(&self) -> Vec<SflMatch> {
        match self {
            JP2024DivisionS => vec![
                ("08/16", 1, 1, DFM, OJA),
                ("08/16", 1, 2, G8S, SNB),
                ("08/16", 1, 3, SOL, IBS),
                ("08/27", 2, 1, SNB, DFM),
                ("08/27", 2, 2, IBS, OJA),
                ("08/27", 2, 3, SOL, G8S),
                ("09/03", 3, 1, OJA, SOL),
                ("09/03", 3, 2, G8S, DFM),
                ("09/03", 3, 3, SNB, IBS),
                ("09/10", 4, 1, G8S, OJA),
                ("09/10", 4, 2, SNB, SOL),
                ("09/10", 4, 3, IBS, DFM),
                ("09/20", 5, 1, IBS, G8S),
                ("09/20", 5, 2, DFM, SOL),
                ("09/20", 5, 3, OJA, SNB),
                ("10/04", 6, 1, IBS, SOL),
                ("10/04", 6, 2, SNB, G8S),
                ("10/04", 6, 3, OJA, DFM),
                ("10/22", 7, 1, G8S, SOL),
                ("10/22", 7, 2, DFM, SNB),
                ("10/22", 7, 3, OJA, IBS),
                ("10/29", 8, 1, IBS, SNB),
                ("10/29", 8, 2, SOL, OJA),
                ("10/29", 8, 3, DFM, G8S),
                ("11/05", 9, 1, OJA, G8S),
                ("11/05", 9, 2, DFM, IBS),
                ("11/05", 9, 3, SOL, SNB),
                ("11/19", 10, 1, SNB, OJA),
                ("11/19", 10, 2, SOL, DFM),
                ("11/19", 10, 3, G8S, IBS),
            ],

            JP2024DivisionF => vec![
                ("08/20", 1, 1, RC, IXA),
                ("08/20", 1, 2, CAG, VAR),
                ("08/20", 1, 3, CR, FAV),
                ("08/30", 2, 1, VAR, RC),
                ("08/30", 2, 2, FAV, IXA),
                ("08/30", 2, 3, CR, CAG),
                ("09/06", 3, 1, IXA, CR),
                ("09/06", 3, 2, CAG, RC),
                ("09/06", 3, 3, VAR, FAV),
                ("09/18", 4, 1, CAG, IXA),
                ("09/18", 4, 2, VAR, CR),
                ("09/18", 4, 3, FAV, RC),
                ("10/01", 5, 1, FAV, CAG),
                ("10/01", 5, 2, RC, CR),
                ("10/01", 5, 3, IXA, VAR),
                ("10/08", 6, 1, FAV, CR),
                ("10/08", 6, 2, VAR, CAG),
                ("10/08", 6, 3, IXA, RC),
                ("10/25", 7, 1, CAG, CR),
                ("10/25", 7, 2, RC, VAR),
                ("10/25", 7, 3, IXA, FAV),
                ("11/01", 8, 1, FAV, VAR),
                ("11/01", 8, 2, CR, IXA),
                ("11/01", 8, 3, RC, CAG),
                ("11/15", 9, 1, IXA, CAG),
                ("11/15", 9, 2, RC, FAV),
                ("11/15", 9, 3, CR, VAR),
                ("11/22", 10, 1, VAR, IXA),
                ("11/22", 10, 2, CR, RC),
                ("11/22", 10, 3, CAG, FAV),
            ],

            JP2024AllDivision => vec![
                ("08/16", 1, 1, DFM, OJA),
                ("08/16", 1, 2, G8S, SNB),
                ("08/16", 1, 3, SOL, IBS),
                ("08/20", 1, 1, RC, IXA),
                ("08/20", 1, 2, CAG, VAR),
                ("08/20", 1, 3, CR, FAV),
                ("08/27", 2, 1, SNB, DFM),
                ("08/27", 2, 2, IBS, OJA),
                ("08/27", 2, 3, SOL, G8S),
                ("08/30", 2, 1, VAR, RC),
                ("08/30", 2, 2, FAV, IXA),
                ("08/30", 2, 3, CR, CAG),
                ("09/03", 3, 1, OJA, SOL),
                ("09/03", 3, 2, G8S, DFM),
                ("09/03", 3, 3, SNB, IBS),
                ("09/06", 3, 1, IXA, CR),
                ("09/06", 3, 2, CAG, RC),
                ("09/06", 3, 3, VAR, FAV),
                ("09/10", 4, 1, G8S, OJA),
                ("09/10", 4, 2, SNB, SOL),
                ("09/10", 4, 3, IBS, DFM),
                ("09/18", 4, 1, CAG, IXA),
                ("09/18", 4, 2, VAR, CR),
                ("09/18", 4, 3, FAV, RC),
                ("09/20", 5, 1, IBS, G8S),
                ("09/20", 5, 2, DFM, SOL),
                ("09/20", 5, 3, OJA, SNB),
                ("10/01", 5, 1, FAV, CAG),
                ("10/01", 5, 2, RC, CR),
                ("10/01", 5, 3, IXA, VAR),
                ("10/04", 6, 1, IBS, SOL),
                ("10/04", 6, 2, SNB, G8S),
                ("10/04", 6, 3, OJA, DFM),
                ("10/08", 6, 1, FAV, CR),
                ("10/08", 6, 2, VAR, CAG),
                ("10/08", 6, 3, IXA, RC),
                ("10/22", 7, 1, G8S, SOL),
                ("10/22", 7, 2, DFM, SNB),
                ("10/22", 7, 3, OJA, IBS),
                ("10/25", 7, 1, CAG, CR),
                ("10/25", 7, 2, RC, VAR),
                ("10/25", 7, 3, IXA, FAV),
                ("10/29", 8, 1, IBS, SNB),
                ("10/29", 8, 2, SOL, OJA),
                ("10/29", 8, 3, DFM, G8S),
                ("11/01", 8, 1, FAV, VAR),
                ("11/01", 8, 2, CR, IXA),
                ("11/01", 8, 3, RC, CAG),
                ("11/05", 9, 1, OJA, G8S),
                ("11/05", 9, 2, DFM, IBS),
                ("11/05", 9, 3, SOL, SNB),
                ("11/15", 9, 1, IXA, CAG),
                ("11/15", 9, 2, RC, FAV),
                ("11/15", 9, 3, CR, VAR),
                ("11/19", 10, 1, SNB, OJA),
                ("11/19", 10, 2, SOL, DFM),
                ("11/19", 10, 3, G8S, IBS),
                ("11/22", 10, 1, VAR, IXA),
                ("11/22", 10, 2, CR, RC),
                ("11/22", 10, 3, CAG, FAV),
            ],
            _ => {
                vec![]
            }
        }
        .iter()
        .map(|tup| {
            let (date_expression, section, branch, team, opponent_team) = tup.to_owned();
            SflMatch {
                section,
                branch,
                date_expression: date_expression.to_string(),
                sfl_stage: self.to_owned(),
                team,
                opponent_team,
                is_home: false,
            }
        })
        .collect()
    }
    fn match_to_records(&self, sfl_match: &SflMatch) -> Vec<SflRecord> {
        match sfl_match.sfl_stage {
            JP2024DivisionS | JP2024DivisionF | JP2024AllDivision | JP2024Playoff
            | JP2024GrandFinal => {
                GameType::get_games_by_stage(&sfl_match.sfl_stage)
                    .iter()
                    .map(|(set_number, game_type)| {
                        SflRecord {
                            sfl_match: sfl_match.to_owned(),
                            set_number: *set_number,
                            win_flag: false,
                            game_type: game_type.to_owned(),
                            // pointはcorrect_recordでセットする
                            point: 0,
                            is_valid: false,
                            is_prediction: true,
                        }
                    })
                    .collect()
            }
            _ => vec![],
        }
    }
    // パフォーマンスの問題もあるから前後の関連だけ見て修正する
    // is_valid = true フラグが立っているレコードについて見直して一部 is_valid = false に変える
    // ポイントを決着セットに書き加える
    // 決着していない場合はもちろんポイントを書かない
    // ランダム結果と実際結果が混じることがある
    pub fn correct_records(&self, records: &mut Vec<SflRecord>) {
        match self {
            JP2024DivisionS | JP2024DivisionF | JP2024AllDivision => {
                let van1 = records.get(0).unwrap().to_owned();
                let van2 = records.get(1).unwrap().to_owned();
                let van3 = records.get(2).unwrap().to_owned();
                let mid1 = records.get(3).unwrap().to_owned();
                let mid2 = records.get(4).unwrap().to_owned();
                let mid3 = records.get(5).unwrap().to_owned();
                let general1 = records.get(6).unwrap().to_owned();
                let general2 = records.get(7).unwrap().to_owned();
                let general3 = records.get(8).unwrap().to_owned();
                let general4 = records.get(9).unwrap().to_owned();
                let general5 = records.get(10).unwrap().to_owned();

                let mut team_point: u32 = 0;
                let mut opponent_team_point: u32 = 0;

                // 先鋒戦のポイント決定
                let van_point = VAN.get_point();
                if van1.is_valid && van2.is_valid {
                    if van1.win_flag == van2.win_flag {
                        let mut_van2 = records.get_mut(1).unwrap();
                        mut_van2.point = van_point;
                        let mut_van3 = records.get_mut(2).unwrap();
                        mut_van3.is_valid = false;
                        // ポイントのリセットはここではしない
                        // mut_van3.point = 0;
                        if van1.win_flag {
                            team_point += van_point;
                        } else {
                            opponent_team_point += van_point;
                        }
                    } else if van3.is_valid {
                        let mut_van3 = records.get_mut(2).unwrap();
                        mut_van3.is_valid = true;
                        mut_van3.point = van_point;
                        // ポイントのリセットはここではしない
                        // let mut_van2 = records.get_mut(1).unwrap();
                        // mut_van2.point = 0;
                        if van3.win_flag {
                            team_point += van_point;
                        } else {
                            opponent_team_point += van_point;
                        }
                    }
                }

                // 中堅戦のポイント決定
                let mid_point = MID.get_point();
                if mid1.is_valid && mid2.is_valid {
                    if mid1.win_flag == mid2.win_flag {
                        let mut_mid2 = records.get_mut(4).unwrap();
                        mut_mid2.point = mid_point;
                        let mut_mid3 = records.get_mut(5).unwrap();
                        mut_mid3.is_valid = false;
                        // ポイントのリセットはここではしない
                        // mut_mid3.point = 0;
                        if mid1.win_flag {
                            team_point += mid_point;
                        } else {
                            opponent_team_point += mid_point;
                        }
                    } else if mid3.is_valid {
                        let mut_mid3 = records.get_mut(5).unwrap();
                        mut_mid3.is_valid = true;
                        mut_mid3.point = mid_point;
                        // ポイントのリセットはここではしない
                        // let mut_mid2 = records.get_mut(4).unwrap();
                        // mut_mid2.point = 0;
                        if mid3.win_flag {
                            team_point += mid_point;
                        } else {
                            opponent_team_point += mid_point;
                        }
                    }
                }

                // 大将戦
                let general_point = GENERAL.get_point();
                if general1.is_valid && general2.is_valid && general3.is_valid {
                    if general1.win_flag == general2.win_flag
                        && general1.win_flag == general3.win_flag
                    {
                        let mut_general3 = records.get_mut(8).unwrap();
                        mut_general3.point = general_point;
                        let mut_general4 = records.get_mut(9).unwrap();
                        mut_general4.is_valid = false;
                        // ポイントのリセットはここではしない
                        // mut_general4.point = 0;
                        let mut_general5 = records.get_mut(10).unwrap();
                        mut_general5.is_valid = false;
                        // ポイントのリセットはここではしない
                        // mut_general5.point = 0;
                        if general1.win_flag {
                            team_point += general_point;
                        } else {
                            opponent_team_point += general_point;
                        }
                    } else if general4.is_valid {
                        let decide_flag = (general4.win_flag == general1.win_flag
                            && general4.win_flag == general2.win_flag
                            && general4.win_flag != general3.win_flag)
                            || (general4.win_flag == general1.win_flag
                                && general4.win_flag != general2.win_flag
                                && general4.win_flag == general3.win_flag)
                            || (general4.win_flag != general1.win_flag
                                && general4.win_flag == general2.win_flag
                                && general4.win_flag == general3.win_flag);
                        if decide_flag {
                            // ポイントのリセットはここではしない
                            let mut_general5 = records.get_mut(10).unwrap();
                            mut_general5.is_valid = false;
                            let mut_general4 = records.get_mut(9).unwrap();
                            mut_general4.is_valid = true;
                            mut_general4.point = general_point;
                            // mut_general5.point = 0;
                            if mut_general4.win_flag {
                                team_point += general_point;
                            } else {
                                opponent_team_point += general_point;
                            }
                        } else {
                            if general5.is_valid {
                                let mut_general5 = records.get_mut(10).unwrap();
                                mut_general5.is_valid = true;
                                mut_general5.point = general_point;
                                // ポイントのリセットはここではしない
                                // let mut_general4 = records.get_mut(9).unwrap();
                                // mut_general4.point = 0;
                                if mut_general5.win_flag {
                                    team_point += general_point;
                                } else {
                                    opponent_team_point += general_point;
                                }
                            }
                        }
                    }
                }

                // 延長戦
                let mut_extra1 = records.get_mut(11).unwrap();
                mut_extra1.is_valid = if team_point == van_point + mid_point
                    && opponent_team_point == general_point
                {
                    mut_extra1.point = EXTRA.get_point();
                    true
                } else {
                    false
                };
            }
            JP2024Playoff => {}
            _ => {}
        }
    }
    pub fn get_win_team(&self, records: &mut Vec<SflRecord>) -> (SflTeam, u32, u32) {
        match self {
            JP2024Playoff => {
                let games = [
                    (vec![0_usize, 1, 2], 1_u32),
                    (vec![3, 4, 5], 1),
                    (vec![6, 7, 8, 9, 10], 2),
                    (vec![11, 12, 13], 1),
                    (vec![14, 15, 16], 1),
                    (vec![17, 18, 19, 20, 21], 2),
                    (vec![22, 23, 24], 1),
                    (vec![25, 26, 27], 1),
                    (vec![28, 29, 30, 31, 32], 2),
                    (vec![33, 34, 35], 1),
                ];
                let mut team_point = 0_u32;
                let mut opponent_team_point = 0_u32;
                for (game, p) in games.iter() {
                    let won = game
                        .iter()
                        .filter(|index| records[**index].win_flag)
                        .collect::<Vec<_>>()
                        .len() as u32;
                    if won > *p {
                        team_point += *p * 10;
                        if team_point >= 70 {
                            return (
                                records[0].sfl_match.team.to_owned(),
                                team_point,
                                opponent_team_point,
                            );
                        }
                    } else {
                        opponent_team_point += *p * 10;
                        if opponent_team_point >= 70 {
                            return (
                                records[0].sfl_match.opponent_team.to_owned(),
                                team_point,
                                opponent_team_point,
                            );
                        }
                    }
                }
                panic!()
            }
            JP2024GrandFinal => {
                let games = [
                    (vec![0_usize, 1, 2], 1_u32),
                    (vec![3, 4, 5], 1),
                    (vec![6, 7, 8, 9, 10], 2),
                    (vec![11, 12, 13], 1),
                    (vec![14, 15, 16], 1),
                    (vec![17, 18, 19, 20, 21], 2),
                    (vec![22, 23, 24], 1),
                    (vec![25, 26, 27], 1),
                    (vec![28, 29, 30, 31, 32], 2),
                    (vec![33, 34, 35], 1),
                    (vec![36, 37, 38], 1),
                    (vec![39, 40, 41, 42, 43], 2),
                    (vec![44, 45, 46], 1),
                ];
                let mut team_point = 0_u32;
                let mut opponent_team_point = 0_u32;
                for (game, p) in games.iter() {
                    let won = game
                        .iter()
                        .filter(|index| records[**index].win_flag)
                        .collect::<Vec<_>>()
                        .len() as u32;
                    if won > *p {
                        team_point += *p * 10;
                        if team_point >= 90 {
                            return (
                                records[0].sfl_match.team.to_owned(),
                                team_point,
                                opponent_team_point,
                            );
                        }
                    } else {
                        opponent_team_point += *p * 10;
                        if opponent_team_point >= 90 {
                            return (
                                records[0].sfl_match.opponent_team.to_owned(),
                                team_point,
                                opponent_team_point,
                            );
                        }
                    }
                }
                panic!()
            }
            _ => {
                panic!()
            }
        }
    }
}

#[derive(Clone, Debug)]
pub struct SflMatch {
    // 節
    pub section: u32,
    // 節内の順序
    pub branch: u32,
    pub date_expression: String,
    pub sfl_stage: SflStage,
    pub team: SflTeam,
    pub opponent_team: SflTeam,
    pub(crate) is_home: bool,
}

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
pub enum SflTeam {
    G8S,
    DFM,
    SOL,
    IBS,
    OJA,
    SNB,
    CR,
    CAG,
    IXA,
    RC,
    VAR,
    FAV,
}

impl SflTeam {
    pub fn get_index(&self) -> usize {
        match self {
            G8S => 0,
            DFM => 1,
            SOL => 2,
            IBS => 3,
            OJA => 4,
            SNB => 5,
            CR => 6,
            CAG => 7,
            IXA => 8,
            RC => 9,
            VAR => 10,
            FAV => 11,
        }
    }
}
impl fmt::Display for SflTeam {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub enum SflRatingSetting {
    TeamOnly,
    HomeAway,
    GameType,
    HomeAwayGameType,
}

pub fn create_key_function_and_init_rating_map(
    setting: SflRatingSetting,
    teams: Vec<SflTeam>,
) -> (
    fn(&SflRecord) -> ((SflTeam, u8), (SflTeam, u8)),
    HashMap<(SflTeam, u8), f64>,
) {
    let default_rating = 1500_f64;
    let mut rating_map: HashMap<(SflTeam, u8), f64> = HashMap::new();
    match setting {
        SflRatingSetting::TeamOnly => {
            for team in teams.iter() {
                rating_map.insert((team.to_owned(), 000_u8), default_rating);
            }
            fn team_only_function(record: &SflRecord) -> ((SflTeam, u8), (SflTeam, u8)) {
                (
                    (record.sfl_match.team.to_owned(), 000_u8),
                    (record.sfl_match.opponent_team.to_owned(), 000_u8),
                )
            }
            (team_only_function, rating_map)
        }
        SflRatingSetting::HomeAway => {
            for team in teams.iter() {
                rating_map.insert((team.to_owned(), 120_u8), default_rating);
                rating_map.insert((team.to_owned(), 121_u8), default_rating);
            }
            fn home_away_function(record: &SflRecord) -> ((SflTeam, u8), (SflTeam, u8)) {
                if record.sfl_match.is_home {
                    (
                        (record.sfl_match.team.to_owned(), 121_u8),
                        (record.sfl_match.opponent_team.to_owned(), 120_u8),
                    )
                } else {
                    (
                        (record.sfl_match.team.to_owned(), 120_u8),
                        (record.sfl_match.opponent_team.to_owned(), 121_u8),
                    )
                }
            }
            (home_away_function, rating_map)
        }
        SflRatingSetting::GameType => {
            for team in teams.iter() {
                rating_map.insert((team.to_owned(), 102_u8), default_rating);
                rating_map.insert((team.to_owned(), 112_u8), default_rating);
            }
            fn game_type_function(record: &SflRecord) -> ((SflTeam, u8), (SflTeam, u8)) {
                if record.game_type.is_leader() {
                    (
                        (record.sfl_match.team.to_owned(), 112_u8),
                        (record.sfl_match.opponent_team.to_owned(), 112_u8),
                    )
                } else {
                    (
                        (record.sfl_match.team.to_owned(), 102_u8),
                        (record.sfl_match.opponent_team.to_owned(), 102_u8),
                    )
                }
            }
            (game_type_function, rating_map)
        }
        SflRatingSetting::HomeAwayGameType => {
            for team in teams.iter() {
                for n in [100_u8, 101_u8, 110_u8, 111_u8] {
                    rating_map.insert((team.to_owned(), n), default_rating);
                }
            }
            fn home_away_game_type_function(record: &SflRecord) -> ((SflTeam, u8), (SflTeam, u8)) {
                if record.sfl_match.is_home {
                    if record.game_type.is_leader() {
                        (
                            (record.sfl_match.team.to_owned(), 111_u8),
                            (record.sfl_match.opponent_team.to_owned(), 110_u8),
                        )
                    } else {
                        (
                            (record.sfl_match.team.to_owned(), 101_u8),
                            (record.sfl_match.opponent_team.to_owned(), 100_u8),
                        )
                    }
                } else {
                    if record.game_type.is_leader() {
                        (
                            (record.sfl_match.team.to_owned(), 110_u8),
                            (record.sfl_match.opponent_team.to_owned(), 111_u8),
                        )
                    } else {
                        (
                            (record.sfl_match.team.to_owned(), 100_u8),
                            (record.sfl_match.opponent_team.to_owned(), 101_u8),
                        )
                    }
                }
            }
            (home_away_game_type_function, rating_map)
        }
    }
}

pub fn create_key_function_and_init_ratings(
    setting: SflRatingSetting,
    teams: Vec<SflTeam>,
) -> (fn(&SflRecord) -> (usize, usize), Vec<f64>) {
    let max_team_index = teams.iter().map(|team| team.get_index()).max().unwrap();
    let mut ratings: Vec<f64> = vec![1500_f64; (max_team_index + 1) * 4];
    match setting {
        SflRatingSetting::TeamOnly => {
            todo!()
        }
        SflRatingSetting::HomeAway => {
            todo!()
        }
        SflRatingSetting::GameType => {
            todo!()
        }
        SflRatingSetting::HomeAwayGameType => {
            fn home_away_game_type_function(record: &SflRecord) -> (usize, usize) {
                let team_index = record.sfl_match.team.get_index() * 4;
                let opponent_team_index = record.sfl_match.opponent_team.get_index() * 4;
                let mut team_mod_index = 0_usize;
                let mut opponent_team_mod_index = 0_usize;
                if record.sfl_match.is_home {
                    team_mod_index += 1;
                } else {
                    opponent_team_mod_index += 1;
                }
                if record.game_type.is_leader() {
                    team_mod_index += 2;
                    opponent_team_mod_index += 2;
                }
                (
                    team_index + team_mod_index,
                    opponent_team_index + opponent_team_mod_index,
                )
            }
            (home_away_game_type_function, ratings)
        }
    }
}

struct SflSimulationResult {}

pub fn simulate(records: Vec<SflRecord>) -> SflSimulationResult {
    SflSimulationResult {}
}

pub fn get_place_sim_count(
    sfl_stage: SflStage,
) -> HashMap<SflTeam, (Vec<u32>, (u32, u32, i32, i32))> {
    let mut count: HashMap<SflTeam, (Vec<u32>, (u32, u32, i32, i32))> = HashMap::new();
    for team in sfl_stage.get_teams().into_iter() {
        count.insert(team, (vec![0; 6], (0, 0, 0, 0)));
    }
    count
}

pub fn update_rating(a_rate: &f64, b_rate: &f64, a_win: &bool) -> (f64, f64) {
    let a_win_percentage = 1_f64 / (10_f64.powf((b_rate - a_rate) / 400_f64) + 1_f64);
    if *a_win {
        let b_win_percentage = 1_f64 - a_win_percentage;
        let a_win_increment = b_win_percentage * K;
        (a_rate + a_win_increment, b_rate - a_win_increment)
    } else {
        let b_win_increment = a_win_percentage * K;
        (a_rate - b_win_increment, b_rate + b_win_increment)
    }
}

pub fn get_win_percentage(a_rate: f64, b_rate: f64) -> (f64, f64) {
    let a_win_percentage = 1_f64 / (10_f64.powf((b_rate - a_rate) / 400_f64) + 1_f64);
    (a_win_percentage, 1_f64 - a_win_percentage)
}
