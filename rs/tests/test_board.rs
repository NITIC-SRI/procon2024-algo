use std::vec;

use rs::board::action::{Action, Direction};
use rs::board::board::Board;
use rs::board::cut::{Cut, Cuts};
use rs::utils::{random_board, shuffle_board};

use rand::rngs::StdRng;
use rand::{self, Rng, SeedableRng};

#[test]
fn test_op_left() {
    let cut = Cut::new(vec![
        vec![false, true, false],
        vec![true, false, true],
        vec![true, true, false],
    ]);

    let mut start: Board<u8> = Board::new(vec![
        vec![1, 0, 1, 1, 2, 2, 1],
        vec![2, 3, 1, 1, 0, 0, 2],
        vec![3, 0, 2, 1, 1, 1, 1],
        vec![3, 0, 0, 2, 2, 3, 1],
        vec![2, 2, 3, 2, 0, 2, 2],
        vec![3, 3, 1, 0, 3, 2, 3],
    ]);

    let end: Board<u8> = Board::new(vec![
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

    let mut start: Board<u8> = Board::new(vec![
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

    let mut start: Board<u8> = Board::new(vec![
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

#[test]
fn test_check_progress() {
    let test_cases: Vec<(Board<u8>, Board<u8>, usize, usize)> = vec![
        (
            Board::new(vec![
                vec![4, 5, 1, 2, 3],
                vec![6, 7, 8, 9, 0],
                vec![0, 0, 0, 0, 0],
                vec![1, 1, 1, 1, 1],
            ]),
            Board::new(vec![
                vec![0, 0, 0, 0, 0],
                vec![1, 1, 1, 1, 1],
                vec![1, 2, 3, 4, 5],
                vec![6, 7, 8, 9, 0],
            ]),
            2,
            3,
        ),
        (
            Board::new(vec![
                vec![2, 2, 2, 2, 2],
                vec![3, 3, 3, 3, 3],
                vec![0, 0, 0, 0, 0],
                vec![1, 1, 1, 1, 1],
            ]),
            Board::new(vec![
                vec![0, 0, 0, 0, 0],
                vec![1, 1, 1, 1, 1],
                vec![2, 2, 2, 2, 2],
                vec![3, 3, 3, 3, 3],
            ]),
            2,
            5,
        ),
    ];

    for (start, end, row_num, col_num) in test_cases {
        let res = start.check_progress(&end);
        assert_eq!(res, (row_num, col_num));
    }
}

fn test_get_fillone_action_score(start: Board, end: Board) {
    let res = start.get_fillone_action_score(&end);
    let actions: Vec<Action>;

    let expected_score: usize = {
        actions = start.get_fillone_actions(&end, 0, 0, true);
        actions.len()
    };
    assert_eq!(
        res, expected_score,
        "res: {}, expected: {}\n start: {:?}, end: {:?}",
        res, expected_score, start, end
    );
}

#[test]
fn tests_get_fillone_action_score() {
    // ToDo: issue #26
    let test_cases = vec![
        (
            Board::new(vec![vec![2], vec![1], vec![1], vec![2], vec![2]]),
            Board::new(vec![vec![2], vec![2], vec![2], vec![1], vec![1]]),
        ),
        (
            Board::new(vec![vec![0, 3, 0, 3]]),
            Board::new(vec![vec![3, 0, 3, 0]]),
        ),
    ];
    // let mut i = 0;
    for (start, end) in test_cases {
        // println!("test case: {}", i);
        // i += 1;
        test_get_fillone_action_score(start, end)
    }

    let mut rng = StdRng::seed_from_u64(42);
    for _ in 0..1 {
        let h = rng.gen_range(256..=256);
        let w = rng.gen_range(256..=256);

        let start = Board::new(random_board(h, w));
        let end = shuffle_board(start.clone().board, 42);

        // println!("random test case: h: {}, w: {}", h, w);
        test_get_fillone_action_score(start, Board::new(end));
    }
}

fn test_get_fillone_score_intermediate(start: Board, end: Board) {
    let cuts = Cuts::new("../data/formal_cuts.json".to_string());
    let mut rng = rand::thread_rng();

    let actions = start.get_fillone_actions(&end, 0, 0, true);

    let mut new = start.clone();
    new.operate_actions(
        Vec::from(&actions[..rng.gen_range(0..actions.len())]),
        &cuts,
    );
    let (row_count, col_count) = new.check_progress(&end);

    let inter_actions = new.get_fillone_actions(&end, row_count, col_count, true);
    let res = new.get_fillone_score_intermediate(&end, row_count, col_count);
    assert_eq!(inter_actions.len(), res);
}

#[test]
fn tests_get_fillone_score_intermediate() {
    let mut rng = StdRng::seed_from_u64(42);
    for _ in 0..16 {
        let h = rng.gen_range(32..=128);
        let w = rng.gen_range(32..=128);

        let start = Board::new(random_board(h, w));
        let end = shuffle_board(start.clone().board, 42);

        test_get_fillone_score_intermediate(start, Board::new(end));
    }
}

fn test_get_fillone_actions(mut start: Board, end: Board, cuts: &Cuts) {
    let actions = start.get_fillone_actions(&end, 0, 0, true);

    start.operate_actions(actions, cuts);
    assert_eq!(start, end, "start: {:?},  end: {:?}", start, end);
}

#[test]
fn tests_actions_fillone() {
    let cuts = Cuts::new("../data/formal_cuts.json".to_string());
    let test_cases: Vec<(Board<u8>, Board<u8>)> = vec![
        (
            Board::new(vec![vec![1, 3], vec![0, 3], vec![1, 0], vec![1, 3]]),
            Board::new(vec![vec![1, 0], vec![0, 1], vec![1, 3], vec![3, 3]]),
        ),
        (
            Board::new(vec![vec![0, 0, 1], vec![2, 2, 1]]),
            Board::new(vec![vec![0, 2, 1], vec![1, 0, 2]]),
        ),
        (
            Board::new(vec![vec![2], vec![1], vec![1], vec![2], vec![2]]),
            Board::new(vec![vec![2], vec![2], vec![2], vec![1], vec![1]]),
        ),
        (
            Board::new(vec![vec![0, 0, 1, 0, 3, 0]]),
            Board::new(vec![vec![0, 3, 0, 1, 0, 0]]),
        ),
    ];

    for (start, end) in test_cases {
        test_get_fillone_actions(start, end, &cuts);
    }

    let mut rng = StdRng::seed_from_u64(42);

    {
        let h = 256;
        let w = 256;

        let start = random_board(h, w);
        let end = shuffle_board(start.clone(), 42);

        let start = Board::new(start);
        let end = Board::new(end);

        test_get_fillone_actions(start, end, &cuts);
    }

    for _ in 0..16 {
        let h = rng.gen_range(1..128);
        let w = rng.gen_range(1..128);

        let start = random_board(h, w);
        let end = shuffle_board(start.clone(), 42);

        let start = Board::new(start);
        let end = Board::new(end);

        test_get_fillone_actions(start, end, &cuts);
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
    let mut start: Board<u8> = Board::new(vec![
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
            // 上で揃える
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
        (
            // 下で揃える
            vec![
                vec![0, 0, 0, 0, 0, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0, 0, 0],
                vec![0, 0, 3, 0, 0, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 2, 0, 0, 0],
                vec![1, 1, 0, 4, 4, 0, 5, 5, 5],
            ],
            2,
            6,
            5,
            7,
        ),
        (
            // 右で揃える
            vec![
                vec![0, 0, 0, 0, 0, 0, 0, 0, 1],
                vec![0, 0, 0, 0, 0, 0, 0, 0, 1],
                vec![0, 0, 0, 0, 0, 0, 3, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0, 0, 4],
                vec![0, 0, 0, 0, 0, 0, 0, 0, 4],
                vec![0, 0, 0, 0, 0, 0, 0, 2, 0],
                vec![0, 0, 0, 0, 0, 0, 0, 0, 5],
                vec![0, 0, 0, 0, 0, 0, 0, 0, 5],
                vec![0, 0, 0, 0, 0, 0, 0, 0, 5],
            ],
            6,
            2,
            7,
            5,
        ),
        (
            // 左で揃える
            vec![
                vec![1, 0, 0, 0, 0, 0, 0, 0, 0],
                vec![1, 0, 0, 0, 0, 0, 0, 0, 0],
                vec![0, 0, 3, 0, 0, 0, 0, 0, 0],
                vec![4, 0, 0, 0, 0, 0, 0, 0, 0],
                vec![4, 0, 0, 0, 0, 0, 0, 0, 0],
                vec![0, 2, 0, 0, 0, 0, 0, 0, 0],
                vec![5, 0, 0, 0, 0, 0, 0, 0, 0],
                vec![5, 0, 0, 0, 0, 0, 0, 0, 0],
                vec![5, 0, 0, 0, 0, 0, 0, 0, 0],
            ],
            2,
            2,
            1,
            5,
        ),
        (
            // 間が2の累乗サイズ以外
            vec![
                vec![1, 1, 0, 4, 4, 4, 0, 5, 5, 5],
                vec![0, 0, 3, 0, 0, 0, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 2, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            ],
            2,
            1,
            6,
            2,
        ),
    ];
    for (board, x1, y1, x2, y2) in test_cases {
        let mut board: Board<u8> = Board::new(board);
        let mut new = board.clone();
        let actions = board.swapping(x1, y1, x2, y2);
        new.operate_actions(actions, &cuts);
        assert_eq!(board, new);
    }
    let mut rng = StdRng::seed_from_u64(42);
    for _ in 0..10 {
        let h: u32 = rng.gen_range(1..256);
        let w: u32 = rng.gen_range(1..256);
        let x1: i32 = rng.gen_range(0..w as i32);
        let x2: i32 = rng.gen_range(0..w as i32);
        let y1: i32 = rng.gen_range(0..h as i32);
        let y2: i32 = rng.gen_range(0..h as i32);
        let mut board = Board::new(random_board(h, w));
        let mut new = board.clone();
        let actions = board.swapping(x1, y1, x2, y2);
        new.operate_actions(actions, &cuts);
        assert_eq!(board, new, "swapping ({} {}), ({} {})", x1, y1, x2, y2);
    }
}

#[test]
fn test_solve_swapping() {
    let test_cases = vec![
        (vec![vec![1, 2, 3, 3, 0, 0]], vec![vec![0, 0, 1, 2, 3, 3]]),
        (
            vec![vec![1, 2, 3], vec![3, 0, 0]],
            vec![vec![0, 0, 1], vec![2, 3, 3]],
        ),
    ];
    for (start, end) in test_cases {
        let mut start: Board<u8> = Board::new(start);
        let end: Board<u8> = Board::new(end);
        // let cuts = Cuts::new("../data/formal_cuts.json".to_string());
        let actions = start.solve_swapping(&end);
        // start.operate_actions(actions, &cuts);
        println!("{:?}", actions.len());
        assert_eq!(start, end);
    }

    let mut rng = StdRng::seed_from_u64(42);
    for _ in 0..16 {
        let h: u32 = rng.gen_range(1..256);
        let w: u32 = rng.gen_range(1..256);
        let mut start: Board<u8> = Board::new(random_board(h, w));

        let end = Board::new(shuffle_board(start.clone().board, 42));
        let actions = start.solve_swapping(&end);
        println!("{:?}", actions.len());
        assert_eq!(start, end);
    }
}

fn test_caterpillar_move(mut start: Board, top_x: usize, target_x: usize, target_y: usize) {
    let cuts = Cuts::new("../data/formal_cuts.json".to_string());
    let expected_board = {
        let mut sb = start.board().clone();
        let target = sb[target_y][target_x];
        sb[0][top_x] = target;
        sb
    };

    let actions = start.caterpillar_move(top_x, target_x, target_y);
    start.operate_actions(actions, &cuts);
    assert_eq!(start.board()[0], expected_board[0]);
}

#[test]
fn tests_catapillar_move() {
    let mut rng = StdRng::seed_from_u64(42);
    for _ in 0..10 {
        let h: u32 = rng.gen_range(32..256);
        let w: u32 = rng.gen_range(32..256);
        let top_x: usize = rng.gen_range(0..w as usize);
        let target_x: usize = rng.gen_range(0..w as usize);
        let target_y: usize = rng.gen_range(1..h as usize);
        let board = Board::new(random_board(h, w));
        test_caterpillar_move(board, top_x, target_x, target_y);
    }
}

#[test]
fn test_line_fillone() {
    let cuts = Cuts::new("../data/formal_cuts.json".to_string());

    let test_cases = vec![(
        vec![
            vec![1, 3, 2, 1, 1, 2],
            vec![0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0],
        ],
        vec![
            vec![0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0],
            vec![1, 1, 1, 2, 2, 3],
            vec![0, 0, 0, 0, 0, 0],
        ],
        4,
    )];
    for (start, end, target_row) in test_cases {
        let mut start: Board<u8> = Board::new(start);
        let end: Board<u8> = Board::new(end);
        let actions = start.line_fillone(&end, target_row);
        start.operate_actions(actions, &cuts);
        assert_eq!(start.board()[0], end.board()[target_row]);
    }

    let mut rng = StdRng::seed_from_u64(42);
    for _ in 0..16 {
        let h: u32 = rng.gen_range(1..127);
        let w: u32 = rng.gen_range(1..256);
        let base: Vec<Vec<u8>> = random_board(h, w);
        let line: Vec<Vec<u8>> = random_board(1, w);

        let mut start =
            Board::new([shuffle_board(line.clone(), 42), base.clone(), base.clone()].concat());
        let end = Board::new([base.clone(), line, base].concat());

        let actions = start.line_fillone(&end, h as usize);
        start.operate_actions(actions, &cuts);
        assert_eq!(start.board()[0], end.board()[h as usize]);
    }
}

fn test_caterpillar_and_line_fillone(start: Board, end: Board, usuable_hegith: usize) {
    let cuts = Cuts::new("../data/formal_cuts.json".to_string());
    let actions = start.caterpillar_and_line_fillone(&end, usuable_hegith);
    let mut new = start.clone();
    new.operate_actions(actions, &cuts);
    assert_eq!(
        new.board()[0],
        end.board()[(end.height() - usuable_hegith) as usize]
    );
}

#[test]
fn tests_catapillar_and_line_fillone() {
    let mut test_cases: Vec<(Board<u8>, Board<u8>, usize)> = vec![
        // (
        // Board::new(vec![
        //     vec![2, 3, 0, 0, 3],
        //     vec![0, 0, 1, 0, 2],
        //     vec![3, 3, 2, 0, 2],
        //     vec![0, 0, 1, 3, 0],
        //     vec![3, 3, 2, 3, 2],
        // ]),
        // Board::new(vec![
        //     vec![0, 0, 1, 3, 0],
        //     vec![3, 3, 2, 3, 2],
        //     vec![0, 3, 3, 0, 0],
        //     vec![3, 2, 2, 2, 3],
        //     vec![2, 0, 0, 1, 0],
        // ]),
        // 3,
        // ),
        (
            Board::new(vec![
                vec![2, 2, 2, 1, 3],
                vec![0, 1, 0, 0, 2],
                vec![0, 0, 0, 3, 2],
                vec![1, 0, 3, 2, 1],
                vec![1, 0, 2, 1, 2],
                vec![1, 2, 0, 3, 3],
            ]),
            Board::new(vec![
                vec![1, 0, 3, 2, 1],
                vec![1, 0, 2, 1, 2],
                vec![1, 2, 0, 3, 3],
                vec![3, 2, 0, 0, 0],
                vec![3, 2, 0, 2, 0],
                vec![2, 1, 1, 0, 2],
            ]),
            3,
        ),
    ];

    let mut rng = StdRng::seed_from_u64(42);
    for _ in 0..1 {
        let correct_height: u32 = rng.gen_range(256..=256);
        let incorrect_height: u32 = rng.gen_range(256..=256);

        let w: u32 = rng.gen_range(256..=256);

        let correct: Vec<Vec<u8>> = random_board(correct_height, w);
        let incorrect: Vec<Vec<u8>> = random_board(incorrect_height, w);

        let start = Board::new([incorrect.clone(), correct.clone()].concat());
        let end = Board::new([correct, shuffle_board(incorrect, 42)].concat());
        test_cases.push((start, end, incorrect_height as usize))
    }

    for (start, end, usuable_hegith) in test_cases {
        test_caterpillar_and_line_fillone(start, end, usuable_hegith);
    }
}

fn test_no_op_top_distance(
    start: Board,
    end: Board,
    usable_height: usize,
    action: Action,
    cuts: &Cuts,
) {
    let res = { start.no_op_top_distance(&end, usable_height, cuts, &action) };
    let expected = {
        let mut new = start.clone();
        new.operate(&action, cuts);
        new.top_first_distance(&end, usable_height)
    };

    assert_eq!(
        res, expected,
        "action: {:?}, start: {:?} end: {:?}",
        action, start, end
    );
}

#[test]
fn tests_no_op_top_distance() {
    let cuts = Cuts::new("../data/formal_cuts.json".to_string());
    let mut test_cases: Vec<(Board, Board, usize, Action)> = vec![
        (
            Board::new(vec![
                vec![2, 3, 1],
                vec![3, 0, 1],
                vec![1, 0, 3],
                vec![1, 2, 3],
                vec![2, 1, 0],
            ]),
            Board::new(vec![
                vec![1, 2, 3],
                vec![2, 1, 0],
                vec![1, 1, 1],
                vec![2, 0, 3],
                vec![3, 0, 3],
            ]),
            3,
            Action::new(1, 2, 1, Direction::Down),
        ),
        (
            Board::new(vec![
                vec![2, 0, 2],
                vec![3, 1, 0],
                vec![2, 3, 0],
                vec![3, 2, 2],
                vec![3, 1, 3],
            ]),
            Board::new(vec![
                vec![3, 2, 2],
                vec![3, 1, 3],
                vec![0, 2, 2],
                vec![2, 3, 3],
                vec![0, 1, 0],
            ]),
            3,
            Action::new(0, 2, 1, Direction::Down),
        ),
        (
            Board::new(vec![
                vec![2, 3, 0, 0, 1],
                vec![0, 2, 0, 3, 3],
                vec![0, 1, 2, 3, 0],
                vec![2, 2, 0, 3, 1],
                vec![1, 2, 3, 3, 2],
            ]),
            Board::new(vec![
                vec![0, 1, 2, 3, 0],
                vec![2, 2, 0, 3, 1],
                vec![1, 2, 3, 3, 2],
                vec![0, 0, 3, 0, 2],
                vec![3, 2, 1, 0, 3],
            ]),
            2,
            Action::new(-1, 1, 2, Direction::Down),
        ),
        (
            Board::new(vec![
                vec![2, 3, 1],
                vec![3, 2, 0],
                vec![2, 1, 1],
                vec![2, 0, 2],
                vec![1, 1, 2],
            ]),
            Board::new(vec![
                vec![2, 1, 1],
                vec![2, 0, 2],
                vec![1, 1, 2],
                vec![2, 2, 0],
                vec![1, 3, 3],
            ]),
            3,
            Action::new(-12, 1, 15, Direction::Down),
        ),
    ];

    let mut rng: StdRng = StdRng::seed_from_u64(42);
    for _ in 0..64 {
        let correct_height: u32 = rng.gen_range(1..=128);
        let incorrect_height: u32 = rng.gen_range(2..=128);

        let w: u32 = rng.gen_range(1..=256);

        let correct: Vec<Vec<u8>> = random_board(correct_height, w);
        let incorrect: Vec<Vec<u8>> = random_board(incorrect_height, w);

        let start = Board::new([incorrect.clone(), correct.clone()].concat());
        let end = Board::new([correct, shuffle_board(incorrect, 42)].concat());

        let action = {
            let x = rng.gen_range(-128..(w as i32));
            let y = rng.gen_range(1..incorrect_height);
            let cut_num = rng.gen_range(0..25);
            Action::new(x as i32, y as i32, cut_num, Direction::Down)
        };
        test_cases.push((start, end, incorrect_height as usize, action));
    }
    for (start, end, usable_height, action) in test_cases {
        test_no_op_top_distance(start, end, usable_height, action, &cuts);
    }
}

fn test_try_caterpillar(start: Board, end: Board, usable_height: usize, expected_res: bool) {
    let cuts = Cuts::new("../data/formal_cuts.json".to_string());
    let mut new = start.clone();
    let (res, actions) = start.try_only_caterpillar(&end, usable_height);
    assert_eq!(res, expected_res);

    if res {
        new.operate_actions(actions, &cuts);
        assert_eq!(new.board()[0], end.board()[end.height() - usable_height]);
    }
}

#[test]
fn tests_try_caterpillar() {
    let test_cases: Vec<(Board, Board, usize, bool)> = vec![
        (
            Board::new(vec![
                vec![2, 3, 1],
                vec![3, 0, 1],
                vec![1, 0, 3],
                vec![1, 2, 3],
                vec![2, 1, 0],
            ]),
            Board::new(vec![
                vec![1, 2, 3],
                vec![2, 1, 0],
                vec![1, 1, 1],
                vec![2, 0, 3],
                vec![3, 0, 3],
            ]),
            3,
            true,
        ),
        (
            Board::new(vec![
                vec![0, 3, 3, 3, 1],
                vec![1, 2, 1, 2, 0],
                vec![2, 3, 1, 0, 1],
                vec![2, 3, 1, 2, 0],
                vec![2, 1, 2, 1, 3],
            ]),
            Board::new(vec![
                vec![2, 3, 1, 0, 1],
                vec![2, 3, 1, 2, 0],
                vec![2, 1, 2, 1, 3],
                vec![2, 3, 0, 1, 2],
                vec![1, 0, 1, 3, 3],
            ]),
            2,
            true,
        ),
        (
            Board::new(vec![
                vec![0, 3, 3, 3, 1],
                vec![1, 2, 1, 2, 0],
                vec![2, 3, 1, 0, 1],
                vec![2, 3, 1, 2, 0],
                vec![2, 1, 2, 1, 3],
            ]),
            Board::new(vec![
                vec![2, 3, 1, 0, 1],
                vec![2, 3, 1, 2, 0],
                vec![2, 1, 2, 1, 3],
                vec![3, 0, 3, 1, 3],
                vec![1, 2, 1, 2, 0],
            ]),
            2,
            false,
        ),
    ];

    for (start, end, usable_height, expected_res) in test_cases {
        test_try_caterpillar(start, end, usable_height, expected_res);
    }
}
