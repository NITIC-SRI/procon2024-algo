use crate::board::action::{Action, Direction};
use crate::board::board::Board;
use crate::board::cut::Cuts;

pub const SCORE_MAX: u64 = 1000000;
pub struct GreedyGame<'a> {
    pub state: GreedyState<'a>,
    pub cuts: &'a Cuts,
    pub end: &'a Board,
    pub legal_actions: &'a Vec<Action>,
    pub turn: usize,
}
pub struct GreedyState<'a> {
    pub board: &'a mut Board,
}

impl<'a> GreedyState<'a> {
    pub fn new(board: &'a mut Board) -> GreedyState<'a> {
        GreedyState { board }
    }
}

impl<'a> GreedyGame<'a> {
    pub fn new(board: &'a mut Board, cuts: &'a Cuts, end: &'a Board, legal_actions: &'a Vec<Action>) -> GreedyGame<'a> {
        GreedyGame {
            state: GreedyState::new(board),
            cuts,
            end,
            legal_actions,
            turn: 0,
        }
    }

    pub fn evaluate_score(&self, end: &Board) -> u64 {
        self.state.board.absolute_distance(end)
    }

    pub fn greedy_acion(&self, state: &GreedyState) -> Action {
        let mut min_score = SCORE_MAX;
        let mut min_action = Action::new(0, 0, 0, Direction::Up);

        for action in self.legal_actions {
            let mut board = state.board.clone();
            board.operate(&action, &self.cuts);
            let score = board.get_fillone_action_score(&self.end) as u64;
            if score < min_score {
                min_score = score;
                min_action = action.clone();
            }
        }

        println!("min_score: {}", min_score);
        min_action
    }
}

pub fn play(game: &mut GreedyGame) -> Vec<Action> {
    let mut actions = Vec::new();

    // TODO: タイムキーパーを設定する
    for i in 0..2000 {
        println!("i: {}", i);
        let now_board = game.state.board.clone();
        let action = game.greedy_acion(&game.state);
        game.state.board.operate(&action, game.cuts);

        if i > 0 && actions[actions.len() - 1] == action {
            break;
        }
        actions.push(action.clone());
        println!("action: {:?}", action);
        println!("{}", game.state.board);
        if game.state.board == game.end {
            break;
        }
        if *game.state.board == now_board {
            break;
        }
        game.turn += 1;
        println!("score: {}", game.evaluate_score(&game.end));
    }
    actions
}
