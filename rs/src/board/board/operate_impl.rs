use super::Board;

use crate::board::action;
use crate::board::action::Action;
use crate::board::cut::{Cut, Cuts};
use std::collections::VecDeque;
use std::convert::Into;
use std::fmt::Debug;

impl<T> Board<T>
where
    T: Copy + PartialEq + Into<usize> + Debug,
{
    pub fn operate(&mut self, action: &Action, cuts: &Cuts) {
        match action.direction() {
            action::Direction::Up => {
                self.op_up(&cuts[action.cut_num() as u32], action.x(), action.y())
            }
            action::Direction::Down => {
                self.op_down(&cuts[action.cut_num() as u32], action.x(), action.y())
            }
            action::Direction::Left => {
                self.op_left(&cuts[action.cut_num() as u32], action.x(), action.y())
            }
            action::Direction::Right => {
                self.op_right(&cuts[action.cut_num() as u32], action.x(), action.y())
            }
        }
    }

    pub fn operate_actions(&mut self, actions: Vec<Action>, cuts: &Cuts) {
        for action in actions {
            self.operate(&action, cuts);
        }
    }

    pub fn op_left(&mut self, cut: &Cut, x: i32, y: i32) {
        let mut q = VecDeque::new();
        for h in y..(y + cut.height() as i32) {
            if !(0 <= h && h < self.height as i32) {
                continue;
            }

            for w in x..(self.width as i32) {
                if !(0 <= w && w < self.width as i32) {
                    continue;
                }

                if w >= x + cut.width() as i32 {
                    q.push_back(self.board[h as usize][w as usize]);
                    continue;
                }

                if !cut[(h - y) as usize][(w - x) as usize] {
                    q.push_back(self.board[h as usize][w as usize]);
                }
            }

            for w in x..(x + cut.width() as i32) {
                if !(0 <= w && w < self.width as i32) {
                    continue;
                }

                if cut[(h - y) as usize][(w - x) as usize] {
                    q.push_back(self.board[h as usize][w as usize]);
                }
            }

            for w in x..(self.width as i32) {
                if !(0 <= w && w < self.width as i32) {
                    continue;
                }

                self.board[h as usize][w as usize] = q.pop_front().unwrap();
            }
        }

        assert!(q.is_empty());
    }

    pub fn op_right(&mut self, cut: &Cut, x: i32, y: i32) {
        let mut q = VecDeque::new();
        for h in y..(y + cut.height() as i32) {
            if !(0 <= h && h < self.height as i32) {
                continue;
            }

            for w in 0..(cut.width() as i32 + x) as i32 {
                if !(0 <= w && w < self.width as i32) {
                    continue;
                }

                if w < x {
                    q.push_back(self.board[h as usize][w as usize]);
                    continue;
                }

                if !cut[(h - y) as usize][(w - x) as usize] {
                    q.push_back(self.board[h as usize][w as usize]);
                }
            }

            for w in (x..(x + cut.width() as i32)).rev() {
                if !(0 <= w && w < self.width as i32) {
                    continue;
                }

                if cut[(h - y) as usize][(w - x) as usize] {
                    q.push_front(self.board[h as usize][w as usize]);
                }
            }

            for w in 0..(cut.width() as i32 + x) {
                if !(0 <= w && w < self.width as i32) {
                    continue;
                }

                self.board[h as usize][w as usize] = q.pop_front().unwrap();
            }
        }

        assert!(q.is_empty());
    }

    pub fn op_up(&mut self, cut: &Cut, x: i32, y: i32) {
        let mut q = VecDeque::new();
        for w in x..(x + cut.width() as i32) {
            if !(0 <= w && w < self.width as i32) {
                continue;
            }

            for h in y..(self.height as i32) {
                if !(0 <= h && h < self.height as i32) {
                    continue;
                }

                if h >= y + cut.height() as i32 {
                    q.push_back(self.board[h as usize][w as usize]);
                    continue;
                }

                if !cut[(h - y) as usize][(w - x) as usize] {
                    q.push_back(self.board[h as usize][w as usize]);
                }
            }

            for h in y..(y + cut.height() as i32) {
                if !(0 <= h && h < self.height as i32) {
                    continue;
                }

                if cut[(h - y) as usize][(w - x) as usize] {
                    q.push_back(self.board[h as usize][w as usize]);
                }
            }

            for h in y..(self.height as i32) {
                if !(0 <= h && h < self.height as i32) {
                    continue;
                }

                self.board[h as usize][w as usize] = q.pop_front().unwrap();
            }
        }

        assert!(q.is_empty());
    }

    pub fn op_down(&mut self, cut: &Cut, x: i32, y: i32) {
        let mut q = VecDeque::new();
        for w in x..(x + cut.width() as i32) {
            if !(0 <= w && w < self.width as i32) {
                continue;
            }

            for h in 0..(cut.height() as i32 + y) as i32 {
                if !(0 <= h && h < self.height as i32) {
                    continue;
                }

                if h < y {
                    q.push_back(self.board[h as usize][w as usize]);
                    continue;
                }

                if !cut[(h - y) as usize][(w - x) as usize] {
                    q.push_back(self.board[h as usize][w as usize]);
                }
            }

            for h in (y..(y + cut.height() as i32)).rev() {
                if !(0 <= h && h < self.height as i32) {
                    continue;
                }

                if cut[(h - y) as usize][(w - x) as usize] {
                    q.push_front(self.board[h as usize][w as usize]);
                }
            }

            for h in 0..(cut.height() as i32 + y) {
                if !(0 <= h && h < self.height as i32) {
                    continue;
                }

                self.board[h as usize][w as usize] = q.pop_front().unwrap();
            }
        }

        assert!(q.is_empty());
    }

    #[allow(dead_code)]
    pub(super) fn op_one_up(&mut self, x: i32, y: i32) {
        // 操作後に盤面が変わらない操作を判定
        if y == self.height() as i32 - 1 {
            return;
        }

        let e = self.board[y as usize][x as usize];

        let mut h: usize = 0;
        let mut hp: usize = 0;
        while h < self.height() {
            if h as i32 == y {
                h += 1
            }
            self.board[hp][x as usize] = self.board[h][x as usize];
            h += 1;
            hp += 1;
        }
        let tmp: usize = self.height() - 1;
        self.board[tmp][x as usize] = e
    }

    pub(super) fn op_one_down(&mut self, x: i32, y: i32) {
        // 操作後に盤面が変わらない操作を判定
        if y == 0 {
            return;
        }

        let e = self.board[y as usize][x as usize];

        let mut h = self.height() as i32 - 1;
        let mut hp = self.height() as i32 - 1;
        while h >= 0 {
            if h == y {
                h -= 1;
            }
            self.board[hp as usize][x as usize] = self.board[h as usize][x as usize];
            h -= 1;
            hp -= 1;
        }
        self.board[0][x as usize] = e;
    }

    pub(super) fn op_one_left(&mut self, x: i32, y: i32) {
        // 操作後に盤面が変わらない操作を判定
        if x == self.width() as i32 - 1 {
            return;
        }

        let e = self.board[y as usize][x as usize];

        let mut w: usize = 0;
        let mut wp: usize = 0;
        while w < self.width() {
            if w as i32 == x {
                w += 1;
            }
            self.board[y as usize][wp] = self.board[y as usize][w];
            w += 1;
            wp += 1;
        }
        let tmp: usize = self.width() - 1;
        self.board[y as usize][tmp] = e;
    }

    pub(super) fn op_one_right(&mut self, x: i32, y: i32) {
        // 操作後に盤面が変わらない操作を判定
        if x == 0 {
            return;
        }

        let e = self.board[y as usize][x as usize];

        let mut w = self.width() as i32 - 1;
        let mut wp = self.width() as i32 - 1;
        while w >= 0 {
            if w == x {
                w -= 1;
            }
            self.board[y as usize][wp as usize] = self.board[y as usize][w as usize];
            w -= 1;
            wp -= 1;
        }
        self.board[y as usize][0] = e;
    }

    pub fn op_row_up(&mut self) {
        let last = self.board[0].clone();
        for h in 0..(self.height() - 1) {
            self.board[h] = self.board[h + 1].clone();
        }
        let last_index = { self.height() - 1 };

        self.board[last_index] = last
    }
}
