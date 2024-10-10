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

fn test_get_fillone_action_score(start: Board, end: Board) {
    let res = start.get_fillone_action_score(&end);
    let mut actions: Vec<Action>;

    let expected_score: usize = {
        let new = start.clone();
        actions = new.get_fillone_actions(&end);
        actions.len()
    };
    assert_eq!(res, expected_score, "res: {}, expected: {}", res, expected_score);
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
    let mut i = 0;
    for (start, end) in test_cases {
        println!("test case: {}", i);
        i += 1;
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

fn test_get_fillone_actions(start: Board, end: Board, cuts: &Cuts) {
    let mut new = start.clone();
    let actions = start.get_fillone_actions(&end);

    new.operate_actions(actions, cuts);
    assert_eq!(new, end, "start: {:?},  end: {:?}", start, end);
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
        )
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
