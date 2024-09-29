use rs::board::board::Board;
use rs::board::cut::Cut;

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
    assert_eq!(start.board(), end.board());
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

    assert_eq!(start.board, end.board);
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
    assert_eq!(start.board(), end.board());
}

fn test_get_fillone_action_score(start: Board, end: Board, expected_score: usize) {
    let res = start.get_fillone_action_score(&end);
    println!("res: {}, expected: {}", res, expected_score);
    assert_eq!(res, expected_score)
}

#[test]
fn test_op_down() {
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
        vec![1, 0, 2, 2, 2, 2, 1],
        vec![2, 2, 3, 1, 0, 0, 2],
        vec![3, 0, 1, 1, 1, 1, 1],
        vec![3, 3, 1, 1, 2, 3, 1],
        vec![2, 0, 0, 2, 0, 2, 2],
        vec![3, 3, 1, 0, 3, 2, 3],
    ]);

    start.op_down(&cut, 1, 2);
    assert_eq!(start.board(), end.board());
  }

fn tests_get_fillone_action_score() {
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
        (Board::new(vec![vec![2], vec![3], vec![3], vec![0], vec![2], vec![1], vec![2], vec![3], vec![3], vec![3], vec![1], vec![0], vec![3], vec![2], vec![3], vec![0], vec![2], vec![3], vec![2], vec![0], vec![1], vec![1], vec![0], vec![1], vec![2], vec![0], vec![2], vec![2], vec![2], vec![2], vec![2], vec![3], vec![2], vec![2], vec![3], vec![1], vec![2], vec![3], vec![3], vec![0], vec![2], vec![0], vec![2], vec![2], vec![0], vec![3], vec![2], vec![3], vec![3], vec![1], vec![3], vec![2], vec![1], vec![1], vec![2], vec![2], vec![2], vec![3], vec![2], vec![2], vec![2], vec![3], vec![1], vec![2],]), Board::new(vec![vec![3], vec![3], vec![0], vec![1], vec![2], vec![0], vec![1], vec![0], vec![3], vec![2], vec![2], vec![1], vec![2], vec![3], vec![3], vec![3], vec![0], vec![2], vec![2], vec![3], vec![3], vec![3], vec![2], vec![2], vec![2], vec![2], vec![2], vec![2], vec![1], vec![3], vec![2], vec![2], vec![2], vec![1], vec![0], vec![1], vec![3], vec![3], vec![3], vec![3], vec![1], vec![0], vec![2], vec![0], vec![2], vec![2], vec![3], vec![2], vec![3], vec![1], vec![1], vec![3], vec![1], vec![2], vec![2], vec![0], vec![2], vec![2], vec![3], vec![0], vec![2], vec![2], vec![2], vec![2],]), 174), 

        // (Board::new(vec![vec![0], vec![0], vec![1], vec![0], vec![0], vec![1], vec![3], vec![0], vec![0], vec![1], vec![3], vec![1], vec![3], vec![1], vec![2], vec![0], vec![2], vec![3], vec![0], vec![3], vec![1], vec![2], vec![3], vec![2], vec![3], vec![0], vec![3], vec![3], vec![0], vec![2], vec![1], vec![1],]), Board::new(vec![vec![1], vec![2], vec![0], vec![2], vec![3], vec![0], vec![3], vec![1], vec![2], vec![3], vec![2], vec![3], vec![0], vec![3], vec![3], vec![0], vec![2], vec![1], vec![1], vec![0], vec![0], vec![1], vec![0], vec![0], vec![1], vec![3], vec![0], vec![0], vec![1], vec![3], vec![1], vec![3],]), 82), 
        
    
    ];

    for (start, end, expected_score) in test_cases {
        test_get_fillone_action_score(start, end, expected_score)
    }
}
