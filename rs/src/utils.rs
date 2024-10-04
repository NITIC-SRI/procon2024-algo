use crate::board::action::{Action, Direction};
use serde::{Deserialize, Serialize};
use rand::{self, Rng};

use rand::seq::SliceRandom;
use rand::SeedableRng;
use rand::rngs::StdRng;

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


pub fn random_board(h: u32, w: u32) -> Vec<Vec<u8>> {
    let mut rng = rand::thread_rng();
    let mut board = vec![vec![0; w as usize]; h as usize];
    for i in 0..h {
        for j in 0..w {
            board[i as usize][j as usize] = rng.gen_range(0..5);
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
