use crate::board::action::Action;
use crate::board::board::Board;
use crate::board::cut::Cuts;
use crate::board::action::Direction;

pub const SCORE_MAX: usize = 1000000000;
pub struct GreedyGame<'a> {
    pub state: GreedyState<'a>,
    pub cuts: &'a Cuts,
    pub end: &'a Board,
    pub actions: Vec<&'a Action>,
}
pub struct GreedyState<'a> {
    pub board: &'a mut Board,
}

impl<'a> GreedyState<'a> {
    pub fn new(board: &'a mut Board) -> GreedyState<'a> {
        GreedyState { board }
    }

    pub fn evaluate_score(&self, end: &Board) -> u64 {
        self.board.absolute_distance(end)
    }
}

impl<'a> GreedyGame<'a> {
    pub fn new(
        board: &'a mut Board,
        cuts: &'a Cuts,
        end: &'a Board,
        actions: Vec<&'a Action>,
    ) -> GreedyGame<'a> {
        GreedyGame {
            state: GreedyState::new(board),
            cuts,
            end,
            actions,
        }
    }

    pub fn greedy_action(&self) -> Action {
        let mut min_score = SCORE_MAX;
        let mut min_action = Action::new(0, 0, 0, Direction::Up);

        for &action in self.actions.iter() {
            let mut board = self.state.board.clone();
            board.operate(action, &self.cuts);
            let score = board.get_fillone_action_score(&self.end);
            // println!("score: {}", score);
            // println!(
            //     "action: x={} y={} cut={} direction={:?}",
            //     action.x(),
            //     action.y(),
            //     action.cut_num(),
            //     action.direction()
            // );
            // println!("------------------------------------");
            if score < min_score {
                min_score = score;
                min_action = *action;
            }
        }

        // assert!(max_action != Action::new(0, 0, 0, Direction::Up));
        min_action
    }
}

pub fn play(game: &mut GreedyGame) -> Vec<Action> {
    let cuts = game.cuts;
	let mut actions = vec![];

    // TODO: タイムキーパーを設定する
    for i in 0..100 {
        println!("i: {}", i);

        let now_board = game.state.board.clone();
        let action = game.greedy_action();
        game.state.board.operate(&action, cuts);
        actions.push(action.clone());
        println!("action: {:?}", action);
        println!("{}", game.state.board);
        if game.state.board == game.end {
            break;
        }
        if *game.state.board == now_board {
            break;
        }

        // println!(
        //     "action: x={} y={} cut={} direction={:?}",
        //     action.x(),
        //     action.y(),
        //     action.cut_num(),
        //     action.direction()
        // );
        // println!("score: {}", game.state.evaluate_score(&game.end));
        // println!("board: {}", game.state.board);
        // println!("----------------------------------");
    }
    actions
}
