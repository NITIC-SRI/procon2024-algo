use crate::board::action::{Action, Direction};
use crate::board::board::Board;
use crate::board::cut::Cuts;
use crate::utils;
use rayon::prelude::*;

pub struct DownFillOne<'a> {
    now_board: Board,
    end: &'a Board,
    down_only_actions: &'a Vec<Action>,
    x_only_actions: &'a Vec<Action>,
    cuts: &'a Cuts,
    usable_height: usize,
    actions: Vec<Action>,
    count: Vec<usize>,
}

impl DownFillOne<'_> {
    pub fn new<'a>(
        start: Board,
        end: &'a Board,
        down_only_actions: &'a Vec<Action>,
        x_only_actions: &'a Vec<Action>,
        cuts: &'a Cuts,
    ) -> DownFillOne<'a> {
        DownFillOne {
            now_board: start.clone(),
            end,
            down_only_actions,
            x_only_actions,
            cuts,
            usable_height: start.height(),
            actions: Vec::new(),
            count: vec![0; 3], // 0: down, 1: x, 2: fillone
        }
    }

    pub fn complete_top_row(&mut self) {
        self.now_board.op_row_up();
        self.usable_height -= 1;
        self.actions.push(Action::new(0, -255, 22, Direction::Up));
    }

    pub fn operate(&mut self, action: &Action) {
        self.now_board.operate(action, self.cuts);
        self.actions.push(action.clone());
    }

    pub fn done(&self) -> bool {
        self.now_board == *self.end || self.usable_height == 0
    }

    pub fn greedy_match_x_direction_action(&self, diff: &Vec<usize>) -> (Action, u64) {
        let mut min_distance: u64 = std::u64::MAX;
        let mut min_action = Action::new(0, 0, 0, Direction::Down);
        let mut min_distance_col_distance = std::u64::MAX;

        for action in self.x_only_actions {
            let mut next_board = self.now_board.clone();

            if action.y() + self.cuts[action.cut_num() as u32].height() as i32
                > self.usable_height as i32
            {
                continue;
            }

            next_board.operate(action, self.cuts);
            // let distance = next_board.match_x_direction_score(&self.end, &diff, self.usable_height);
            let (distance, col_score) =
                next_board.match_x_direction_and_col_score(&self.end, &diff, self.usable_height);

            if min_distance > distance {
                min_distance = distance;
                min_action = action.clone();
            }

            if min_distance == distance && min_distance_col_distance > col_score {
                min_distance_col_distance = col_score;
                min_action = action.clone();
            }
        }

        (min_action, min_distance)
    }

    // pub fn down_greedy_action(&self) -> (Action, u64, Vec<usize>) {
    //     let mut min_distance: u64 = std::u64::MAX;
    //     let mut min_action = Action::new(0, 0, 0, Direction::Down);
    //     let mut min_diff = vec![std::usize::MAX];

    //     for action in self.down_only_actions {
    //         if action.y() + self.cuts[action.cut_num() as u32].height() as i32
    //             > self.usable_height as i32
    //         {
    //             continue;
    //         }

    //         let (distance, diff) = self.now_board.no_op_top_distance(
    //             &self.end,
    //             self.usable_height,
    //             self.cuts,
    //             &action,
    //         );

    //         if distance < min_distance {
    //             min_distance = distance;
    //             min_action = action.clone();
    //             min_diff = diff;
    //         }
    //     }

    //     (min_action, min_distance, min_diff)
    // }

    pub fn down_greedy_action(&self) -> (Action, u64, Vec<usize>) {
        let (min_action, min_distance, min_diff) = self
            .down_only_actions
            .par_iter()
            .filter_map(|action| {
                if action.y() + self.cuts[action.cut_num() as u32].height() as i32
                    > self.usable_height as i32
                {
                    return None;
                }

                let (distance, diff) = self.now_board.no_op_top_distance(
                    &self.end,
                    self.usable_height,
                    self.cuts,
                    action,
                );

                Some((action.clone(), distance, diff))
            })
            .min_by_key(|&(_, distance, _)| distance)
            .unwrap_or((
                Action::new(0, 0, 0, Direction::Down),
                std::u64::MAX,
                vec![std::usize::MAX],
            ));

        (min_action, min_distance, min_diff)
    }

    pub fn caterpillar_and_line_fillone(&mut self) {
        let mut top_distance = 0;
        for w in 0..self.now_board.width() {
            if self.now_board.board()[0][w]
                != self.end.board()[self.now_board.height() - self.usable_height][w]
            {
                top_distance += 1;
            }
        }

        let actions = self
            .now_board
            .caterpillar_and_line_fillone(&self.end, self.usable_height);
        for action in actions.iter() {
            self.operate(action);
        }

        println!(
            "caterpillar fillone: \n actions: {}, distance: {}",
            actions.len(),
            top_distance
        );

        self.count[2] += actions.len();
    }
}

pub fn play<'a>(
    start: &Board,
    end: &Board,
    legal_actions: &Vec<Action>,
    cuts: &Cuts,
    max_iterations: usize,
) -> Vec<Action> {
    let mut cuts = cuts.clone();
    cuts.delete_only_zero_bottoms();
    let (down_only_actions, x_only_actions) = utils::get_action_by_direction(&legal_actions);
    let mut down_fillone_game = DownFillOne::new(
        start.clone(),
        end,
        &down_only_actions,
        &x_only_actions,
        &cuts,
    );

    // TODO: タイムキーパー
    for _i in 0..max_iterations {
        if down_fillone_game.usable_height == 1 {
            down_fillone_game.caterpillar_and_line_fillone();
            down_fillone_game.complete_top_row();
            println!("break");
            break;
        }
        // println!("iter: {}", i);
        // println!("{}", down_fillone_game.now_board);
        println!("height: {}", down_fillone_game.usable_height);

        // 一番上の行に寄せられるだけ寄せる
        let mut prev_distance = std::u64::MAX;
        let mut last_diff = vec![];
        loop {
            let (action, distance, diff) = down_fillone_game.down_greedy_action();
            if prev_distance <= distance {
                break;
            }
            down_fillone_game.operate(&action);
            down_fillone_game.count[0] += 1;

            prev_distance = distance;
            last_diff = diff;
            if distance == 0 {
                break;
            }
        }

        // 一番上の行で揃えられないものが存在するなら横に篩う
        if !last_diff.is_empty() {
            let (action, distance) = down_fillone_game.greedy_match_x_direction_action(&last_diff);
            if last_diff.len() == distance as usize {
                let (res, actions) = down_fillone_game
                    .now_board
                    .try_only_caterpillar(down_fillone_game.end, down_fillone_game.usable_height);
                if res {
                    for action in actions.iter() {
                        down_fillone_game.operate(action);
                    }
                    down_fillone_game.count[2] += actions.len();
                } else {
                    down_fillone_game.caterpillar_and_line_fillone();
                }
            } else {
                down_fillone_game.count[1] += 1;

                down_fillone_game.operate(&action);
                continue;
            }
        }

        down_fillone_game.complete_top_row();
        if down_fillone_game.done() {
            break;
        }
    }

    for (i, &c) in down_fillone_game.count.iter().enumerate() {
        println!("{}: {}", i, c);
    }
    down_fillone_game.actions
}
