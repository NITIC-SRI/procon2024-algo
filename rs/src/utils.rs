use crate::board::action::{Action, Direction};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct ActionFormat {
    p: u16,
    x: i32,
    y: i32,
    s: String,
}

#[derive(Serialize, Deserialize)]
struct ActionsFormat {
	n: usize,
    ops: Vec<ActionFormat>,
}

pub fn export_actions(actions: Vec<Action>) -> String {
    let mut actions_format = ActionsFormat {
		n: actions.len(),
        ops: Vec::new(),
    };
    for action in actions {
        let action_format = ActionFormat {
            p: action.cut_num(),
            x: action.x(),
            y: action.y(),
            s: match action.direction() {
				Direction::Up => "0".to_string(),
				Direction::Down => "1".to_string(),
				Direction::Right => "2".to_string(),
				Direction::Left => "3".to_string(),
            },
        };
        actions_format.ops.push(action_format);
    }
    let json = serde_json::to_string(&actions_format).unwrap();
	json
}
