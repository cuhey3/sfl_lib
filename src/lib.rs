pub mod sfl;
mod utils;
use crate::sfl::SflRatingSetting::HomeAwayGameType;
use crate::sfl::SflStage::{
    JP2024AllDivision, JP2024DivisionF, JP2024DivisionS, JP2024GrandFinal, JP2024Playoff,
};
use crate::sfl::{
    create_key_function_and_init_ratings, get_win_percentage, update_rating, SflMatch,
    SflRatingSetting, SflRecord, SflStage, SflTeam,
};
use rand::prelude::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen_test::console_log;

#[wasm_bindgen]
pub struct SflRating {
    ratings: Vec<f64>,
}

impl SflRating {
    pub fn new() -> SflRating {
        SflRating { ratings: vec![] }
    }
    pub fn calc_ratings(&mut self, sfl_stage: &SflStage, sfl_records: &Vec<Vec<SflRecord>>) {
        let (rate_index_function, mut ratings) =
            create_key_function_and_init_ratings(HomeAwayGameType, sfl_stage.get_teams());
        for records in sfl_records.iter() {
            for record in records.into_iter() {
                // 無効なセットおよび予想のセットは無視
                // ただし1マッチ最大12セットのうち、途中のセットが無効になることはあるので、breakはしない
                if !record.is_valid || record.is_prediction {
                    continue;
                }
                let (team_index, opponent_team_index) = rate_index_function(record);
                let team_rating = ratings.get(team_index).unwrap();
                let opponent_team_rating = ratings.get(opponent_team_index).unwrap();
                let (updated_rating, updated_opponent_rating) =
                    update_rating(team_rating, opponent_team_rating, &record.win_flag);
                ratings[team_index] = updated_rating;
                ratings[opponent_team_index] = updated_opponent_rating;
            }
        }
        self.ratings = ratings;
    }
    pub fn get_rating(&self, team_index: usize, is_home: bool, is_reader: bool) -> f64 {
        let index = team_index * 4 + if is_home { 1 } else { 0 } + if is_reader { 2 } else { 0 };
        self.ratings[index]
    }
}

#[derive(Copy, Clone)]
#[wasm_bindgen]
pub struct SflStats {
    points: [u32; 12],
    battles: [i32; 12],
}

#[wasm_bindgen]
impl SflStats {
    fn new() -> SflStats {
        SflStats {
            points: [0_u32; 12],
            battles: [0_i32; 12],
        }
    }
    pub fn get_points(&self) -> Vec<u32> {
        self.points.to_vec()
    }
    pub fn get_battles(&self) -> Vec<i32> {
        self.battles.to_vec()
    }
}

pub struct SflSimulationResult {
    pub division_place_count: Vec<Vec<u32>>,
    pub division_points_battles: Vec<Vec<i32>>,
    pub playoff_place_count: Vec<Vec<u32>>,
    pub match_points: Vec<Vec<u32>>,
    pub division_places: Vec<Vec<usize>>,
    pub playoff_places: Vec<usize>,
}

impl SflSimulationResult {
    pub fn new() -> SflSimulationResult {
        SflSimulationResult {
            division_place_count: vec![vec![0_u32; 6]; 12],
            division_points_battles: vec![vec![0_i32; 2]; 12],
            playoff_place_count: vec![vec![0_u32; 4]; 12],
            match_points: vec![vec![0_u32; 4]; 60],
            division_places: vec![vec![]; 2],
            playoff_places: vec![],
        }
    }

    pub fn current_simulated_result() -> SflSimulationResult {
        SflSimulationResult {
            division_places: vec![vec![5, 0, 2, 4, 3, 1], vec![9, 7, 8, 6, 10, 11]],
            playoff_places: vec![9, 0, 5, 2, 7, 8, 6, 10, 4, 3, 1, 11],
            division_place_count: vec![
                vec![3035, 3480, 2484, 765, 223, 13],
                vec![1, 8, 48, 420, 1833, 7690],
                vec![2470, 2681, 3492, 1105, 218, 34],
                vec![54, 225, 844, 3401, 4030, 1446],
                vec![129, 445, 1270, 3760, 3592, 804],
                vec![4311, 3161, 1862, 549, 104, 13],
                vec![414, 1045, 1982, 3655, 2747, 157],
                vec![3492, 2980, 2155, 1036, 333, 4],
                vec![1935, 2683, 2936, 1747, 687, 12],
                vec![4039, 2923, 1913, 864, 259, 2],
                vec![120, 369, 1014, 2624, 5432, 441],
                vec![0, 0, 0, 74, 542, 9384],
            ],
            division_points_battles: vec![
                vec![2389360, 114442],
                vec![1427990, -119575],
                vec![2329985, 86395],
                vec![1818825, -53577],
                vec![1900280, -101426],
                vec![2481925, 73741],
                vec![2063780, 1422],
                vec![2475185, 89294],
                vec![2369615, -686],
                vec![2499925, 116162],
                vec![1864545, 10012],
                vec![1168085, -216204],
            ],
            playoff_place_count: vec![
                vec![1807, 1297, 2928, 2967],
                vec![4, 8, 9, 36],
                vec![1490, 1573, 2745, 2835],
                vec![45, 136, 306, 636],
                vec![97, 228, 497, 1022],
                vec![1791, 1524, 3515, 2504],
                vec![369, 413, 1029, 1630],
                vec![1437, 1675, 3119, 2396],
                vec![774, 1011, 2539, 3230],
                vec![2076, 2002, 2897, 1900],
                vec![110, 133, 416, 844],
                vec![0, 0, 0, 0],
            ],
            match_points: vec![
                vec![200000, 0, 50000, 200000],
                vec![200000, 0, 200000, 0],
                vec![200000, 0, 50000, 200000],
                vec![100000, 100000, 0, 200000],
                vec![100000, 100000, 200000, 0],
                vec![100000, 100000, 200000, 0],
                vec![100000, 100000, 200000, 0],
                vec![200000, 0, 0, 250000],
                vec![100000, 100000, 200000, 0],
                vec![100000, 100000, 0, 200000],
                vec![0, 200000, 0, 200000],
                vec![0, 200000, 0, 200000],
                vec![100000, 100000, 200000, 0],
                vec![200000, 0, 200000, 0],
                vec![0, 200000, 200000, 50000],
                vec![200000, 0, 0, 250000],
                vec![200000, 0, 0, 250000],
                vec![200000, 0, 200000, 0],
                vec![100000, 100000, 200000, 0],
                vec![100000, 100000, 200000, 0],
                vec![100000, 100000, 200000, 0],
                vec![0, 200000, 200000, 50000],
                vec![0, 200000, 0, 200000],
                vec![0, 200000, 0, 200000],
                vec![100000, 100000, 0, 200000],
                vec![100000, 100000, 0, 200000],
                vec![0, 200000, 0, 200000],
                vec![0, 200000, 200000, 50000],
                vec![100000, 100000, 200000, 0],
                vec![0, 200000, 200000, 50000],
                vec![0, 200000, 0, 200000],
                vec![200000, 0, 200000, 0],
                vec![100000, 100000, 200000, 0],
                vec![79270, 120730, 90695, 121635],
                vec![81670, 118330, 98615, 113815],
                vec![98710, 101290, 65900, 146540],
                vec![101930, 98070, 127875, 84285],
                vec![104020, 95980, 94775, 118130],
                vec![83740, 116260, 110330, 102190],
                vec![100690, 99310, 95815, 116880],
                vec![88620, 111380, 121770, 91170],
                vec![107450, 92550, 114140, 97970],
                vec![89700, 110300, 95465, 117075],
                vec![116560, 83440, 112245, 100390],
                vec![117540, 82460, 96550, 116715],
                vec![66780, 133220, 113330, 101060],
                vec![73250, 126750, 109575, 103855],
                vec![81990, 118010, 121065, 92775],
                vec![96660, 103340, 106455, 105530],
                vec![106040, 93960, 101680, 110800],
                vec![101170, 98830, 113520, 98915],
                vec![75890, 124110, 93785, 118435],
                vec![112870, 87130, 139975, 71800],
                vec![75030, 124970, 120660, 93135],
                vec![108110, 91890, 134585, 77375],
                vec![116350, 83650, 137785, 73735],
                vec![110710, 89290, 140800, 71160],
                vec![87600, 112400, 91725, 120735],
                vec![89640, 110360, 87070, 125445],
                vec![110180, 89820, 133025, 78740],
            ],
        }
    }
}
#[wasm_bindgen]
pub struct SflSimulation {
    pub count: usize,
    pub option: SimulationOption,
    pub sfl_stage: SflStage,
    sfl_records: Vec<Vec<SflRecord>>,
    #[wasm_bindgen(skip)]
    pub sfl_rating: SflRating,
    pub max_team_index: usize,
    pub sfl_stats: SflStats,
    result: SflSimulationResult,
}

#[wasm_bindgen]
impl SflSimulation {
    #[wasm_bindgen(constructor)]
    pub fn new(simulated: bool) -> SflSimulation {
        let sfl_matches = JP2024AllDivision.get_matches();
        let sfl_records = sfl_matches
            .iter()
            .map(|sfl_match| sfl_match.to_records())
            .collect();
        let max_team_index: usize = JP2024AllDivision.get_max_team_index();
        SflSimulation {
            count: 10000,
            option: SimulationOption { enable_rate: true },
            sfl_stage: JP2024AllDivision,
            sfl_records,
            sfl_rating: SflRating::new(),
            max_team_index,
            sfl_stats: SflStats::new(),
            result: if simulated {
                SflSimulationResult::current_simulated_result()
            } else {
                SflSimulationResult::new()
            },
        }
    }

    pub fn get_team_names(&self, stage: SflStage) -> Vec<String> {
        stage
            .get_teams()
            .iter()
            .map(|team| format!("{:?}", team))
            .collect()
    }
    pub fn enable_rate(&mut self, flag: bool) {
        self.option = SimulationOption { enable_rate: flag }
    }
    pub fn get_matches(&self) -> Vec<SflMatch> {
        self.sfl_stage.get_matches()
    }
    pub fn get_match_records(&self, match_index: usize) -> Vec<SflRecord> {
        self.sfl_records[match_index].to_owned()
    }
    pub fn get_match_points(&self, match_index: usize) -> Vec<u32> {
        for (index, m) in self.result.match_points.iter().enumerate() {
            if index == match_index {
                return m.to_owned();
            }
        }
        vec![]
    }
    pub fn set_match_result(&mut self, match_index: usize, results: Vec<JsValue>) {
        self.sfl_records[match_index]
            .iter_mut()
            .enumerate()
            .for_each(|(index, record)| {
                let record_result = results.get(index);
                if record_result.is_none() {
                    // 入力の長さが足りない分は初期状態に戻す
                    record.point = 0;
                    record.win_flag = false;
                    record.is_valid = false;
                    record.is_prediction = true;
                } else {
                    // ポイントは更新するたび初期化
                    record.point = 0;
                    record.win_flag = record_result.unwrap().as_bool().unwrap();
                    record.is_valid = true;
                    record.is_prediction = false;
                }
            });
        for records in self.sfl_records.iter_mut() {
            self.sfl_stage.correct_records(records);
        }
        self.calc_ratings();
        self.update_stats();
    }

    fn calc_ratings(&mut self) {
        self.sfl_rating
            .calc_ratings(&self.sfl_stage, &self.sfl_records)
    }

    pub fn get_rating(&self, team_index: usize, is_home: bool, is_reader: bool) -> f64 {
        self.sfl_rating.get_rating(team_index, is_home, is_reader)
    }
    fn update_stats(&mut self) {
        // チームごとに現在ポイントと現在バトル得失を集計
        for team in self.sfl_stage.get_teams() {
            let team_index = team.get_index();
            // チームが含まれる有効なレコードのみ抽出
            let records: Vec<&SflRecord> = self
                .sfl_records
                .iter()
                .flatten()
                .filter(|r| {
                    r.is_valid
                        && !r.is_prediction
                        && ((r.sfl_match.team == team) || (r.sfl_match.opponent_team == team))
                })
                .collect();
            // 現在ポイントを集計
            let point: u32 = records
                .iter()
                .filter(|r| {
                    r.point != 0
                        && ((r.win_flag && r.sfl_match.team == team)
                            || (!r.win_flag && r.sfl_match.opponent_team == team))
                })
                .map(|r| r.point)
                .sum();
            // 現在バトル得失を集計
            let battle: i32 = records
                .iter()
                .map(|r| {
                    if r.win_flag {
                        if r.sfl_match.team == team {
                            1
                        } else {
                            -1
                        }
                    } else {
                        if r.sfl_match.opponent_team == team {
                            1
                        } else {
                            -1
                        }
                    }
                })
                .sum();
            self.sfl_stats.points[team_index] = point;
            self.sfl_stats.battles[team_index] = battle;
        }
    }

    pub fn get_current_places(&self, stage: SflStage) -> Vec<usize> {
        let mut division_s_places = vec![];
        let mut division_f_places = vec![];
        for (index, d) in self.result.division_places.iter().enumerate() {
            if index == 0 {
                for p in d.iter() {
                    division_s_places.push(*p)
                }
            } else {
                for p in d.iter() {
                    division_f_places.push(*p)
                }
            }
        }

        match stage {
            JP2024DivisionS => division_s_places,
            JP2024DivisionF => division_f_places,
            JP2024AllDivision => {
                let mut playoff_places = vec![];
                for p in self.result.playoff_places.iter() {
                    playoff_places.push(*p)
                }
                playoff_places
            }
            _ => vec![],
        }
    }

    pub fn get_expect_point(&self, team_index: usize) -> i32 {
        self.result.division_points_battles[team_index][0]
    }
    pub fn get_expect_battle(&self, team_index: usize) -> i32 {
        self.result.division_points_battles[team_index][1]
    }
    pub fn get_place_count(&self, sfl_stage: SflStage, team_index: usize) -> Vec<u32> {
        match sfl_stage {
            JP2024DivisionS | JP2024DivisionF => {
                for (index, count) in self.result.division_place_count.iter().enumerate() {
                    if index == team_index {
                        return count.to_owned();
                    }
                }
                vec![]
            }
            JP2024AllDivision => {
                for (index, count) in self.result.playoff_place_count.iter().enumerate() {
                    if index == team_index {
                        return count.to_owned();
                    }
                }
                vec![]
            }
            _ => vec![],
        }
    }
    pub fn simulate(&mut self, output_flag: bool) {
        let seed: [u8; 32] = [5; 32];
        let mut rng: StdRng = rand::SeedableRng::from_seed(seed);
        self.result = SflSimulationResult::new();
        for _ in 0..10000 {
            self.simulate_one_time(&mut rng);
        }
        let mut division_places: Vec<Vec<usize>> = [JP2024DivisionS, JP2024DivisionF]
            .iter()
            .map(|division| {
                division
                    .get_teams()
                    .iter()
                    .map(|team| team.get_index())
                    .collect()
            })
            .collect();
        for places in division_places.iter_mut() {
            places.sort_by(|team_index_a, team_index_b| {
                self.result.division_points_battles[*team_index_b][0]
                    .cmp(&self.result.division_points_battles[*team_index_a][0])
            });
        }
        self.result.division_places = division_places;
        let mut playoff_places: Vec<usize> = JP2024AllDivision
            .get_teams()
            .iter()
            .map(|team| team.get_index())
            .collect();
        playoff_places.sort_by(|team_index_a, team_index_b| {
            self.result.playoff_place_count[*team_index_b][0]
                .cmp(&self.result.playoff_place_count[*team_index_a][0])
        });
        self.result.playoff_places = playoff_places;
        if output_flag {
            console_log!("{:?}", self.result.division_place_count);
        } else {
            let simulated_result_str: Vec<String> = [
                format!("division_places: {:?}", self.result.division_places),
                format!("playoff_places: {:?}", self.result.playoff_places),
                format!(
                    "division_place_count: {:?}",
                    self.result.division_place_count
                ),
                format!(
                    "division_points_battles: {:?}",
                    self.result.division_points_battles
                ),
                format!("playoff_place_count: {:?}", self.result.playoff_place_count),
                format!("match_points: {:?}", self.result.match_points),
            ]
            .iter()
            .map(|str| str.replace("[", "vec!["))
            .collect();
            let joined = simulated_result_str.join(",");
            console_log!("{}", joined);
        }
    }
    fn simulate_one_time(&mut self, rng: &mut StdRng) {
        let (rate_index_function, _) =
            create_key_function_and_init_ratings(HomeAwayGameType, self.sfl_stage.get_teams());

        let mut sfl_records = self.sfl_records.to_owned();
        // レーティングに基づきランダムに結果をセット
        for records in sfl_records.iter_mut() {
            for record in records.iter_mut() {
                // 前の試行でポイントが入っているのでリセットする
                record.point = 0;
                // すでに行われた結果では is_prediction: false となっているので continue
                if !record.is_prediction {
                    // 乱数を消費して影響を減らす
                    let _: bool = if self.option.enable_rate {
                        rng.gen_bool(0.5_f64)
                    } else {
                        rng.random()
                    };
                    continue;
                }
                let (team_index, opponent_team_index) = rate_index_function(record);
                let team_rating = self.sfl_rating.ratings.get(team_index).unwrap();
                let opponent_team_rating =
                    self.sfl_rating.ratings.get(opponent_team_index).unwrap();
                if self.option.enable_rate {
                    let (team_win_percentage, _) =
                        get_win_percentage(*team_rating, *opponent_team_rating);
                    record.win_flag = rng.gen_bool(team_win_percentage);
                } else {
                    record.win_flag = rng.random();
                }
                record.is_valid = true;
            }

            // 予想分の補正処理
            self.sfl_stage.correct_records(records);
            // let sum: u32 = records.iter().map(|r| r.point).sum();
            // // ポイントのセットがうまくいっていないと1試合のポイントが45を超える
            // if sum > 45 || sum < 40 {
            //     console_log!("{:?}", sum);
            //     console_log!("{:?}", records);
            //     console_log!("{:?}", x);
            //     panic!()
            // }
        }

        for (index, sfl_match) in sfl_records.iter().enumerate() {
            let mut van_away_point = 0_u32;
            let mut van_home_point = 0_u32;
            let mut general_away_point = 0_u32;
            let mut general_home_point = 0_u32;
            for record in sfl_match {
                if !record.is_valid || record.point == 0 {
                    continue;
                }
                // is_home = false の前提でコードが書かれている…
                if record.win_flag {
                    if record.game_type.is_leader() {
                        general_away_point += record.point;
                    } else {
                        van_away_point += record.point;
                    }
                } else {
                    if record.game_type.is_leader() {
                        general_home_point += record.point;
                    } else {
                        van_home_point += record.point;
                    }
                }
            }
            self.result.match_points[index][0] += van_away_point;
            self.result.match_points[index][1] += van_home_point;
            self.result.match_points[index][2] += general_away_point;
            self.result.match_points[index][3] += general_home_point;
        }
        // 一次元vectorに変更
        let sfl_records: Vec<&SflRecord> = sfl_records.iter().flat_map(|x| x).collect();

        // この試行におけるポイント、バトル得失を集計するvector
        // チームの分だけ初期化
        let mut point_count = vec![0_u32; self.max_team_index + 1];
        let mut battle_count = vec![0_i32; self.max_team_index + 1];

        // レコードごとにポイント集計開始
        for record in sfl_records.iter() {
            // 無効ならスキップ
            if !record.is_valid {
                continue;
            }
            let team_index = record.sfl_match.team.get_index();
            let opponent_team_index = record.sfl_match.opponent_team.get_index();
            if record.win_flag {
                point_count[team_index] += record.point;
                battle_count[team_index] += 1;
                battle_count[opponent_team_index] -= 1;
            } else {
                point_count[opponent_team_index] += record.point;
                battle_count[team_index] -= 1;
                battle_count[opponent_team_index] += 1;
            }
        }
        let mut playoff_team: Vec<Vec<(SflTeam, u32, i32)>> = vec![vec![], vec![]];
        for (n, stage) in [JP2024DivisionS, JP2024DivisionF].iter().enumerate() {
            // ポイントとバトルでソートして順位を算出
            let mut sortable: Vec<(usize, u32, i32, SflTeam)> = stage
                .get_teams()
                .iter()
                .map(|team| {
                    let team_index = team.get_index();
                    let point = point_count[team_index];
                    let battle = battle_count[team_index];
                    (team_index, point, battle, team.to_owned())
                })
                .collect();
            sortable.sort_by(
                |(a_team, a_point, a_battle, ..), (b_team, b_point, b_battle, ..)| {
                    b_point
                        .cmp(&a_point)
                        .then(b_battle.cmp(&a_battle))
                        .then(b_team.cmp(a_team))
                },
            );
            // 順位のカウントアップとポイント・バトルの合計更新
            for nth in 0..6 {
                let (team_index, point, battle, team) = sortable.get(nth).unwrap().to_owned();
                self.result.division_place_count[team_index][nth] += 1;
                self.result.division_points_battles[team_index][0] += point as i32;
                self.result.division_points_battles[team_index][1] += battle;
                if nth < 3 {
                    playoff_team[n].push((team, point, battle));
                }
            }
        }
        for n in 0..2_usize {
            for m in 0..2_usize {
                let team_info = &playoff_team[n][2 - m];
                let opponent_team_info = &playoff_team[n][1 - m];
                let records =
                    &mut JP2024Playoff.get_playoff_records(&team_info.0, &opponent_team_info.0);
                for record in records.iter_mut() {
                    let (team_index, opponent_team_index) = rate_index_function(record);
                    let team_rating = self.sfl_rating.ratings.get(team_index).unwrap();
                    let opponent_team_rating =
                        self.sfl_rating.ratings.get(opponent_team_index).unwrap();
                    if self.option.enable_rate {
                        let (team_win_percentage, _) =
                            get_win_percentage(*team_rating, *opponent_team_rating);
                        record.win_flag = rng.gen_bool(team_win_percentage);
                    } else {
                        record.win_flag = rng.random();
                    }
                    record.is_valid = true;
                }

                // 予想分の簡易得点処理
                let (win_team, ..) = JP2024Playoff.get_win_team(records);
                if m == 0 {
                    if team_info.0 == win_team {
                        // プレイオフ5位
                        self.result.playoff_place_count[opponent_team_info.0.get_index()][3] += 1;
                        // place_sim_counts[opponent_team_info.0.get_index()][11] += 1;
                        playoff_team[n] = vec![playoff_team[n][0].to_owned(), team_info.to_owned()];
                    } else {
                        // プレイオフ5位
                        self.result.playoff_place_count[team_info.0.get_index()][3] += 1;
                        // place_sim_counts[team_info.0.get_index()][11] += 1;
                        playoff_team[n] =
                            vec![playoff_team[n][0].to_owned(), opponent_team_info.to_owned()];
                    }
                } else {
                    if team_info.0 == win_team {
                        // プレイオフ3位
                        self.result.playoff_place_count[opponent_team_info.0.get_index()][2] += 1;
                        // place_sim_counts[opponent_team_info.0.get_index()][10] += 1;
                        playoff_team[n] = vec![team_info.to_owned()];
                    } else {
                        // プレイオフ3位
                        self.result.playoff_place_count[team_info.0.get_index()][2] += 1;
                        // place_sim_counts[team_info.0.get_index()][10] += 1;
                        playoff_team[n] = vec![opponent_team_info.to_owned()];
                    }
                }
            }
        }

        // HOME / AWAY の決定
        // ポイント > バトル得失
        let mut gf_team_info = &playoff_team[0][0];
        let mut gf_opponent_team_info = &playoff_team[1][0];

        if gf_team_info.1 == gf_opponent_team_info.1 {
            if gf_team_info.2 > gf_opponent_team_info.2 {
                gf_team_info = &playoff_team[1][0];
                gf_opponent_team_info = &playoff_team[0][0];
            }
        } else if gf_team_info.1 > gf_opponent_team_info.1 {
            gf_team_info = &playoff_team[1][0];
            gf_opponent_team_info = &playoff_team[0][0];
        }
        // グランドファイナル処理ここから
        let records = &mut JP2024GrandFinal
            .get_grand_final_records(&gf_team_info.0, &gf_opponent_team_info.0);
        for record in records.iter_mut() {
            let (team_index, opponent_team_index) = rate_index_function(record);
            let team_rating = self.sfl_rating.ratings.get(team_index).unwrap();
            let opponent_team_rating = self.sfl_rating.ratings.get(opponent_team_index).unwrap();
            if self.option.enable_rate {
                let (team_win_percentage, _) =
                    get_win_percentage(*team_rating, *opponent_team_rating);
                record.win_flag = rng.gen_bool(team_win_percentage);
            } else {
                record.win_flag = rng.random();
            }
            record.is_valid = true;
        }
        let (win_team, ..) = JP2024GrandFinal.get_win_team(records);
        // 優勝
        self.result.playoff_place_count[win_team.get_index()][0] += 1;
        if playoff_team[0][0].0 == win_team {
            // 準優勝
            self.result.playoff_place_count[playoff_team[1][0].0.get_index()][1] += 1;
        } else {
            // 準優勝
            self.result.playoff_place_count[playoff_team[0][0].0.get_index()][1] += 1;
        }
    }
}

#[wasm_bindgen]
#[derive(Copy, Clone)]
pub struct SimulationOption {
    pub enable_rate: bool,
}
