use crate::board::action::{Action, Direction};
use crate::board::board::Board;
use crate::board::cut::Cuts;
use crate::utils;

use rand::rngs::SmallRng;
use rand::seq::SliceRandom;
use rand::SeedableRng;
use rayon::prelude::*;

pub struct DownFillOne<'a> {
    now_board: Board,
    end: &'a Board,
    down_only_actions: &'a Vec<Action>,
    x_only_actions: &'a Vec<Action>,
    cuts: &'a Cuts,
    usable_height: usize,
    actions: Vec<Action>,
    n_simulations: usize,
    base_down_only_actions: Vec<Action>,
    base_x_only_actions: Vec<Action>,
}

impl DownFillOne<'_> {
    pub fn new<'a>(
        start: Board,
        end: &'a Board,
        down_only_actions: &'a Vec<Action>,
        x_only_actions: &'a Vec<Action>,
        cuts: &'a Cuts,
        n_simulations: usize,
    ) -> DownFillOne<'a> {
        let mut base_down_only_actions = vec![];
        let mut base_x_only_actions = vec![];
        for action in down_only_actions.iter() {
            if action.cut_num() != 0 {
                continue;
            }
            if action.direction() == Direction::Down {
                base_down_only_actions.push(action.clone());
            } else if action.direction() == Direction::Right
                || action.direction() == Direction::Left
            {
                base_x_only_actions.push(action.clone());
            }
        }

        DownFillOne {
            now_board: start.clone(),
            end,
            down_only_actions,
            x_only_actions,
            cuts,
            usable_height: start.height(),
            actions: Vec::new(),
            n_simulations,
            base_down_only_actions,
            base_x_only_actions,
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
        let mut rng = SmallRng::from_entropy();
        let mut random_legal_actions: Vec<Action> = self
            .x_only_actions
            .choose_multiple(&mut rng, self.n_simulations)
            .cloned()
            .collect();
        random_legal_actions.extend(self.base_x_only_actions.clone());

        for action in random_legal_actions.iter() {
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

    pub fn down_greedy_action(&self) -> (Action, u64, Vec<usize>) {
        let mut rng = SmallRng::from_entropy();
        let mut random_legal_actions: Vec<Action> = self
            .down_only_actions
            .choose_multiple(&mut rng, self.n_simulations)
            .cloned()
            .collect();
        random_legal_actions.extend(self.base_down_only_actions.clone());
        let (min_action, min_distance, min_diff) = random_legal_actions
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
    }
}

pub fn play<'a>(
    start: &Board,
    end: &Board,
    legal_actions: &Vec<Action>,
    cuts: &Cuts,
    n_simulations: usize,
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
        n_simulations,
    );

    // TODO: タイムキーパー
    for _ in 0..max_iterations {
        if down_fillone_game.usable_height == 1 {
            down_fillone_game.caterpillar_and_line_fillone();
            down_fillone_game.complete_top_row();
            println!("break");
            break;
        }
        println!("{}", down_fillone_game.now_board);
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
                } else {
                    down_fillone_game.caterpillar_and_line_fillone();
                }
            } else {
                down_fillone_game.operate(&action);
                continue;
            }
        }

        down_fillone_game.complete_top_row();
        if down_fillone_game.done() {
            break;
        }
    }
    down_fillone_game.actions
}
