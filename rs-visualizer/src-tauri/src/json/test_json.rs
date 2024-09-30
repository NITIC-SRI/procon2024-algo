use std::vec;

use super::json;
use super::json::{Action, JsonAction};

#[test]
fn test_get_json() {
    // 現在のディレクトリを取得
    let path = String::from("submit.json");
    let data = json::get_action_from_json(path);
    let expected_data = JsonAction {
        start: vec![
            vec![1, 1, 2, 1, 0],
            vec![3, 0, 1, 2, 1],
            vec![3, 0, 0, 2, 2],
            vec![3, 0, 2, 1, 1],
            vec![2, 2, 3, 2, 0],
        ],
        end: vec![
            vec![2, 1, 0, 1, 1],
            vec![0, 2, 1, 3, 1],
            vec![3, 0, 0, 2, 2],
            vec![3, 0, 2, 1, 1],
            vec![2, 2, 3, 2, 0],
        ],
        actions: vec![
            Action {
                x: 1,
                y: 0,
                p: 0,
                s: "0".to_string(),
            },
            Action {
                x: -2,
                y: 3,
                p: 4,
                s: "1".to_string(),
            },
            Action {
                x: 0,
                y: 0,
                p: 5,
                s: "2".to_string(),
            },
            Action {
                x: 3,
                y: 3,
                p: 2,
                s: "3".to_string(),
            },
        ],
    };

    assert_eq!(data, expected_data);
}

#[test]
fn test_get_cut() {
    let path = String::from("../../data/formal_cuts.json");
    let data = json::get_formal_cut(path);

    let cut5 = vec![
        vec![1, 1, 1, 1],
        vec![0, 0, 0, 0],
        vec![1, 1, 1, 1],
        vec![0, 0, 0, 0],
    ];

    assert_eq!(data[5], cut5);
}

#[test]
fn test_get_action() {
	let formal_cut_path = String::from("../../data/formal_cuts.json");
    let path = String::from("submit.json");
    let (start, end, actions) = json::get_actions(path, formal_cut_path);
	let expected_start = vec![
		vec![1, 1, 2, 1, 0],
		vec![3, 0, 1, 2, 1],
		vec![3, 0, 0, 2, 2],
		vec![3, 0, 2, 1, 1],
		vec![2, 2, 3, 2, 0],
	];
	let expected_end = vec![
		vec![2, 1, 0, 1, 1],
		vec![0, 2, 1, 3, 1],
		vec![3, 0, 0, 2, 2],
		vec![3, 0, 2, 1, 1],
		vec![2, 2, 3, 2, 0],
	];
	let expected_actions = vec![
		json::ActionJS {
			x: 1,
			y: 0,
			cut: vec![
				vec![1]
			],
			direction: "top".to_string(),
		},
		json::ActionJS {
			x: -2,
			y: 3,
			cut: vec![
				vec![1, 1, 1, 1],
				vec![1, 1, 1, 1],
				vec![1, 1, 1, 1],
				vec![1, 1, 1, 1],
			],
			direction: "bottom".to_string(),
		},
		json::ActionJS {
			x: 0,
			y: 0,
			cut: vec![
				vec![1, 1, 1, 1],
				vec![0, 0, 0, 0],
				vec![1, 1, 1, 1],
				vec![0, 0, 0, 0],
			],
			direction: "right".to_string(),
		},
		json::ActionJS {
			x: 3,
			y: 3,
			cut: vec![
				vec![1, 1],
				vec![0, 0]
			],
			direction: "left".to_string(),
		},
	];

	assert_eq!(start, expected_start);
	assert_eq!(end, expected_end);
	assert_eq!(actions, expected_actions);
}
