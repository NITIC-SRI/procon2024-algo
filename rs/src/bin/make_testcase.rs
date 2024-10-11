use rand::Rng;
use rs::board::board::Board;
use rs::utils;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
struct Test {
    board: TestBoard,
    general: TestGeneral,
}

#[derive(Deserialize, Serialize)]
struct TestBoard {
    width: u32,
    height: u32,
    start: Vec<String>,
    goal: Vec<String>,
}

#[derive(Deserialize, Serialize)]
struct TestGeneral {
    n: u32,
    pattern: Vec<Pattern>,
}

#[derive(Deserialize, Serialize)]
struct Pattern {
    p: u32,
    width: u32,
    height: u32,
    cells: Vec<String>,
}

fn main() {
    let mut testcases = vec![];
    let mut rng = rand::thread_rng();
    let mut cnt = 0;
    for k in 5..9 {
        cnt += 1;
        for _ in 0..4 {
            let size = 2u32.pow(k);
            let start = Board::new(utils::random_board(size, size));
            let end = Board::new(utils::shuffle_board(start.board.clone(), cnt));

            let num_general_cuts = rng.gen_range(1..5);
            let mut general_cuts = vec![];
            for i in 0..num_general_cuts {
                let general_cut = utils::random_general_cut(size, size);
                let pattern = Pattern {
                    p: 25 + i,
                    width: size,
                    height: size,
                    cells: general_cut,
                };
                general_cuts.push(pattern);
            }

            let test = Test {
                board: TestBoard {
                    width: size,
                    height: size,
                    start: board_to_string(&start),
                    goal: board_to_string(&end)
                },
                general: TestGeneral {
                    n: num_general_cuts,
                    pattern: general_cuts,
                },
            };

			testcases.push(test);
        }
    }
	let json = serde_json::to_string(&testcases).unwrap();
	let path = "./test.json".to_string();
	std::fs::write(path, json).expect("Unable to write file");
}

fn board_to_string(board: &Board<u8>) -> Vec<String> {
    let mut board_str = vec![];
    for row in &board.board {
        let row_str = row.iter().map(|&x| x.to_string()).collect::<Vec<String>>().join("");
        board_str.push(row_str);
    }
    board_str
}
