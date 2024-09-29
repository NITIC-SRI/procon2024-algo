use std::collections::VecDeque;
use std::fmt::Display;

use crate::board::action::Action;
use crate::board::cut::Cut;

use super::action;

#[derive(Debug, Clone)]
pub struct Board {
    pub board: Vec<Vec<u8>>,
    width: usize,
    height: usize,
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for row in &self.board {
            for cell in row {
                write!(f, "{}", cell)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl Board {
    pub fn new(board: Vec<Vec<u8>>) -> Board {
        let height = board.len();
        let width = board[0].len();
        Board {
            board,
            width,
            height,
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn board(&self) -> &Vec<Vec<u8>> {
        &self.board
    }

    pub fn operate(&mut self, action : &Action) {
        match action.direction() {
            action::Direction::Up => self.op_up(action.cut(), action.x(), action.y()),
            action::Direction::Down => self.op_down(action.cut(), action.x(), action.y()),
            action::Direction::Left => self.op_left(action.cut(), action.x(), action.y()),
            action::Direction::Right => self.op_right(action.cut(), action.x(), action.y()),
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

    // pub fn operate_and_absolute_distance(&mut self, cut: &Cut, x: i32, y: i32, direction: u8, end: &Self) -> (Action, u64) {

    // }

}
