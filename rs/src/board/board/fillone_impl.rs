use std::collections::HashMap;
use std::hash::Hash;

use super::Board;

use crate::board::action;
use crate::board::action::Action;
use crate::board::cut::Cuts;
use std::convert::Into;
use std::fmt::Debug;
use std::vec;

// 圧縮
impl<T> Board<T>
where
    T: Copy + PartialEq + Into<usize> + Debug,
{
    fn calc_complesed_action_num(
        &self,
        count: usize,
        action: Action,
        continue_count: usize,
    ) -> usize {
        if action.y() == 0 {
            match action.x() {
                0 => {
                    if continue_count == self.width {
                        count - continue_count
                    } else {
                        count + 1 - continue_count
                    }
                }
                x if x == self.width as i32 - continue_count as i32 => count - continue_count,
                _ => count - continue_count + continue_count.count_ones() as usize,
            }
        } else {
            count
        }
    }

    fn _compress_left(
        &self,
        actions: &Vec<Action>,
        consecutive: usize,
        i: usize,
    ) -> (bool, Vec<Action>) {
        let action = &actions[i - 1];
        if action.direction() == action::Direction::Left {
            if action.x() == 0 && action.y() == 0 {
                if consecutive == self.width() {
                    return (true, vec![]);
                } else {
                    return (
                        true,
                        vec![Action::new(
                            -256 + consecutive as i32,
                            -255,
                            22,
                            action::Direction::Left,
                        )],
                    );
                }
            } else if action.x() == self.width() as i32 - consecutive as i32 {
                return (true, vec![]);
            } else if action.x() != 0 && action.y() == 0 {
                let b = format!("{:b}", consecutive);
                let mut tmp_actions = vec![];
                for (idx, j) in b.chars().rev().enumerate() {
                    if j == '1' {
                        tmp_actions.push(Action::new(
                            action.x(),
                            1 - (1 << idx),
                            if idx == 0 { 0 } else { 1 + 3 * (idx - 1) } as u16,
                            action::Direction::Left,
                        ));
                    }
                }
                return (true, tmp_actions);
            }
        }

        return (false, vec![]);
    }

    fn _compress_rowup(
        &self,
        actions: &Vec<Action>,
        consecutive: usize,
        i: usize,
    ) -> (bool, Vec<Action>) {
        let action = &actions[i - 1];
        if action.direction() == action::Direction::Up {
            if consecutive == self.height() {
                return (true, vec![]);
            } else {
                return (
                    true,
                    vec![Action::new(
                        0,
                        -256 + consecutive as i32,
                        22,
                        action::Direction::Up,
                    )],
                );
            }
        }
        return (false, vec![]);
    }

    fn _compress_actions(
        &self,
        actions: &Vec<Action>,
        func: fn(&Self, &Vec<Action>, usize, usize) -> (bool, Vec<Action>),
    ) -> Vec<Action> {
        let mut compressed_actions = vec![];
        let mut consecutive = 1;
        for i in 1..actions.len() {
            compressed_actions.push(actions[i - 1].clone());
            if actions[i - 1] == actions[i] {
                consecutive += 1;
            } else {
                let (check, comp_action) = func(&self, &actions, consecutive, i);
                if check {
                    compressed_actions.splice(
                        compressed_actions.len() - consecutive..,
                        comp_action.iter().cloned(),
                    );
                }
                consecutive = 1;
            }
        }
        compressed_actions.push(actions[actions.len() - 1].clone());
        let (check, comp_action) = func(&self, &actions, consecutive, actions.len());
        if check {
            compressed_actions.splice(
                compressed_actions.len() - consecutive..,
                comp_action.iter().cloned(),
            );
        }
        return compressed_actions;
    }

    fn compress_actions(&self, actions: &Vec<Action>) -> Vec<Action> {
        let mut comped_actions: Vec<Action>;
        comped_actions = self._compress_actions(actions, Self::_compress_left);
        comped_actions = self._compress_actions(&comped_actions, Self::_compress_rowup);
        return comped_actions;
    }
}

// fillone本体
impl<T> Board<T>
where
    T: Copy + PartialEq + Into<usize> + Debug,
{
    pub fn get_fillone_actions(
        &self,
        end: &Self,
        row_num: usize,
        col_num: usize,
        is_compress: bool,
    ) -> Vec<Action> {
        let actions: &mut Vec<Action> = &mut vec![];
        self.fillone(end, row_num, col_num, Some(actions), is_compress);
        actions.to_vec()
    }

    pub fn fillone(
        &self,
        end: &Self,
        row_num: usize,
        col_num: usize,
        mut actions: Option<&mut Vec<Action>>,
        is_compress: bool,
    ) -> usize {
        let mut count: usize = 0;
        let mut continue_count: usize = 1;
        let mut rowup_continue_count: usize = 0;
        let mut before_action = Action::new(256, 256, 0, action::Direction::Up);

        let mut new = self.clone();

        // 終盤面についてのループ
        for y in row_num..self.height() {
            // skip_flag: 上の行がすでにそろっていたらtrue
            let mut skip_flag = true;
            if end.board[y] != new.board[0] {
                skip_flag = false;

                let col_num = if row_num == y { col_num } else { 0 };
                'loop_x: for x in col_num..self.width() {
                    // 一番上の行についてのループ
                    for w in 0..(self.width() - x) {
                        if end.board[y][x] == new.board[0][w] {
                            new.op_one_left(w as i32, 0 as i32);
                            count += 1;
                            if let Some(ref mut acts) = actions {
                                acts.push(Action::new(w as i32, 0, 0, action::Direction::Left));
                            }

                            // if cfg!(debug_assertions) {
                            //     println!("Action left {}, {}", w, 0);
                            // }

                            let tmp = Action::new(w as i32, 0, 0, action::Direction::Left);

                            if tmp == before_action {
                                continue_count += 1;
                            } else {
                                count = self.calc_complesed_action_num(
                                    count,
                                    before_action,
                                    continue_count,
                                );
                                continue_count = 1;
                                before_action = tmp;
                            }

                            continue 'loop_x;
                        };
                    }

                    // 移動させたい場所より左下についてのループ
                    for h in 1..(self.height() - y) {
                        for w in 0..(self.width() - x) {
                            if end.board[y][x] == new.board[h][w] {
                                new.op_one_down(w as i32, h as i32);
                                new.op_one_left(w as i32, 0 as i32);
                                count += 2;
                                if let Some(ref mut acts) = actions {
                                    acts.push(Action::new(
                                        w as i32,
                                        h as i32,
                                        0,
                                        action::Direction::Down,
                                    ));

                                    acts.push(Action::new(w as i32, 0, 0, action::Direction::Left));
                                }

                                // if cfg!(debug_assertions) {
                                //     println!("Action down {}, {}", w, h);
                                //     println!("Action left {}, {}", w, 0);
                                // }

                                count = self.calc_complesed_action_num(
                                    count,
                                    before_action,
                                    continue_count,
                                );

                                continue_count = 1;
                                before_action =
                                    Action::new(w as i32, 0, 0, action::Direction::Left);

                                continue 'loop_x;
                            }
                        }
                    }

                    // それ以外の場所についてのループ
                    for h in 1..(self.height() - y) {
                        for w in (self.width - x)..self.width() {
                            if end.board[y][x] == new.board[h][w] {
                                new.op_one_right(w as i32, h as i32);
                                new.op_one_down(0 as i32, h as i32);
                                new.op_one_left(0 as i32, 0 as i32);
                                count += 3;
                                if let Some(ref mut acts) = actions {
                                    acts.push(Action::new(
                                        w as i32,
                                        h as i32,
                                        0,
                                        action::Direction::Right,
                                    ));

                                    acts.push(Action::new(
                                        0 as i32,
                                        h as i32,
                                        0,
                                        action::Direction::Down,
                                    ));

                                    acts.push(Action::new(0 as i32, 0, 0, action::Direction::Left));
                                }

                                // if cfg!(debug_assertions) {
                                //     println!("Action right {}, {}", w, h);
                                //     println!("Action down {}, {}", 0, h);
                                //     println!("Action left {}, {}", 0, 0);
                                // }

                                count = self.calc_complesed_action_num(
                                    count,
                                    before_action,
                                    continue_count,
                                );

                                continue_count = 1;
                                before_action =
                                    Action::new(0 as i32, 0, 0, action::Direction::Left);

                                continue 'loop_x;
                            }
                        }
                    }
                }
            }

            new.op_row_up();
            count += 1;

            if let Some(ref mut acts) = actions {
                acts.push(Action::new(0 as i32, -255, 22, action::Direction::Up));
            }
            // if cfg!(debug_assertions) {
            //     println!("Action row_up {}, {}", 0, 0);
            //     println!();
            // }

            count = self.calc_complesed_action_num(count, before_action, continue_count);

            continue_count = 1;
            before_action = Action::new(0 as i32, -255, 22, action::Direction::Up);

            if skip_flag {
                rowup_continue_count += 1;
            } else {
                if rowup_continue_count != 0 {
                    count = count + 1 - rowup_continue_count;
                }
                rowup_continue_count = 1;
            }
        }

        if rowup_continue_count == new.height {
            count = count - rowup_continue_count;
        } else {
            count = count + 1 - rowup_continue_count;
        }

        if is_compress {
            if let Some(acts) = actions {
                *acts = self.compress_actions(acts);
            }
        }

        return count;
    }

    pub fn caterpillar_move(&self, top_x: usize, target_x: usize, target_y: usize) -> Vec<Action> {
        assert_ne!(target_y, 0, "target_y must be greater than 0");
        let mut actions: Vec<Action> = vec![];

        let x_diff = top_x as i32 - target_x as i32;

        if x_diff > 0 {
            actions.push(Action::new(
                -256 + x_diff,
                -255,
                22,
                action::Direction::Left,
            ));
        } else if x_diff < 0 {
            actions.push(Action::new(
                self.width() as i32 + x_diff,
                -255,
                22,
                action::Direction::Right,
            ));
        }

        actions.push(Action::new(
            target_x as i32,
            target_y as i32,
            0,
            action::Direction::Down,
        ));

        if x_diff > 0 {
            actions.push(Action::new(
                self.width() as i32 - x_diff,
                -255,
                22,
                action::Direction::Right,
            ));
        } else if x_diff < 0 {
            actions.push(Action::new(
                -256 - x_diff,
                -255,
                22,
                action::Direction::Left,
            ));
        }

        actions
    }

    pub fn half_caterpillar_move(
        &self,
        top_x: usize,
        target_x: usize,
        target_y: usize,
    ) -> Vec<Action> {
        assert_ne!(target_y, 0, "target_y must be greater than 0");
        let mut actions: Vec<Action> = vec![];

        let x_diff = top_x as i32 - target_x as i32;

        if x_diff > 0 {
            actions.push(Action::new(
                -256 + x_diff,
                -255,
                22,
                action::Direction::Left,
            ));
        } else if x_diff < 0 {
            actions.push(Action::new(
                self.width() as i32 + x_diff,
                -255,
                22,
                action::Direction::Right,
            ));
        }

        actions.push(Action::new(
            target_x as i32,
            target_y as i32,
            0,
            action::Direction::Down,
        ));

        actions
    }
    pub fn get_fillone_action_score(&self, end: &Self) -> usize {
        self.fillone(end, 0, 0, None, true)
    }

    pub fn check_progress(&self, end: &Self) -> (usize, usize) {
        let (h, w) = (self.height(), self.width());
        let s_vec = self.board();
        let e_vec = end.board();
        let row_count = {
            let mut row_count = 0;
            for tmp_row_count in (1..=h).rev() {
                if e_vec[..tmp_row_count] == s_vec[h - tmp_row_count..] {
                    row_count = tmp_row_count;
                    break;
                }
            }
            row_count
        };

        let col_count = {
            let mut col_count = 0;
            let erow = &e_vec[row_count];
            for tmp_col_count in (1..=w).rev() {
                if erow[..tmp_col_count] == s_vec[0][w - tmp_col_count..] {
                    col_count = tmp_col_count;
                    break;
                }
            }
            col_count
        };
        (row_count, col_count)
    }

    pub fn get_fillone_score_intermediate(
        &self,
        end: &Self,
        row_num: usize,
        col_num: usize,
    ) -> usize {
        self.fillone(end, row_num, col_num, None, true)
    }
}

impl<T> Board<T>
where
    T: Copy + PartialEq + Into<usize> + Debug + Eq + Hash,
{
    pub fn line_fillone(&self, end: &Self, target_row: usize) -> Vec<Action> {
        let start_row = Board::new(vec![self.board()[0].clone()]);
        let end_row = Board::new(vec![end.board()[target_row].clone()]);

        if cfg!(debug_assertions) {
            let mut counts = HashMap::new();
            for i in 0..self.width() {
                *counts.entry(start_row.board()[0][i]).or_insert(0) += 1;
                *counts.entry(end_row.board()[0][i]).or_insert(0) -= 1;
            }
            assert!(
                counts.values().all(|&x| x == 0),
                "line_fillone: start and end row must have same number of each element"
            );
        }
        let actions = start_row.get_fillone_actions(&end_row, 0, 0, true);
        actions
    }

    pub fn caterpillar_and_line_fillone(&self, end: &Self, usable_height: usize) -> Vec<Action> {
        let end_row_y = self.height() - usable_height;
        let mut new = self.clone();
        let mut actions = vec![];

        let mut row_cnt: Vec<i32> = vec![0; 4];
        // let mut wrongs: Vec<Vec<usize>> = vec![vec![]; 4];

        for w in 0..self.width() {
            if self.board()[0][w] == end.board()[end_row_y][w] {
                continue;
            }
            row_cnt[self.board()[0][w].into()] += 1;
            row_cnt[end.board()[end_row_y][w].into()] -= 1;
        }
        let cuts = Cuts::new("../data/formal_cuts.json".to_string());
        // targetnum: ほしい数
        for (target_num, &cnt) in row_cnt.clone().iter().enumerate() {
            if cnt >= 0 {
                continue;
            }

            for _ in 0..-cnt {
                'loop1: for source_x in 0..self.width() {
                    if row_cnt[new.board()[0][source_x].into()] > 0 {
                        for y in 1..usable_height {
                            for x in 0..self.width() {
                                if new.board()[y][x].into() == target_num {
                                    row_cnt[new.board()[0][source_x].into()] -= 1;
                                    let tmp = new.half_caterpillar_move(source_x, x, y);
                                    new.operate_actions(tmp.clone(), &cuts);
                                    actions.extend(tmp);
                                    break 'loop1;
                                }
                            }
                        }
                    }
                }
            }
        }

        // println!("inner caterpillar actions: {}", actions.len());

        actions.extend(new.line_fillone(end, end_row_y));

        // println!("inner actions len: {}", actions.len());
        // println!("actions sequence: {:?}", actions);

        actions
    }

    pub fn try_only_caterpillar(&self, end: &Self, usable_height: usize) -> (bool, Vec<Action>) {
        let end_row_y = self.height() - usable_height;
        let mut new = self.clone();
        let mut actions = vec![];

        let cuts = Cuts::new("../data/formal_cuts.json".to_string());
        // let mut wrongs: Vec<Vec<usize>> = vec![vec![]; 4];

        'loop_w: for w in 0..self.width() {
            if self.board()[0][w] == end.board()[end_row_y][w] {
                continue;
            }
            for y in 1..usable_height {
                for x in 0..self.width() {
                    if new.board()[y][x] == end.board()[end_row_y][w] {
                        let caterpillar_actions = new.caterpillar_move(w, x, y);
                        new.operate_actions(caterpillar_actions.clone(), &cuts);
                        actions.extend(caterpillar_actions);
                        continue 'loop_w;
                    }
                }
            }
            return (false, vec![]);
        }
        return (true, actions);
    }
}
