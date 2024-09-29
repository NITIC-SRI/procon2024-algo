use crate::board::board::Board;
use crate::board::cut::Cuts;

pub const SCORE_MAX: u64 = 1000000000;
pub struct Game {
    pub state: State,
    pub cuts: Cuts,
    pub end: Board,
}
pub struct State {
    pub board: Board,
}

impl State {
    pub fn new(board: Board) -> State {
        State { board }
    }

    pub fn evaluate_score(&self, end: &Board) -> u64 {
        self.board.absolute_distance(end)
    }
}
