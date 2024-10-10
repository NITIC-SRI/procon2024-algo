use std::collections::VecDeque;
use std::convert::Into;
use std::fmt::{Debug, Display};

use crate::board::action;
use crate::board::action::Action;
use crate::board::cut::{Cut, Cuts};

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct Board<T = u8>
where
    T: Copy + PartialEq + Into<usize> + Debug,
{
    pub board: Vec<Vec<T>>,
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

impl<T> Board<T>
where
    T: Copy + PartialEq + Into<usize> + Debug,
{
    pub fn new(board: Vec<Vec<T>>) -> Self {
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

    pub fn board(&self) -> &Vec<Vec<T>> {
        &self.board
    }

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

    pub fn operate_one(&mut self, action: &Action) {
        match action.direction() {
            action::Direction::Up => self.op_one_up(action.x(), action.y()),
            action::Direction::Down => self.op_one_down(action.x(), action.y()),
            action::Direction::Left => self.op_one_left(action.x(), action.y()),
            action::Direction::Right => self.op_one_right(action.x(), action.y()),
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

    pub fn get_fillone_actions(&self, end: &Self) -> Vec<Action> {
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

        actions = self.compress_actions(&actions);

        actions
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
