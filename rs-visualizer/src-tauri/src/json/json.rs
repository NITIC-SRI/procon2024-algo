use serde::{Deserialize, Serialize};
use serde_json;
use std::fs::File;
use tauri::api::dir;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Action {
    pub x: i32,
    pub y: i32,
    pub p: i32,
    pub s: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct JsonAction {
    pub start: Vec<Vec<i32>>,
    pub end: Vec<Vec<i32>>,
    pub actions: Vec<Action>,
}

pub fn get_action_from_json(path: String) -> JsonAction {
    let file = File::open(path).unwrap();
    let data = serde_json::from_reader(file).unwrap();
    data
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ActionJS {
    pub x: i32,
    pub y: i32,
    pub cut: Vec<Vec<i32>>,
    pub direction: String,
}

pub fn get_actions(
    path: String,
    formal_cut_path: String,
) -> (Vec<Vec<i32>>, Vec<Vec<i32>>, Vec<ActionJS>) {
    let data = get_action_from_json(path);
    let formal_cut = get_formal_cut(formal_cut_path);
    let mut actions = vec![];
    for i in 0..(data.actions.len()) {
        let direction = match data.actions[i].s.as_str() {
            "0" => "top",
            "1" => "bottom",
            "2" => "right",
            "3" => "left",
            _ => "error",
        };
        let mut action = ActionJS {
            x: data.actions[i].x,
            y: data.actions[i].y,
            cut: vec![],
            direction: direction.to_string(),
        };

        let idx = data.actions[i].p as usize;
        action.cut = formal_cut[idx].clone();
        actions.push(action);
    }

    (data.start, data.end, actions)
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct JsonCut {
    formal: Vec<Cells>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Cells {
    p: u32,
    width: u32,
    height: u32,
    cells: Vec<String>,
}

pub fn get_formal_cut(path: String) -> Vec<Vec<Vec<i32>>> {
    let file = File::open(path).unwrap();
    let data: JsonCut = serde_json::from_reader(file).unwrap();

    let mut cuts = vec![];
    for idx in 0..(data.formal.len()) {
        let mut cut = vec![];
        for y in 0..(data.formal[idx].cells.len()) {
            let mut cell = vec![];
            for x in 0..(data.formal[idx].cells[0].len()) {
                if data.formal[idx].cells[y].chars().nth(x).unwrap() == '0' {
                    cell.push(0);
                } else {
                    cell.push(1);
                }
            }
            cut.push(cell);
        }
        cuts.push(cut);
    }

    cuts
}
