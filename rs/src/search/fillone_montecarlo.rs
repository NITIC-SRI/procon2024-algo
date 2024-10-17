use super::game::{Game, State};
use crate::board::action::{Action, Direction};
use crate::board::board::Board;
use crate::board::cut::Cuts;
use rand::seq::SliceRandom;
use rand::SeedableRng;
use rand::rngs::SmallRng;


pub const N_SIMULATIONS: usize = 10000;
pub const SCORE_MAX: u64 = 0;
pub struct MontecarloGame<'a> {
    pub state: State,
    pub cuts: &'a Cuts,
    pub end: &'a Board,
    pub legal_actions: &'a Vec<Action>,
    pub turn: usize,
}

impl<'a> Game<'a> for MontecarloGame<'a> {
    fn new(
        board: Board,
        cuts: &'a Cuts,
        end: &'a Board,
        legal_actions: &'a Vec<Action>,
    ) -> MontecarloGame<'a> {
        MontecarloGame {
            state: State::new(board),
            cuts,
            end,
            legal_actions,
            turn: 0,
        }
    }

    fn action(&self, state: &State) -> Action {
        let mut min_score = state.board.get_fillone_action_score(&self.end);
        let mut min_action = Action::new(0, 0, 0, Direction::Up);

        let mut rng = SmallRng::from_entropy();
        let random_legal_actions: Vec<Action> = self.legal_actions.choose_multiple(&mut rng, N_SIMULATIONS).cloned().collect();
        for action in random_legal_actions {
            let mut board = state.board.clone();
            board.operate(&action, &self.cuts);
            let score = board.get_fillone_action_score(&self.end);
            if score < min_score {
                min_score = score;
                min_action = action.clone();
            }
        }

        println!("turn: {}, min_score: {}", self.turn, min_score);
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
