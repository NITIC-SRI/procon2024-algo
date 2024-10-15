use super::Board;

use crate::board::action;
use crate::board::action::Action;
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
    pub fn _get_fillone_actions(&self, end: &Self, is_compress: bool) -> Vec<Action> {
        let mut actions = vec![];
        let mut new = self.clone();

        for y in 0..self.height() {
            'loop_x: for x in 0..self.width() {
                for w in 0..(self.width() - x) {
                    if end.board[y][x] == new.board[0][w] {
                        new.op_one_left(w as i32, 0 as i32);
                        actions.push(Action::new(w as i32, 0, 0, action::Direction::Left));
                        continue 'loop_x;
                    }
                }

                for h in 1..self.height() - y {
                    for w in 0..self.width() - x {
                        if end.board[y][x] == new.board[h][w] {
                            new.op_one_down(w as i32, h as i32);
                            new.op_one_left(w as i32, 0 as i32);
                            actions.push(Action::new(
                                w as i32,
                                h as i32,
                                0,
                                action::Direction::Down,
                            ));
                            actions.push(Action::new(w as i32, 0, 0, action::Direction::Left));
                            continue 'loop_x;
                        }
                    }
                }

                for h in 1..self.height() - y {
                    for w in self.width() - x..self.width() {
                        if end.board[y][x] == new.board[h][w] {
                            new.op_one_right(w as i32, h as i32);
                            new.op_one_down(0 as i32, h as i32);
                            new.op_one_left(0 as i32, 0 as i32);
                            actions.push(Action::new(
                                w as i32,
                                h as i32,
                                0,
                                action::Direction::Right,
                            ));
                            actions.push(Action::new(
                                0 as i32,
                                h as i32,
                                0,
                                action::Direction::Down,
                            ));
                            actions.push(Action::new(0 as i32, 0, 0, action::Direction::Left));
                            continue 'loop_x;
                        }
                    }
                }
            }
            new.op_row_up();
            actions.push(Action::new(0, -255, 22, action::Direction::Up));
        }

        if is_compress {
            actions = self.compress_actions(&actions)
        }
        actions
    }

    pub fn get_fillone_actions(&self, end: &Self) -> Vec<Action> {
        self._get_fillone_actions(end, true)
    }
    pub fn get_fillone_action_score(&self, end: &Self) -> usize {
        let mut count: usize = 0;
        let mut continue_count: usize = 1;
        let mut rowup_continue_count: usize = 0;
        let mut before_action = Action::new(256, 256, 0, action::Direction::Up);

        let mut new = self.clone();

        // 終盤面についてのループ
        for y in 0..self.height() {
            // skip_flag: 上の行がすでにそろっていたらtrue
            let mut skip_flag = true;
            if end.board[y] != new.board[0] {
                skip_flag = false;
                'loop_x: for x in 0..self.width() {
                    // 一番上の行についてのループ
                    for w in 0..(self.width() - x) {
                        if end.board[y][x] == new.board[0][w] {
                            new.op_one_left(w as i32, 0 as i32);
                            count += 1;
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

        return count;
    }

    pub fn check_progress(&self, end: &Self) -> (usize, usize) {
        let (h, w) = (self.height(), self.width());
        let s_vec = self.board();
        let e_vec = end.board();
        let row_count = {
            let mut tmp_row_count = 0;
            for ev in e_vec.iter().rev() {
                if ev == &s_vec[h - tmp_row_count - 1] {
                    tmp_row_count += 1;
                } else {
                    tmp_row_count = 0
                }
            }
            tmp_row_count
        };

        let col_count = {
            let mut tmp_col_count = 0;
            let erow = &e_vec[row_count];
            for ec in erow.iter().rev() {
                if ec == &s_vec[0][w - tmp_col_count - 1] {
                    tmp_col_count += 1;
                } else {
                    tmp_col_count = 0
                }
            }
            tmp_col_count
        };
        (row_count, col_count)
    }

}
