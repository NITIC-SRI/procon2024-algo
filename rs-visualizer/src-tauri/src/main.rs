// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use serde::{Deserialize, Serialize};

mod json;
// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ReturnData {
    pub start: Vec<Vec<i32>>,
    pub end: Vec<Vec<i32>>,
    pub actions: Vec<json::json::ActionJS>,
}

#[tauri::command]
fn get_data() -> ReturnData {
    let formal_cut_path = String::from("../../data/formal_cuts.json");
    let path = String::from("submit.json");
    let (start, end, actions) = json::json::get_actions(path, formal_cut_path);
    let ret = ReturnData {
        start: start,
        end: end,
        actions: actions,
    };
    ret
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet])
        .invoke_handler(tauri::generate_handler![get_data])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
