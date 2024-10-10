use crate::board::action::{Action, Direction};
use crate::board::board::Board;
use crate::board::cut::Cuts;
use crate::search::game::{Game, State};

pub const SCORE_MAX: u64 = 1000000;
pub struct FilloneGreedyGame<'a> {
    pub state: State,
    pub cuts: &'a Cuts,
    pub end: &'a Board,
    pub legal_actions: &'a Vec<Action>,
    pub turn: usize,
}

impl<'a> Game<'a> for FilloneGreedyGame<'a> {
    fn new(board: Board, cuts: &'a Cuts, end: &'a Board, legal_actions: &'a Vec<Action>) -> FilloneGreedyGame<'a> {
        FilloneGreedyGame {
            state: State::new(board),
            cuts,
            end,
            legal_actions,
            turn: 0,
        }
    }

    fn action(&self, state: &State) -> Action {
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

    fn state(&self) -> State {
        self.state.clone()
    }

    fn end(&self) -> &'a Board {
        self.end
    }

    fn step(&mut self) {
        self.turn += 1;
    }

    fn cuts(&self) -> &'a Cuts {
        self.cuts
    }

    fn operate(&mut self, action: &Action, cuts: &'a Cuts) {
        self.state.board.operate(action, cuts);
    }
}
