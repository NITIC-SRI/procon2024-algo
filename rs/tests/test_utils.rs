use rs::board::action::{Action, Direction};
use rs::board::board::Board;
use rs::board::cut::Cuts;
use rs::utils::{export_actions, get_actions, validate_actions};

use std::vec;

#[test]
fn test_export_actions() {
    let actions = vec![
        Action::new(1, 1, 1, Direction::Up),
        Action::new(2, 2, 2, Direction::Down),
        Action::new(3, 3, 3, Direction::Right),
        Action::new(4, 4, 4, Direction::Left),
    ];

    let json = export_actions(actions);
    let collect = r#"{"n":4,"ops":[{"p":1,"x":1,"y":1,"s":"0"},{"p":2,"x":2,"y":2,"s":"1"},{"p":3,"x":3,"y":3,"s":"2"},{"p":4,"x":4,"y":4,"s":"3"}]}"#.to_string();
    assert_eq!(json, collect);
}

#[test]
fn test_random_board() {
    let board = Board::new(rs::utils::random_board(10, 10));
    let shuffled_board = Board::new(rs::utils::shuffle_board(board.clone().board, 42));

    println!("{}", board);
    println!("{}", shuffled_board);
}

#[test]
fn test_get_actions() {
    let cuts = Cuts::new("../data/formal_cuts.json".to_string());
    let actions = get_actions(16, 16, &cuts);
    // magic number
    assert_eq!(14072, actions.len());
}

// #[test]
// fn test_read_actions() {
//     let actions = rs::utils::read_actions("output.txt".to_string());
//     let mut sized_actions = vec![];
//     for action in actions {
//         if action.x() > 31 || action.y() > 31 || action.cut_num() > 15 {
//             continue;
//         }
//         sized_actions.push(action);
//     }
//     println!("{}", sized_actions.len());
// }

#[test]
fn test_read_actions_by_size() {
    let testcases = vec![
        (34, 34, "64*64"),
        // (70, 80, "128*128"),
        // (127, 129, "256*256"),
        // (256, 256, "256*256"),
    ];

    for testcase in testcases {
        let actions =
            rs::utils::read_actions(format!("../data/compress_actions/{}.json", testcase.2));
        let mut sized_actions_len = 0;
        for action in actions.iter() {
            if action.x() >= testcase.0 || action.y() >= testcase.1 {
                continue;
            }
            sized_actions_len += 1;
        }

        assert_eq!(
            sized_actions_len,
            rs::utils::read_actions_by_size(testcase.0 as usize, testcase.1 as usize).len()
        );
    }
}

#[test]
fn test_validate_actions() {
    let board = Board::new(rs::utils::random_board(10, 10));
    let end = Board::new(rs::utils::shuffle_board(board.clone().board, 42));
    let cuts = Cuts::new("../data/formal_cuts.json".to_string());

    let actions = board.get_fillone_actions(&end, 0, 0, true);
    assert!(validate_actions(&board, &end, &actions, &cuts));
}
