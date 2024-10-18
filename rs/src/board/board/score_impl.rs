use super::Board;

use crate::board::action::Action;
use crate::board::cut::Cuts;
use std::convert::Into;
use std::fmt::Debug;

impl<T> Board<T>
where
    T: Copy + PartialEq + Into<usize> + Debug,
{
    pub fn absolute_distance(&self, end: &Self) -> u64 {
        let mut d = 0;
        for h in 0..self.height {
            for w in 0..self.width {
                if self.board[h][w] != end.board[h][w] {
                    d += 1;
                }
            }
        }

        d
    }

    pub fn weighted_absolute_distance(&self, end: &Self, turn: usize, val: i32, span: i32) -> u64 {
        let mut score = 0;
        for h in 0..self.height() {
            for w in 0..self.width() {
                // 中心からのユークリッド距離
                let mut distance = (h as i32 - self.height() as i32 / 2).pow(2)
                    + (w as i32 - self.width() as i32 / 2).pow(2);
                distance = (distance as f64).sqrt() as i32;
                if distance * span <= turn as i32 {
                    if self.board()[h][w] == end.board()[h][w] {
                        score += ((val - distance)
                            * (val - distance)
                            * (val - distance)
                            * (val - distance)) as u64;
                    }
                } else {
                    if self.board()[h][w] != end.board()[h][w] {
                        score += 1;
                    }
                }
            }
        }

        score
    }

    // 行ごとに、列外れていても同じ数字があればスコアを加算
    pub fn row_score(&self, end: &Self) -> u64 {
        let mut score = 0;
        for h in 0..self.height() {
            let mut row_map = vec![0; 4];
            let mut row_score = 0;
            for w in 0..self.width() {
                row_map[self.board()[h][w].into()] += 1;
                row_map[end.board()[h][w].into()] -= 1;
            }
            for i in 0..4 {
                row_score += (row_map[i] as i32).abs();
            }
            score += row_score as u64;
        }

        score
    }

    pub fn col_score(&self, end: &Self) -> u64 {
        let mut score = 0;
        for w in 0..self.width() {
            let mut col_map = vec![0; 4];
            let mut col_score = 0;
            for h in 0..self.height() {
                col_map[self.board()[h][w].into()] += 1;
                col_map[end.board()[h][w].into()] -= 1;
            }
            for i in 0..4 {
                col_score += (col_map[i] as i32).abs();
            }
            score += col_score as u64;
        }

        score
    }

    pub fn migishita_score(&self, end: &Self) -> u64 {
        // 右下から優先して揃える
        let mut score = 0;
        let mut zero = 1.0;
        for h in 0..self.height() {
            for w in 0..self.width() {
                if h < self.height() - 3 && w < self.width() - 3 {
                    continue;
                }
                if self.board()[h][w] == end.board()[h][w] {
                    score += (1 * w * h) as u64;
                }
                if self.board()[h][w] != end.board()[h][w] {
                    zero = 0.5;
                }
            }
        }

        (score as f64 * zero) as u64
    }

    pub fn before_board_and_weighted_absolute_distance(
        &self,
        end: &Self,
        turn: usize,
        before_board: &Self,
        val: i32,
    ) -> u64 {
        let mut score = 0;
        for h in 0..self.height() {
            for w in 0..self.width() {
                // 中心からのユークリッド距離
                let mut distance = (h as i32 - self.height() as i32 / 2).pow(2)
                    + (w as i32 - self.width() as i32 / 2).pow(2);
                distance = (distance as f64).sqrt() as i32;
                if turn > 20 {
                    if self.board()[h][w] == end.board()[h][w] {
                        score += 1;
                    }
                    continue;
                }
                if self.board()[h][w] == end.board()[h][w] {
                    score += ((val - distance) * (val - distance)) as u64;
                } else if self.board()[h][w] == end.board()[h][w]
                    && before_board.board()[h][w] != end.board()[h][w]
                {
                    score += ((val - distance) * (val - distance)) as u64;
                }
            }
        }

        score
    }

    // diffは揃っていない箇所のインデックス
    pub fn top_first_distance(&self, end: &Self, usable_height: usize) -> (u64, Vec<usize>) {
        let mut distance = 0;
        // 揃っていないところを保存
        let mut diff = vec![];
        for w in 0..self.width() {
            if self.board()[0][w] != end.board()[self.height() - usable_height][w] {
                distance += 1;
                diff.push(w);
            }
        }
        (distance, diff)
    }

    pub fn match_x_direction_score(
        self,
        end: &Self,
        diff: &Vec<usize>,
        usable_height: usize,
    ) -> u64 {
        let mut score = 0;
        for &d in diff {
            for h in 1..usable_height {
                if self.board[h][d] == end.board[self.height() - usable_height][d] {
                    score += 1;
                    break;
                }
            }
        }
        (diff.len() - score) as u64
    }

    pub fn no_op_top_distance(
        &self,
        end: &Self,
        end_row_num: usize,
        prev_distance: u64,
        cuts: &Cuts,
        action: Action,
    ) -> u64 {
        let mut distance_diff: i32 = 0;
        let mut diff = vec![];

        let cut = &cuts[action.cut_num() as u32];
        if cut.width() as i32 + action.x() < 0 {
            return 0;
        }

        for w in 0..self.width() {
            if w as i32 > (cut.width() as i32) + action.x() {
                break;
            }
            let cut_flag = if w < action.x() as usize || w >= action.x() as usize + cut.width() {
                false
            } else {
                cut[0][w - action.x() as usize]
            };
            let now_top_cell = self.board()[0][w];
            let now_midlle_cell = self.board()[action.y() as usize][w];
            let end_cell = end.board()[end_row_num][w];

            if cut_flag {
                if now_top_cell != end_cell && now_midlle_cell == end_cell {
                    distance_diff -= 1;
                } else if now_top_cell == end_cell && now_midlle_cell != end_cell {
                    distance_diff += 1;
                    diff.push(w);
                } else if now_top_cell != end_cell && now_midlle_cell != end_cell {
                    diff.push(w);
                }
            } else {
                if now_top_cell != end_cell {
                    diff.push(w);
                }
            }
        }

        ((prev_distance as i32) + distance_diff) as u64
    }
}
