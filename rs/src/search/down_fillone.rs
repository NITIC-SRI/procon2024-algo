use super::game::{Game, State};
use crate::board;
use crate::board::action::{Action, Direction};
use crate::board::board::Board;
use crate::board::cut::Cuts;



pub fn play<'a>(start: &Board, end: &Board, legal_actions: &Vec<Action>, count: usize, log: bool) -> Vec<Action> {
	let mut down_only_actions = get_action_by_direction(&legal_actions, Direction::Down);
	// leftとrightを取得して結合する
	let mut x_only_actions = get_action_by_direction(&legal_actions, Direction::Left);
	x_only_actions.append(&mut get_action_by_direction(&legal_actions, Direction::Right));

    let mut actions = Vec::new();

    // TODO: タイムキーパーを設定する
    for i in 0..count {
        // 上に寄せられるだけ寄せる
		let now_board = start.clone();

    }
    actions
}


pub fn get_action_by_direction(legal_actions: &Vec<Action>, direction: Direction) -> Vec<Action> {
	let mut down_only = Vec::new();
	for action in legal_actions {
		if action.direction() == direction {
			down_only.push(action.clone());
		}
	}
	down_only
}

pub fn down_greedy_action(now_board: &Board, end: &Board, down_only_legal_actions: &Vec<Action>, cuts: &Cuts) -> Action {
    let mut min_distance = 100000;
    let mut min_action = down_only_legal_actions[0].clone();
    
    for action in down_only_legal_actions {
        let next_board = now_board.clone().operate(action, cuts);
        let distance = next_board.top_priority_distance(&end);
        if distance < min_distance {
            min_distance = distance;
            min_action = action.clone();
        }
    }
    min_action
}