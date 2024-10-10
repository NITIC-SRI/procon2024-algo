use crate::board::action::{Action, Direction};
use crate::board::board::Board;
use crate::board::cut::Cuts;

#[derive(Clone)]
pub struct State {
    pub board: Board,
    pub score: u64,
    pub first_action: Action,
}

impl State {
    pub fn new(board: Board) -> State {
        State {
            board,
            score: 0,
            first_action: Action::new(0, 0, 0, Direction::Up),
        }
    }
}

pub trait Game<'a> {
    fn new(board: Board, cuts: &'a Cuts, end: &'a Board, legal_actions: &'a Vec<Action>) -> Self;
    fn action(&self, state: &State) -> Action;
    fn state(&self) -> State;
    fn end(&self) -> &'a Board;
    fn step(&mut self);
    fn cuts(&self) -> &'a Cuts;
    fn operate(&mut self, action: &Action, cuts: &'a Cuts);
}

pub fn play<'a, T>(game: &mut T, count: usize, log: bool) -> Vec<Action>
where
    T: Game<'a>,
{
    let mut actions = Vec::new();

    // TODO: タイムキーパーを設定する
    for i in 0..count {
        let now_board = game.state().board.clone();
        let action = game.action(&game.state());
        game.operate(&action, game.cuts());

        if i > 0 && actions[actions.len() - 1] == action {
            break;
        }
        actions.push(action.clone());

        if log {
            println!("i: {}", i);
            println!("action: {:?}", action);
            println!("{}", game.state().board);
            println!("score: {}", game.state().board.absolute_distance(&game.end()));
        }
        if game.state().board == *game.end() {
            break;
        }
        if game.state().board == now_board {
            println!("same board");
            break;
        }
        game.step();
    }
    actions
}
