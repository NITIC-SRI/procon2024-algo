use crate::board::action::{Action, Direction};
use crate::board::board::Board;
use crate::board::cut::Cuts;
use std::hash::{Hash, Hasher};

use rand::rngs::StdRng;
use rand::seq::SliceRandom;
use rand::SeedableRng;
use rand::{self, Rng};
use serde::{Deserialize, Serialize};

use ahash::AHashSet as HashSet;
use std::collections::hash_map::DefaultHasher;

#[derive(Serialize, Deserialize)]
struct ActionFormat {
    p: u16,
    x: i32,
    y: i32,
    s: String,
}

#[derive(Serialize, Deserialize)]
struct ActionsFormat {
    n: usize,
    ops: Vec<ActionFormat>,
}

pub fn export_actions(actions: Vec<Action>) -> String {
    let mut actions_format = ActionsFormat {
        n: actions.len(),
        ops: Vec::new(),
    };
    for action in actions {
        let action_format = ActionFormat {
            p: action.cut_num(),
            x: action.x(),
            y: action.y(),
            s: match action.direction() {
                Direction::Up => "0".to_string(),
                Direction::Down => "1".to_string(),
                Direction::Right => "2".to_string(),
                Direction::Left => "3".to_string(),
            },
        };
        actions_format.ops.push(action_format);
    }
    let json = serde_json::to_string(&actions_format).unwrap();
    json
}

#[derive(Serialize, Deserialize)]
struct Visualizer {
    start: Vec<Vec<u8>>,
    end: Vec<Vec<u8>>,
    actions: Vec<ActionFormat>,
}

pub fn export_visualyzer_json(start: &Board, end: &Board, actions: Vec<Action>) -> String {
    let mut actions_format = Vec::new();
    for action in actions {
        let action_format = ActionFormat {
            p: action.cut_num(),
            x: action.x(),
            y: action.y(),
            s: match action.direction() {
                Direction::Up => "0".to_string(),
                Direction::Down => "1".to_string(),
                Direction::Right => "2".to_string(),
                Direction::Left => "3".to_string(),
            },
        };
        actions_format.push(action_format);
    }
    let visualizer = Visualizer {
        start: start.clone().board,
        end: end.clone().board,
        actions: actions_format,
    };
    let json = serde_json::to_string(&visualizer).unwrap();
    json
}

pub fn random_board(h: u32, w: u32) -> Vec<Vec<u8>> {
    let mut rng = rand::thread_rng();
    let mut board = vec![vec![0; w as usize]; h as usize];
    for i in 0..h {
        for j in 0..w {
            board[i as usize][j as usize] = rng.gen_range(0..4);
        }
    }
    board
}

pub fn shuffle_board(mut board: Vec<Vec<u8>>, seed: u64) -> Vec<Vec<u8>> {
    let mut rng = StdRng::seed_from_u64(seed);

    // ボードを1次元ベクトルに変換
    let mut flat_board: Vec<u8> = board.iter().flat_map(|row| row.iter()).cloned().collect();

    // シャッフル
    flat_board.as_mut_slice().shuffle(&mut rng);

    // シャッフル後の1次元ベクトルを2次元ベクトルに戻す
    let h = board.len();
    let w = board[0].len();
    for i in 0..h {
        for j in 0..w {
            board[i][j] = flat_board[i * w + j];
        }
    }

    board
}

pub fn get_actions(h_size: usize, w_size: usize, cuts: &Cuts) -> Vec<Action> {
    let mut actions = Vec::with_capacity(10000); // 容量を事前に確保
    let mut saw = HashSet::with_capacity(10000); // HashSetの容量も事前確保

    let mut board_vec: Vec<Vec<usize>> = vec![vec![0; w_size]; h_size];
    for h in 0..h_size {
        for w in 0..w_size {
            board_vec[h][w] = w + h * w_size;
        }
    }
    let board: Board<usize> = Board::new(board_vec);

    for i in 0..cuts.len() {
        if i >= 3 && i < 25 {
            let cut = &cuts[i as u32 - 3];
            if cut.width() >= w_size || cut.height() >= h_size {
                break;
            }
        }

        let cut = &cuts[i as u32];
        let cut_w = cut.width() as i32;
        let cut_h = cut.height() as i32;

        for w in (1 - cut_w)..(w_size as i32) {
            for h in (1 - cut_h)..(h_size as i32) {
                for d in vec![
                    Direction::Left,
                    Direction::Right,
                    Direction::Up,
                    Direction::Down,
                ] {
                    let action = Action::new(w, h, i as u16, d);
                    let mut new_board = board.clone();

                    new_board.operate(&action, cuts);
                    let new_board_hash = calculate_hash(&new_board);
                    if new_board == board || saw.contains(&new_board_hash) {
                        continue;
                    }

                    saw.insert(new_board_hash);
                    actions.push(action);
                }
            }
        }
    }
    actions
}

pub fn read_actions(path: String) -> Vec<Action> {
    let file = std::fs::File::open(path).unwrap();
    let reader = std::io::BufReader::new(file);
    let actions_format: ActionsFormat = serde_json::from_reader(reader).unwrap();
    let mut actions = Vec::new();
    for action_format in actions_format.ops {
        let action = Action::new(
            action_format.x,
            action_format.y,
            action_format.p,
            match action_format.s.as_str() {
                "0" => Direction::Up,
                "1" => Direction::Down,
                "2" => Direction::Right,
                "3" => Direction::Left,
                _ => panic!("Invalid direction"),
            },
        );
        actions.push(action);
    }
    actions
}

pub fn random_general_cut(h: u32, w: u32) -> Vec<String> {
    // 0がw個並んだものをh個並べる
    let mut general_cut = Vec::new();
    let mut rng = rand::thread_rng();

    for _ in 0..h {
        let mut row = Vec::new();
        for _ in 0..w {
            row.push(rng.gen_range(0..2).to_string());
        }
        general_cut.push(row.join(""));
    }

    general_cut
}

fn calculate_hash<T: Hash>(t: &T) -> u64 {
    let mut hasher = DefaultHasher::new();
    t.hash(&mut hasher);
    hasher.finish()
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Data {
    pub board: TestBoard,
    pub general: TestGeneral,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct TestBoard {
    pub width: u32,
    pub height: u32,
    pub start: Vec<String>,
    pub goal: Vec<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct TestGeneral {
    pub n: u32,
    pub pattern: Vec<Pattern>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Pattern {
    pub p: u32,
    pub width: u32,
    pub height: u32,
    pub cells: Vec<String>,
}

pub fn get_action_by_direction(legal_actions: &Vec<Action>) -> (Vec<Action>, Vec<Action>) {
    let mut down_only = Vec::new();
    let mut x_only = Vec::new();
    for action in legal_actions {
        if action.y() < 1 {
            continue;
        }
        if action.direction() == Direction::Down {
            down_only.push(action.clone());
        } else if action.direction() == Direction::Left || action.direction() == Direction::Right {
            x_only.push(action.clone());
        }
    }
    (down_only, x_only)
}

pub fn validate_actions(start: &Board, end: &Board, actions: &Vec<Action>, cuts: &Cuts) -> bool {
    let mut now = start.clone();
    for action in actions.iter() {
        now.operate(action, cuts)
    }

    now == *end
}
