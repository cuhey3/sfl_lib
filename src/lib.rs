mod sfl;
mod utils;
use crate::sfl::SflStage::{
    JP2024AllDivision, JP2024DivisionF, JP2024DivisionS, JP2024GrandFinal, JP2024Playoff,
};
use crate::sfl::{
    create_key_function_and_init_ratings, get_win_percentage, update_rating, SflRatingSetting,
    SflRecord, SflStage, SflTeam,
};
use js_sys;
use rand::prelude::*;
use std::ops::Deref;
use wasm_bindgen::prelude::*;
use web_sys;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

#[wasm_bindgen]
pub fn get_division_matches() -> JsValue {
    let result = js_sys::Array::new();
    for division in [JP2024DivisionS, JP2024DivisionF].iter() {
        for sfl_match in division.get_matches().iter() {
            let division_expression = match division {
                JP2024DivisionS => "S",
                JP2024DivisionF => "F",
                _ => "N",
            };
            let match_array = js_sys::Array::new();
            match_array.push(&JsValue::from(sfl_match.date_expression.to_owned()));
            match_array.push(&JsValue::from(sfl_match.section.to_owned()));
            match_array.push(&JsValue::from(sfl_match.branch.to_owned()));
            match_array.push(&JsValue::from(division_expression));
            match_array.push(&JsValue::from(&sfl_match.team.to_string()));
            match_array.push(&JsValue::from(&sfl_match.opponent_team.to_string()));
            match_array.push(&JsValue::from(sfl_match.is_home));
            result.push(&match_array);
        }
    }
    result.sort();
    JsValue::from(&result)
}

#[wasm_bindgen]
pub fn greet(division_raw_results: js_sys::Array, enable_rate: bool) -> JsValue {
    let mut division_results: Vec<Vec<bool>> = vec![];
    let mut results = js_sys::Array::new();
    for parent_array in division_raw_results.iter() {
        let match_result: Vec<bool> = js_sys::Array::from(&parent_array)
            .iter()
            .map(|v| v.as_bool().unwrap())
            .collect();
        division_results.push(match_result);
    }
    console_log!("{:?}", division_results.to_owned());
    let simulate_results =
        get_simulate_result(JP2024AllDivision, enable_rate, division_results.to_owned());
    for division in [JP2024DivisionS, JP2024DivisionF].iter() {
        let mut array = js_sys::Array::new();
        for team in division.get_teams().iter() {
            let team_index = team.get_index();
            let (counts, points) = simulate_results[team_index].to_owned();
            let mut row = js_sys::Array::new();
            let value = JsValue::from(team.to_string());
            row.push(&value);
            for c in counts.iter().enumerate() {
                // TODO 後から追加したフィールドを読みやすくする
                if c.0 < 8 {
                    row.push(&JsValue::from(*c.1));
                }
            }
            let (
                battle_actual,
                battle_prediction,
                van_mid_away,
                van_mid_home,
                general_extra_away,
                general_extra_home,
            ) = points;
            row.push(&JsValue::from(battle_actual));
            row.push(&JsValue::from(battle_prediction));
            row.push(&JsValue::from(van_mid_away));
            row.push(&JsValue::from(van_mid_home));
            row.push(&JsValue::from(general_extra_away));
            row.push(&JsValue::from(general_extra_home));
            // TODO 後から追加したフィールドを読みやすくする
            row.push(&JsValue::from(counts[8]));
            row.push(&JsValue::from(counts[9]));
            row.push(&JsValue::from(counts[10]));
            row.push(&JsValue::from(counts[11]));
            array.push(&JsValue::from(row));
        }
        results.push(&JsValue::from(array));
    }
    JsValue::from(results)
}

fn get_simulate_result(
    sfl_stage: SflStage,
    enable_rate: bool,
    played_match_results: Vec<Vec<bool>>,
) -> Vec<(Vec<u32>, (i32, i32, f64, f64, f64, f64))> {
    let seed: [u8; 32] = [5; 32];
    let mut rng: StdRng = rand::SeedableRng::from_seed(seed);
    let (rate_index_function, mut ratings) = create_key_function_and_init_ratings(
        SflRatingSetting::HomeAwayGameType,
        sfl_stage.get_teams(),
    );
    // ステージに応じた初期状態のレコードを生成
    let mut initial_record_matches: Vec<Vec<SflRecord>> = sfl_stage.get_initial_records();

    // すでに行われた結果を初期状態のレコードに記入
    for (index, played_match_result) in played_match_results.into_iter().enumerate() {
        // すでに行われたマッチ結果のインデックスの方がマッチ予定より大きい時はbreak
        let initial_records = initial_record_matches.get_mut(index);
        if initial_records.is_none() {
            break;
        }
        let initial_records = initial_records.unwrap();
        for (index, win_flag) in played_match_result.into_iter().enumerate() {
            // すでに行われたバトル結果のインデックスの方が初期レコードのサイズより大きい時はpanic
            let initial_record = initial_records.get_mut(index);
            if initial_record.is_none() {
                break;
            }
            let initial_record = initial_record.unwrap();
            initial_record.win_flag = win_flag; // 初期状態は false
            initial_record.is_valid = true; // 初期状態は false
            initial_record.is_prediction = false; // 初期状態は true
        }
    }

    // すでに行われた分の補正を行い、その後にレーティングに反映する
    for records in initial_record_matches.iter_mut() {
        // 第1セットから予想の場合は補正対象外
        if records.get(0).unwrap().is_prediction {
            continue;
        }
        // 補正実行
        // すでに行われた分を補正して、実際には行われなかったセットに is_valid = false を立てる
        // 決着局にポイントを付与する
        sfl_stage.correct_records(records);

        // レーティング計算開始
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

    // 順位の集計map
    let max_team_index = sfl_stage
        .get_teams()
        .iter()
        .map(|team| team.get_index())
        .max()
        .unwrap();
    let mut place_sim_counts = vec![vec![0_u32; 12]; max_team_index + 1];
    let mut place_sim_battles = vec![vec![0_i32; 2]; max_team_index + 1];

    // チームごとに現在ポイントと現在バトル得失を集計
    for team in sfl_stage.get_teams() {
        let team_index = team.get_index();
        // チームが含まれる有効なレコードのみ抽出
        let records: Vec<&SflRecord> = initial_record_matches
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
        place_sim_counts[team_index][6] = point;
        place_sim_battles[team_index][0] = battle;
    }

    let max_team_index = sfl_stage
        .get_teams()
        .iter()
        .map(|team| team.get_index())
        .max()
        .unwrap();

    // 10000回試行して小数点第一位まで表示
    for x in 0..10000 {
        // レーティングに基づきランダムに結果をセット
        for records in initial_record_matches.iter_mut() {
            for record in records.iter_mut() {
                // 前の試行でポイントが入っているのでリセットする
                record.point = 0;
                // すでに行われた結果では is_prediction: false となっているので continue
                if !record.is_prediction {
                    continue;
                }
                let (team_index, opponent_team_index) = rate_index_function(record);
                let team_rating = ratings.get(team_index).unwrap();
                let opponent_team_rating = ratings.get(opponent_team_index).unwrap();
                if enable_rate {
                    let (team_win_percentage, _) =
                        get_win_percentage(*team_rating, *opponent_team_rating);
                    record.win_flag = rng.gen_bool(team_win_percentage);
                } else {
                    record.win_flag = rng.random();
                }
                record.is_valid = true;
            }

            // 予想分の補正処理
            sfl_stage.correct_records(records);
            // let sum: u32 = records.iter().map(|r| r.point).sum();
            // // ポイントのセットがうまくいっていないと1試合のポイントが45を超える
            // if sum > 45 || sum < 40 {
            //     console_log!("{:?}", sum);
            //     console_log!("{:?}", records);
            //     console_log!("{:?}", x);
            //     panic!()
            // }
        }

        // 一次元vectorに変更
        let sfl_records: Vec<&SflRecord> = initial_record_matches.iter().flat_map(|x| x).collect();

        // この試行におけるポイント、バトル得失を集計するvector
        // チームの分だけ初期化
        let mut point_count = vec![0_u32; max_team_index + 1];
        let mut battle_count = vec![0_i32; max_team_index + 1];

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
                place_sim_counts[team_index][nth] += 1;
                place_sim_counts[team_index][7] += point;
                place_sim_battles[team_index][1] += battle;
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
                    let team_rating = ratings.get(team_index).unwrap();
                    let opponent_team_rating = ratings.get(opponent_team_index).unwrap();
                    if enable_rate {
                        let (team_win_percentage, _) =
                            get_win_percentage(*team_rating, *opponent_team_rating);
                        record.win_flag = rng.gen_bool(team_win_percentage);
                    } else {
                        record.win_flag = rng.random();
                    }
                    record.is_valid = true;
                }

                // 予想分の簡易得点処理
                let (win_team, point, opponent_point) = JP2024Playoff.get_win_team(records);
                if m == 0 {
                    if team_info.0 == win_team {
                        // プレイオフ5位
                        place_sim_counts[opponent_team_info.0.get_index()][11] += 1;
                        playoff_team[n] = vec![playoff_team[n][0].to_owned(), team_info.to_owned()];
                    } else {
                        // プレイオフ5位
                        place_sim_counts[team_info.0.get_index()][11] += 1;
                        playoff_team[n] =
                            vec![playoff_team[n][0].to_owned(), opponent_team_info.to_owned()];
                    }
                } else {
                    if team_info.0 == win_team {
                        // プレイオフ3位
                        place_sim_counts[opponent_team_info.0.get_index()][10] += 1;
                        playoff_team[n] = vec![team_info.to_owned()];
                    } else {
                        // プレイオフ3位
                        place_sim_counts[team_info.0.get_index()][10] += 1;
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
            let team_rating = ratings.get(team_index).unwrap();
            let opponent_team_rating = ratings.get(opponent_team_index).unwrap();
            if enable_rate {
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
        place_sim_counts[win_team.get_index()][8] += 1;
        if playoff_team[0][0].0 == win_team {
            // 準優勝
            place_sim_counts[playoff_team[1][0].0.get_index()][9] += 1;
        } else {
            // 準優勝
            place_sim_counts[playoff_team[0][0].0.get_index()][9] += 1;
        }
    }
    let mut results: Vec<(Vec<u32>, (i32, i32, f64, f64, f64, f64))> =
        vec![(vec![], (0, 0, 0.0, 0.0, 0.0, 0.0)); max_team_index + 1];
    for team in sfl_stage.get_teams() {
        let team_index = team.get_index();
        let team_count = &place_sim_counts[team_index];
        let team_battle = &place_sim_battles[team_index];
        let vec: Vec<f64> = [0, 1, 2, 3]
            .iter()
            .map(|n| ratings.get(team_index * 4 + n).unwrap().to_owned())
            .collect();
        results[team.get_index()] = (
            team_count.to_owned(),
            (
                team_battle[0],
                team_battle[1],
                vec[0],
                vec[1],
                vec[2],
                vec[3],
            ),
        );
    }
    console_log!("{:?}", results);
    results
}
