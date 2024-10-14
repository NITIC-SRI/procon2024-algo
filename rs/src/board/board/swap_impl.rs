use super::Board;

use crate::board::action;
use crate::board::action::Action;
use std::convert::Into;
use std::fmt::Debug;
use std::vec;

impl<T> Board<T>
where
    T: Copy + PartialEq + Into<usize> + Debug,
{
    fn swapping_one_line(self, x1: i32, y1: i32, x2: i32, y2: i32) -> Vec<Action> {
        let mut actions = vec![];
        if x1 == x2 {
            // 1列での入れ替え
            let x = x1;
            let (min, max) = if y1 < y2 { (y1, y2) } else { (y2, y1) };
            let (dir_min, dir_max) = (action::Direction::Down, action::Direction::Up);

            // 逆端に寄せる
            actions.push(Action::new(x, min, 0, dir_max));
            actions.push(Action::new(x, max - 1, 0, dir_min));

            // ほかのセルを戻す
            for _ in 0..min {
                actions.push(Action::new(x, min, 0, dir_min));
            }
            for _ in 0..(self.height as i32 - max - 1) {
                actions.push(Action::new(x, max, 0, dir_max));
            }
        } else if y1 == y2 {
            // 1行での入れ替え
            let y = y1;
            let (min, max) = if x1 < x2 { (x1, x2) } else { (x2, x1) };
            let (dir_min, dir_max) = (action::Direction::Right, action::Direction::Left);

            // 逆端に寄せる
            actions.push(Action::new(min, y, 0, dir_max));
            actions.push(Action::new(max - 1, y, 0, dir_min));

            // ほかのセルを戻す
            for _ in 0..min {
                actions.push(Action::new(min, y, 0, dir_min));
            }
            for _ in 0..(self.width as i32 - max - 1) {
                actions.push(Action::new(max, y, 0, dir_max));
            }
        } else {
            unreachable!("x1, y1, x2, y2 must be on the same line");
        }

        actions
    }

    fn swapping_others(
        self,
        x1: i32,
        y1: i32,
        x2: i32,
        y2: i32,
        dir: action::Direction,
    ) -> Vec<Action> {
        let mut actions = vec![];

        // 1. セルを端に寄せる
        actions.push(Action::new(x1, y1, 0, dir));
        actions.push(Action::new(x2, y2, 0, dir));

        // 2. 並べ替えて、交換する
        let (min_point, max_point, edge_pos, line_length, next_dir) = match dir {
            action::Direction::Up | action::Direction::Down => {
                let (min_point, max_point) = if x1 < x2 { (x1, x2) } else { (x2, x1) };
                let edge_pos_y = if dir == action::Direction::Up {
                    self.height as i32 - 1
                } else {
                    0
                };
                (
                    min_point,
                    max_point,
                    edge_pos_y,
                    self.width as i32,
                    action::Direction::Right,
                )
            }
            action::Direction::Left | action::Direction::Right => {
                let (min_point, max_point) = if y1 < y2 { (y1, y2) } else { (y2, y1) };
                let edge_pos_x = if dir == action::Direction::Left {
                    self.width as i32 - 1
                } else {
                    0
                };
                (
                    min_point,
                    max_point,
                    edge_pos_x,
                    self.height as i32,
                    action::Direction::Down,
                )
            }
        };
        process_direction(
            &mut actions,
            min_point,
            max_point,
            edge_pos,
            line_length,
            next_dir,
        );
        {
            // 3. セルを元の場所に戻す
            let (count1, count2) = match dir {
                action::Direction::Down => (y1, y2),
                action::Direction::Up => (self.height as i32 - y1 - 1, self.height as i32 - y2 - 1),
                action::Direction::Right => (x1, x2),
                action::Direction::Left => (self.width as i32 - x1 - 1, self.width as i32 - x2 - 1),
            };
            for _ in 0..count1 {
                actions.push(Action::new(x1, y1, 0, dir))
            }
            for _ in 0..count2 {
                actions.push(Action::new(x2, y2, 0, dir))
            }
        }
        actions
    }

    fn swap(&mut self, x1: i32, y1: i32, x2: i32, y2: i32) {
        let tmp = self.board[y1 as usize][x1 as usize];
        self.board[y1 as usize][x1 as usize] = self.board[y2 as usize][x2 as usize];
        self.board[y2 as usize][x2 as usize] = tmp;
    }

    pub fn swapping(&mut self, x1: i32, y1: i32, x2: i32, y2: i32) -> Vec<Action> {
        let mut actions = vec![];

        let scores = [
            (
                action::Direction::Left,
                (2 * self.width as i32 - x1 - x2) + self.height() as i32,
            ),
            (action::Direction::Right, x1 + x2 + self.height() as i32),
            (
                action::Direction::Up,
                2 * self.height as i32 - y1 - y2 + self.width() as i32,
            ),
            (action::Direction::Down, y1 + y2 + self.width() as i32),
        ];

        let new = self.clone();

        if x1 == x2 {
            let score_one_line = y1 + (self.height as i32 - y2);
            // １ラインスコアが小さいなら1行での入れ替えを優先

            let (dir, score) = {
                // スコアが最小の方向を選択できるらしい
                let res = scores[..2].iter().min_by_key(|&&(_, score)| score).unwrap();
                (res.0, res.1)
            };

            if score_one_line < score {
                actions.extend(new.swapping_one_line(x1, y1, x2, y2));
            } else {
                actions.extend(new.swapping_others(x1, y1, x2, y2, dir))
            }
        } else if y1 == y2 {
            let score_one_line = x1 + (self.width() as i32 - x2);

            let (dir, score) = {
                // スコアが最小の方向を選択できるらしい
                let res = scores[2..].iter().min_by_key(|&&(_, score)| score).unwrap();
                (res.0, res.1)
            };

            // １ラインスコアが小さいなら1列での入れ替えを優先
            if score_one_line < score {
                actions.extend(new.swapping_one_line(x1, y1, x2, y2));
            } else {
                actions.extend(new.swapping_others(x1, y1, x2, y2, dir))
            }
        } else {
            // 行と列が異なる場合
            let dir = scores.iter().min_by_key(|&&(_, score)| score).unwrap().0;
            actions.extend(new.swapping_others(x1, y1, x2, y2, dir));
        }

        self.swap(x1, y1, x2, y2);
        actions
    }

    pub fn solve_swapping(&mut self, end: &Self) -> Vec<Action> {
        let mut actions = vec![];
        let mut pairs: Vec<Vec<Vec<(i32, i32)>>> = vec![
            vec![vec![], vec![], vec![], vec![]],
            vec![vec![], vec![], vec![], vec![]],
            vec![vec![], vec![], vec![], vec![]],
            vec![vec![], vec![], vec![], vec![]],
        ];

        // 終盤面と現盤面を比べ異なるセルをペアにして保存
        // pairs[i][j] = (x, y):iが欲しいところにjがあり、座標は(x, y)
        for y in 0..self.height() {
            for x in 0..self.width() {
                if self.board[y][x] != end.board[y][x] {
                    let source = self.board[y][x];
                    let target = end.board[y][x];
                    pairs[target.into()][source.into()].push((x as i32, y as i32));
                }
            }
        }

        // 最適ペアでの交換
        for i in 0..4 {
            for j in 0..4 {
                if i == j {
                    continue;
                }
                while pairs[i][j].len() > 0 && pairs[j][i].len() > 0 {
                    let (x1, y1) = pairs[i][j].pop().unwrap();
                    let (x2, y2) = pairs[j][i].pop().unwrap();
                    actions.extend(self.swapping(x1, y1, x2, y2));
                }
            }
        }

        // 残りの交換
        for i in 0..4 {
            let mut index = 0;

            for j in 0..4 {
                if i == j {
                    continue;
                }
                while pairs[i][j].len() > 0 {
                    while pairs[index][i].len() > 0 {
                        let (x1, y1) = pairs[i][j].pop().unwrap();
                        let (x2, y2) = pairs[index][i].pop().unwrap();

                        actions.extend(self.swapping(x1, y1, x2, y2));
                        pairs[index][j].push((x2, y2));
                        break;
                    }

                    if pairs[index][i].len() == 0 {
                        index += 1;
                    }
                }
            }
        }

        actions
    }
}

/// 端の寄せた後の一列をそろえる処理
fn process_direction(
    actions: &mut Vec<Action>,
    min_point: i32,
    max_point: i32,
    edge_pos: i32,
    line_lenght: i32,
    direction: action::Direction,
) {
    // ToDo: edge_posは端寄せしたあとの位置

    // 0側に寄席せたならでかい型を使うときにずらす必要がある
    // 逆側に寄せたなら必要ない
    let sign = if edge_pos == 0 { 1 } else { 0 };

    actions.push(Action::new_from_axis_point(
        max_point + 1,
        edge_pos + (sign * -255),
        22,
        direction,
    ));
    actions.push(Action::new_from_axis_point(
        min_point + (line_lenght - max_point - 1),
        edge_pos,
        0,
        direction,
    ));

    let diff = max_point - min_point - 1;
    let diff_binary_str = format!("{:b}", diff);

    // 交換したいセルの間にあるセルを動かす
    for (i, c) in diff_binary_str.chars().rev().enumerate() {
        if c == '1' {
            let cut_num = if i == 0 { 0 } else { 1 + 3 * (i - 1) } as u16;
            let size = 1 << i;
            let base_line = edge_pos - (sign * (size - 1));
            actions.push(Action::new_from_axis_point(
                line_lenght - size - 1,
                base_line,
                cut_num,
                direction,
            ));
        }
    }

    // 元の盤面で小さいほうにある塊の幅はmin_pointと同じ値
    actions.push(Action::new_from_axis_point(
        line_lenght - min_point - 1,
        edge_pos + (sign * -255),
        22,
        direction,
    ));
}
