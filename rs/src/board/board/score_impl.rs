use super::Board;

use crate::board::action::Action;
use crate::board::cut::Cut;
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

    #[allow(unused_variables)]
    pub fn operate_and_absolute_distance(
        &mut self,
        cut: &Cut,
        x: i32,
        y: i32,
        direction: u8,
        end: &Self,
    ) -> (Action, u64) {
        unimplemented!()
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

    pub fn top_first_distance(&self, end: &Self, already_height: usize) -> (u64, Vec<usize>) {
        let mut distance = 0;
        // 揃っていないところを保存
        let mut diff = vec![];
        for w in 0..self.width() {
            if self.board()[0][w] != end.board()[self.height()-already_height][w] {
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
        already_height: usize,
    ) -> u64 {
        let mut distance = 0;
        for &d in diff {
            for h in 1..already_height {
                if self.board[h][d] != end.board[self.height()-already_height][d] {
                    distance += 1;
                }
            }
        }
        distance
    }
}
