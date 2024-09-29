use crate::board::action::{Action, Direction};
use crate::board::board::Board;
use crate::board::cut::Cuts;

pub const SCORE_MAX: u64 = 1000000000;
pub struct GreedyGame {
    pub state: GreedyState,
    pub cuts: Cuts,
    pub end: Board,
}
pub struct GreedyState {
    pub board: Board,
}

impl GreedyState {
    pub fn new(board: Board) -> GreedyState {
        GreedyState { board }
    }

    pub fn evaluate_score(&self, end: &Board) -> u64 {
        self.board.absolute_distance(end)
    }
}

impl GreedyGame {
    pub fn new(board: Board, cuts: Cuts, end: Board) -> GreedyGame {
        GreedyGame {
            state: GreedyState::new(board),
            cuts,
            end,
        }
    }

    pub fn get_all_legal_actions(&self) -> Vec<Action> {
        let mut legal_actions = Vec::new();
        for i in 0..(self.cuts.len() - 16) {
            for w in (1 - self.cuts[i as u32].width() as i32)..(self.state.board.width() as i32) {
                for h in
                    (1 - self.cuts[i as u32].height() as i32)..(self.state.board.height() as i32)
                {
                    for d in vec![
                        Direction::Up,
                        Direction::Down,
                        Direction::Left,
                        Direction::Right,
                    ] {
                        let action = Action::new(w, h, i as u8, d);
                        legal_actions.push(action);
                    }
                }
            }
        }
        legal_actions
    }

    pub fn greedy_acion(&self, state: &GreedyState) -> Action {
        let legal_actions = self.get_all_legal_actions();
        let mut min_score = SCORE_MAX;
        let mut min_action = Action::new(0, 0, 0, Direction::Up);

        for action in legal_actions {
            let mut board = state.board.clone();
            board.operate(&action);
            let score = board.absolute_distance(&self.end);
            println!("score: {}", score);
            println!(
                "action: x={} y={} cut={} direction={:?}",
                action.x(),
                action.y(),
                action.cut_num(),
                action.direction()
            );
            println!("------------------------------------");
            if score < min_score {
                min_score = score;
                min_action = action;
            }
        }

        // assert!(max_action != Action::new(0, 0, 0, Direction::Up));
        min_action
    }
}

pub fn play(game: &mut GreedyGame) -> Vec<Action> {
    println!("start");
    let mut actions = Vec::new();
    for i in 0..150 {
        println!("i: {}", i);
        let action = game.greedy_acion(&game.state);
        game.state.board.operate(&action);
        actions.push(action.clone());
        if game.state.board == game.end {
            break;
        }

        println!(
            "action: x={} y={} cut={} direction={:?}",
            action.x(),
            action.y(),
            action.cut_num(),
            action.direction()
        );
        println!("score: {}", game.state.evaluate_score(&game.end));
        println!("board: {}", game.state.board);
        println!("----------------------------------");
    }
    actions
}
