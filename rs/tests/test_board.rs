use std::iter::{zip, Enumerate};

use rs::board::action::{Action, Direction};
use rs::board::board::Board;
use rs::board::cut::{Cut, Cuts};

#[test]
fn test_op_left() {
    let cut = Cut::new(vec![
        vec![false, true, false],
        vec![true, false, true],
        vec![true, true, false],
    ]);

    let mut start = Board::new(vec![
        vec![1, 0, 1, 1, 2, 2, 1],
        vec![2, 3, 1, 1, 0, 0, 2],
        vec![3, 0, 2, 1, 1, 1, 1],
        vec![3, 0, 0, 2, 2, 3, 1],
        vec![2, 2, 3, 2, 0, 2, 2],
        vec![3, 3, 1, 0, 3, 2, 3],
    ]);

    let end = Board::new(vec![
        vec![1, 0, 1, 1, 2, 2, 1],
        vec![2, 3, 1, 1, 0, 0, 2],
        vec![3, 0, 1, 1, 1, 1, 2],
        vec![3, 0, 2, 3, 1, 0, 2],
        vec![2, 2, 0, 2, 2, 2, 3],
        vec![3, 3, 1, 0, 3, 2, 3],
    ]);

    start.op_left(&cut, 1, 2);
    assert_eq!(start, end);
}

#[test]
fn test_op_left_over() {
    let cut = Cut::new(vec![
        vec![true, true, true],
        vec![true, true, true],
        vec![true, true, true],
    ]);

    let mut start = Board::new(vec![
        vec![1, 0, 1, 1, 2, 2, 1],
        vec![2, 3, 1, 1, 0, 0, 2],
        vec![3, 0, 2, 1, 1, 1, 1],
        vec![3, 0, 0, 2, 2, 3, 1],
        vec![2, 2, 3, 2, 0, 2, 2],
        vec![3, 3, 1, 0, 3, 2, 3],
    ]);

    let end = Board::new(vec![
        vec![1, 1, 2, 2, 1, 1, 0],
        vec![2, 3, 1, 1, 0, 0, 2],
        vec![3, 0, 2, 1, 1, 1, 1],
        vec![3, 0, 0, 2, 2, 3, 1],
        vec![2, 2, 3, 2, 0, 2, 2],
        vec![3, 3, 1, 0, 3, 2, 3],
    ]);

    start.op_left(&cut, -1, -2);

    assert_eq!(start, end);
}

#[test]
fn test_op_right() {
    let cut = Cut::new(vec![
        vec![false, true, false],
        vec![true, false, true],
        vec![true, true, false],
    ]);

    let mut start = Board::new(vec![
        vec![1, 0, 1, 1, 2, 2, 1],
        vec![2, 3, 1, 1, 0, 0, 2],
        vec![3, 0, 2, 1, 1, 1, 1],
        vec![3, 0, 0, 2, 2, 3, 1],
        vec![2, 2, 3, 2, 0, 2, 2],
        vec![3, 3, 1, 0, 3, 2, 3],
    ]);

    let end = Board::new(vec![
        vec![1, 0, 1, 1, 2, 2, 1],
        vec![2, 3, 1, 1, 0, 0, 2],
        vec![2, 3, 0, 1, 1, 1, 1],
        vec![0, 2, 3, 0, 2, 3, 1],
        vec![2, 3, 2, 2, 0, 2, 2],
        vec![3, 3, 1, 0, 3, 2, 3],
    ]);

    start.op_right(&cut, 1, 2);
    assert_eq!(start, end);
}

fn test_get_fillone_action_score(start: Board, end: Board, expected_score: usize) {
    let res = start.get_fillone_action_score(&end);
    println!("res: {}, expected: {}", res, expected_score);
    assert_eq!(res, expected_score)
}

#[test]
fn tests_get_fillone_action_score() {
    // ToDo: issue #26
    let test_cases = vec![
        // (
        //     Board::new(vec![
        //         vec![1, 0, 1, 1, 2, 2, 0, 0],
        //         vec![2, 3, 0, 1, 0, 0, 1, 3],
        //         vec![2, 0, 3, 1, 0, 3, 3, 0],
        //         vec![0, 3, 1, 2, 1, 1, 2, 0],
        //         vec![2, 2, 1, 1, 3, 1, 1, 2],
        //         vec![2, 1, 0, 2, 0, 3, 1, 1],
        //         vec![2, 2, 2, 0, 2, 0, 2, 2],
        //     ]),
        //     Board::new(vec![
        //         vec![3, 1, 2, 0, 3, 2, 1, 0],
        //         vec![2, 0, 3, 2, 1, 2, 0, 0],
        //         vec![1, 1, 0, 0, 1, 0, 1, 3],
        //         vec![2, 1, 2, 1, 2, 2, 3, 0],
        //         vec![3, 2, 2, 0, 1, 1, 3, 2],
        //         vec![1, 2, 0, 2, 2, 2, 1, 0],
        //         vec![0, 1, 2, 1, 0, 3, 1, 0],
        //     ]),
        //     74,
        // ),
        // (
        //     Board::new(vec![
        //         vec![2, 1, 2, 1, 0, 0, 0, 2],
        //         vec![0, 0, 3, 2, 0, 1, 1, 0],
        //         vec![2, 3, 3, 3, 2, 2, 0, 1],
        //         vec![2, 0, 2, 0, 2, 0, 3, 1],
        //         vec![3, 3, 0, 2, 2, 1, 2, 2],
        //         vec![3, 1, 3, 0, 1, 3, 0, 1],
        //         vec![0, 2, 2, 1, 1, 3, 1, 0],
        //     ]),
        //     Board::new(vec![
        //         vec![0, 2, 1, 3, 1, 0, 0, 2],
        //         vec![0, 0, 2, 2, 1, 1, 0, 0],
        //         vec![2, 3, 3, 3, 2, 2, 0, 1],
        //         vec![2, 0, 2, 0, 2, 0, 3, 1],
        //         vec![3, 3, 0, 2, 2, 1, 2, 2],
        //         vec![3, 1, 0, 1, 3, 0, 1, 3],
        //         vec![0, 2, 2, 1, 1, 3, 1, 0],
        //     ]),
        //     19,
        // ),
        // (
        //     Board::new(vec![
        //         vec![0, 2, 3, 3, 0],
        //         vec![1, 2, 0, 0, 0],
        //         vec![3, 0, 1, 3, 3],
        //     ]),
        //     Board::new(vec![
        //         vec![3, 3, 0, 0, 0],
        //         vec![2, 3, 1, 0, 3],
        //         vec![2, 0, 3, 0, 1],
        //     ]),
        //     21,
        // ),
        // (
        //     Board::new(vec![
        //         vec![1, 3, 2, 0, 0],
        //         vec![3, 1, 0, 1, 1],
        //         vec![2, 3, 2, 0, 0],
        //     ]),
        //     Board::new(vec![
        //         vec![2, 3, 0, 3, 1],
        //         vec![0, 2, 0, 1, 3],
        //         vec![0, 1, 0, 2, 1],
        //     ]),
        //     20,
        // ),
        // (
        //     Board::new(vec![vec![
        //         0, 3, 0, 3, 0, 0, 0, 3, 2, 3, 3, 2, 2, 0, 1, 2, 0, 1, 2, 0, 3, 0, 0, 1, 2, 1, 0, 0,
        //         3, 0, 3, 0, 0, 3, 2, 1, 0, 2, 0, 3, 3, 1, 3, 0, 0, 2, 2, 0, 2, 1, 0, 1, 3, 3, 3, 0,
        //         0, 0, 0, 3, 1, 3, 0, 2, 3, 2, 3, 1, 2, 1, 1, 3, 0, 1, 3, 0, 3, 2, 3, 1, 3, 1, 2, 0,
        //         0, 1, 1, 0, 3, 2, 3, 3, 3, 2, 3, 0, 2, 3, 0, 2, 0, 0, 0, 3, 3, 0, 2, 1, 2, 1, 2, 1,
        //         2, 2, 1, 3, 3, 2, 2, 1, 0, 3, 0, 0, 3, 3, 1, 0, 2, 1, 1, 2, 1, 1, 2, 2, 1, 0, 2, 2,
        //         0, 0, 2, 3, 1, 1, 0, 1, 3, 0, 2, 1, 3, 0, 1, 0, 3, 0, 2, 3, 1, 0, 1, 3, 2, 3, 1, 1,
        //         0, 0, 2, 2, 0, 1, 1, 0, 1, 3, 0, 1, 3, 3, 3, 3, 1, 3, 0, 1, 1, 0, 0, 0, 1, 0, 2, 3,
        //         2, 3, 3, 1, 0, 3, 1, 0, 3, 1, 2, 3, 3, 1, 0, 2, 2, 3, 0, 3, 2, 0, 2, 2, 2, 0, 1, 1,
        //         0, 1, 1, 1, 2, 2, 3, 3, 0, 0, 1, 1, 3, 3, 2, 2, 2, 3, 1, 1, 3, 1, 0, 2, 0, 3, 3, 0,
        //         2, 2, 0, 0,
        //     ]]),
        //     Board::new(vec![vec![
        //         3, 0, 3, 0, 0, 0, 3, 2, 3, 3, 2, 2, 0, 1, 2, 0, 1, 2, 0, 3, 0, 0, 1, 2, 1, 0, 0, 3,
        //         0, 3, 0, 0, 3, 2, 1, 0, 2, 0, 3, 3, 1, 3, 0, 0, 2, 2, 0, 2, 1, 0, 1, 3, 3, 3, 0, 0,
        //         0, 0, 3, 1, 3, 0, 2, 3, 2, 3, 1, 2, 1, 1, 3, 0, 1, 3, 0, 3, 2, 3, 1, 3, 1, 2, 0, 0,
        //         1, 1, 0, 3, 2, 3, 3, 3, 2, 3, 0, 2, 3, 0, 2, 0, 0, 0, 3, 3, 0, 2, 1, 2, 1, 2, 1, 2,
        //         2, 1, 3, 3, 2, 2, 1, 0, 3, 0, 0, 3, 3, 1, 0, 2, 1, 1, 2, 1, 1, 2, 2, 1, 0, 2, 2, 0,
        //         0, 2, 3, 1, 1, 0, 1, 3, 0, 2, 1, 3, 0, 1, 0, 3, 0, 2, 3, 1, 0, 1, 3, 2, 3, 1, 1, 0,
        //         0, 2, 2, 0, 1, 1, 0, 1, 3, 0, 1, 3, 3, 3, 3, 1, 3, 0, 1, 1, 0, 0, 0, 1, 0, 2, 3, 2,
        //         3, 3, 1, 0, 3, 1, 0, 3, 1, 2, 3, 3, 1, 0, 2, 2, 3, 0, 3, 2, 0, 2, 2, 2, 0, 1, 1, 0,
        //         1, 1, 1, 2, 2, 3, 3, 0, 0, 1, 1, 3, 3, 2, 2, 2, 3, 1, 1, 3, 1, 0, 2, 0, 3, 3, 0, 2,
        //         2, 0, 0, 0,
        //     ]]),
        //     134,
        // ),
        // (Board::new(vec![vec![2], vec![3], vec![3], vec![0], vec![2], vec![1], vec![2], vec![3], vec![3], vec![3], vec![1], vec![0], vec![3], vec![2], vec![3], vec![0], vec![2], vec![3], vec![2], vec![0], vec![1], vec![1], vec![0], vec![1], vec![2], vec![0], vec![2], vec![2], vec![2], vec![2], vec![2], vec![3], vec![2], vec![2], vec![3], vec![1], vec![2], vec![3], vec![3], vec![0], vec![2], vec![0], vec![2], vec![2], vec![0], vec![3], vec![2], vec![3], vec![3], vec![1], vec![3], vec![2], vec![1], vec![1], vec![2], vec![2], vec![2], vec![3], vec![2], vec![2], vec![2], vec![3], vec![1], vec![2],]), Board::new(vec![vec![3], vec![3], vec![0], vec![1], vec![2], vec![0], vec![1], vec![0], vec![3], vec![2], vec![2], vec![1], vec![2], vec![3], vec![3], vec![3], vec![0], vec![2], vec![2], vec![3], vec![3], vec![3], vec![2], vec![2], vec![2], vec![2], vec![2], vec![2], vec![1], vec![3], vec![2], vec![2], vec![2], vec![1], vec![0], vec![1], vec![3], vec![3], vec![3], vec![3], vec![1], vec![0], vec![2], vec![0], vec![2], vec![2], vec![3], vec![2], vec![3], vec![1], vec![1], vec![3], vec![1], vec![2], vec![2], vec![0], vec![2], vec![2], vec![3], vec![0], vec![2], vec![2], vec![2], vec![2],]), 174),

        // (Board::new(vec![vec![0], vec![0], vec![1], vec![0], vec![0], vec![1], vec![3], vec![0], vec![0], vec![1], vec![3], vec![1], vec![3], vec![1], vec![2], vec![0], vec![2], vec![3], vec![0], vec![3], vec![1], vec![2], vec![3], vec![2], vec![3], vec![0], vec![3], vec![3], vec![0], vec![2], vec![1], vec![1],]), Board::new(vec![vec![1], vec![2], vec![0], vec![2], vec![3], vec![0], vec![3], vec![1], vec![2], vec![3], vec![2], vec![3], vec![0], vec![3], vec![3], vec![0], vec![2], vec![1], vec![1], vec![0], vec![0], vec![1], vec![0], vec![0], vec![1], vec![3], vec![0], vec![0], vec![1], vec![3], vec![1], vec![3],]), 82),
    ];

    for (start, end, expected_score) in test_cases {
        test_get_fillone_action_score(start, end, expected_score)
    }
}

#[test]
fn test_get_formal_cut() {
    let cut_5 = Cut::new(vec![
        vec![true, true, true, true],
        vec![false, false, false, false],
        vec![true, true, true, true],
        vec![false, false, false, false],
    ]);
    let path = "../data/formal_cuts.json".to_string();
    let cuts = Cuts::new(path);

    for h in 0..cuts[5].height() {
        for w in 0..cuts[5].width() {
            assert_eq!(cuts[5][h][w], cut_5[h][w]);
        }
    }
}

#[test]
fn test_formal_cut_operate() {
    let mut start = Board::new(vec![
        vec![1, 0, 1, 1, 2, 2, 1],
        vec![2, 3, 1, 1, 0, 0, 2],
        vec![3, 0, 2, 1, 1, 1, 1],
        vec![3, 0, 0, 2, 2, 3, 1],
        vec![2, 2, 3, 2, 0, 2, 2],
        vec![3, 3, 1, 0, 3, 2, 3],
    ]);

    let cuts = Cuts::new("../data/formal_cuts.json".to_string());
    let action = Action::new(0, 4, 1, Direction::Left);

    let end = Board::new(vec![
        vec![1, 0, 1, 1, 2, 2, 1],
        vec![2, 3, 1, 1, 0, 0, 2],
        vec![3, 0, 2, 1, 1, 1, 1],
        vec![3, 0, 0, 2, 2, 3, 1],
        vec![3, 2, 0, 2, 2, 2, 2],
        vec![1, 0, 3, 2, 3, 3, 3],
    ]);

    start.operate(&action, &cuts);
    assert_eq!(start, end);
}

#[test]
fn test_swapping() {
    let cuts = Cuts::new("../data/formal_cuts.json".to_string());

    let test_cases = vec![
        (
            // 横一列
            vec![vec![1, 1, 3, 0, 0, 2, 1, 1, 1]],
            2,
            0,
            5,
            0,
        ),
        (
            // 縦一列
            vec![
                vec![1],
                vec![1],
                vec![3],
                vec![0],
                vec![0],
                vec![2],
                vec![1],
                vec![1],
                vec![1],
            ],
            0,
            2,
            0,
            5,
        ),
        (
            vec![
                vec![1, 1, 0, 4, 4, 0, 5, 5, 5],
                vec![0, 0, 3, 0, 0, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 2, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0, 0, 0],
            ],
            2,
            1,
            5,
            2,
        ),
    ];
    for (board, x1, y1, x2, y2) in test_cases {
        let mut board = Board::new(board);
        let mut new = board.clone();
        let actions = board.swapping(x1, y1, x2, y2);
        new.operate_actions(actions, &cuts);
        assert_eq!(board.board(), new.board());
    }
}
