use std::collections::VecDeque;
use std::fmt::Display;

use crate::board::action::Action;
use crate::board::cut::Cut;

use super::action::{self, Direction};

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

    pub fn operate(&mut self, action: &Action) {
        match action.direction() {
            action::Direction::Up => {
                self.op_up(&Cut::new(vec![vec![true]]), action.x(), action.y())
            }
            action::Direction::Down => {
                self.op_down(&Cut::new(vec![vec![true]]), action.x(), action.y())
            }
            action::Direction::Left => {
                self.op_left(&Cut::new(vec![vec![true]]), action.x(), action.y())
            }
            action::Direction::Right => {
                self.op_right(&Cut::new(vec![vec![true]]), action.x(), action.y())
            }
            _ => unreachable!(),
        }
    }

    pub fn operate_one(&mut self, action: &Action) {
        match action.direction() {
            action::Direction::Up => self.op_one_up(action.x(), action.y()),
            action::Direction::Down => self.op_one_down(action.x(), action.y()),
            action::Direction::Left => self.op_one_left(action.x(), action.y()),
            action::Direction::Right => self.op_one_right(action.x(), action.y()),
            _ => unreachable!(),
        }
    }

    pub fn operate_actions(&mut self, actions: Vec<Action>) {
        for action in actions {
            self.operate(&action);
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

    fn op_one_up(&mut self, x: i32, y: i32) {
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

    fn op_one_down(&mut self, x: i32, y: i32) {
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

    fn op_one_left(&mut self, x: i32, y: i32) {
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

    fn op_one_right(&mut self, x: i32, y: i32) {
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

    fn op_row_up(&mut self) {
        let last = self.board[0].clone();
        for h in 0..(self.height() - 1) {
            self.board[h] = self.board[h + 1].clone();
        }
        let last_index = { self.height() - 1 };

        self.board[last_index] = last
    }

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

    pub fn get_fillone_action_score(&self, end: &Self) -> usize {
        let mut count: usize = 0;
        let mut continue_count: usize = 1;
        let mut rowup_continue_count: usize = 1;
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
                            if cfg!(debug_assertions) {
                                println!("Action left {}, {}", w, 0);
                            }

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

                                if cfg!(debug_assertions) {
                                    println!("Action down {}, {}", w, h);
                                    println!("Action left {}, {}", w, 0);
                                }

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

                                if cfg!(debug_assertions) {
                                    println!("Action right {}, {}", w, h);
                                    println!("Action down {}, {}", 0, h);
                                    println!("Action left {}, {}", 0, 0);
                                }

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

            if cfg!(debug_assertions) {
                println!("Action row_up {}, {}", 0, 0);
                println!();
            }

            count = self.calc_complesed_action_num(count, before_action, continue_count);

            continue_count = 1;
            before_action = Action::new(0 as i32, -255, 22, action::Direction::Up);

            if skip_flag {
                rowup_continue_count += 1;
            } else {
                count = count + 1 - rowup_continue_count;
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

    fn swapping_one_line(self, x1: i32, y1: i32, x2: i32, y2: i32) -> Vec<Action> {
        let mut actions = vec![];
        if x1 == x2 {
            // 1列での入れ替え
            let x = x1;
            let (min, max) = if y1 < y2 { (y1, y2) } else { (y2, y1) };
            let (dir_min, dir_max) = (Direction::Down, Direction::Up);

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
            let (dir_min, dir_max) = (Direction::Right, Direction::Left);

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
    fn swap(&mut self, x1: i32, y1: i32, x2: i32, y2: i32) {
        let tmp = self.board[y1 as usize][x1 as usize];
        self.board[y1 as usize][x1 as usize] = self.board[y2 as usize][x2 as usize];
        self.board[y2 as usize][x2 as usize] = tmp;
    }

    pub fn swapping(&mut self, x1: i32, y1: i32, x2: i32, y2: i32) -> Vec<Action> {
        let mut actions = vec![];
        if x1 == x2 || y1 == y2 {
            // 1行または1列での入れ替え
            // まずはスコア比較
            let new = self.clone();
            actions.extend(new.swapping_one_line(x1, y1, x2, y2));
        }

        for action in actions.iter() {
            self.operate(action);
        }

        self.swap(x1, y1, x2, y2);
        actions
    }
}
