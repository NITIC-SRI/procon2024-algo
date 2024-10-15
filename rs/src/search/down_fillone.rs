use crate::board::action::{Action, Direction};
use crate::board::board::Board;
use crate::board::cut::Cuts;

pub fn play<'a>(
    start: &Board,
    end: &Board,
    legal_actions: &Vec<Action>,
    cuts: &Cuts,
) -> Vec<Action> {
    let mut cuts = cuts.clone();
    cuts.delete_only_zero_bottoms();
    let (down_only_actions, x_only_actions) = get_action_by_direction(&legal_actions);

    let mut actions = Vec::new();
    let mut now_board = start.clone();
    let mut usable_height = start.height();

    // TODO: タイムキーパー
    for i in 0..100 {
        println!("{}", now_board);
        println!("height: {}", usable_height);

        // 一番上の行に寄せられるだけ寄せる
        let mut prev_distance = std::u64::MAX;
        let mut min_diff = vec![];
        loop {
            let (action, distance, diff) =
                down_greedy_action(&now_board, &end, &down_only_actions, &cuts, usable_height);
            if prev_distance <= distance {
                break;
            }
            now_board.operate(&action, &cuts);

            prev_distance = distance;
            actions.push(action);
            min_diff = diff;
            if distance == 0 {
                break;
            }
        }

        // 一番上の行で揃えられないものが存在するなら横に篩う
        if !min_diff.is_empty() {
            let action = greedy_match_x_direction_action(
                &now_board,
                &end,
                &x_only_actions,
                &cuts,
                &min_diff,
                usable_height,
            );
            now_board.operate(&action, &cuts);
            actions.push(action);
            continue;
        }

        now_board.op_row_up();
        usable_height -= 1;
        actions.push(Action::new(0, -255, 22, Direction::Up));
        if now_board == *end || usable_height == 0{
            break;
        }
    }

    actions
}

pub fn greedy_match_x_direction_action(
    now_board: &Board,
    end: &Board,
    x_only_legal_actions: &Vec<Action>,
    cuts: &Cuts,
    diff: &Vec<usize>,
    usable_height: usize,
) -> Action {
    let mut min_distance: u64 = std::u64::MAX;
    let mut min_action = Action::new(0, 0, 0, Direction::Down);

    for action in x_only_legal_actions {
        let mut next_board = now_board.clone();

        if action.y() + cuts[action.cut_num() as u32].height() as i32 > usable_height as i32 {
            continue;
        }

        next_board.operate(action, cuts);
        let distance = next_board.match_x_direction_score(&end, &diff, usable_height);
        if min_distance > distance {
            min_distance = distance;
            min_action = action.clone();
        }
    }

    min_action
}

pub fn down_greedy_action(
    now_board: &Board,
    end: &Board,
    down_only_legal_actions: &Vec<Action>,
    cuts: &Cuts,
    usable_height: usize,
) -> (Action, u64, Vec<usize>) {
    let mut min_distance: u64 = std::u64::MAX;
    let mut min_action = Action::new(0, 0, 0, Direction::Down);
    let mut min_diff = vec![std::usize::MAX];

    for action in down_only_legal_actions {
        let mut next_board = now_board.clone();
        if action.y() + cuts[action.cut_num() as u32].height() as i32 > usable_height as i32 {
            continue;
        }
        next_board.operate(action, cuts);
        let (distance, diff) = next_board.top_first_distance(&end, usable_height);

        if distance < min_distance {
            min_distance = distance;
            min_action = action.clone();
            min_diff = diff;
        }
    }

    (min_action, min_distance, min_diff)
}

pub fn get_action_by_direction(legal_actions: &Vec<Action>) -> (Vec<Action>, Vec<Action>) {
    let mut down_only = Vec::new();
    let mut x_only = Vec::new();
    for action in legal_actions {
        if action.y() < 1 {
            continue;
        }
        if action.direction() == Direction::Down {
            down_only.push(action.clone());
        } else if action.direction() == Direction::Left || action.direction() == Direction::Right {
            x_only.push(action.clone());
        }
    }
    (down_only, x_only)
}
