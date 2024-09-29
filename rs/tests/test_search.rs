use rs::board::board::Board;
use rs::board::cut::{Cut, Cuts};
use rs::search::greedy::{self, play};

#[test]
fn test_greedy_play() {
    println!("start");

    let start_board = Board::new(vec![
        vec![1, 0, 1, 1, 2, 2, 1],
        vec![2, 3, 1, 1, 0, 0, 2],
        vec![3, 0, 2, 1, 1, 1, 1],
        vec![3, 0, 0, 2, 2, 3, 1],
        vec![2, 2, 3, 2, 0, 2, 2],
        vec![3, 3, 1, 0, 3, 2, 3],
    ]);

    let end_board = Board::new(vec![
        vec![2, 3, 1, 1, 0, 0, 2],
        vec![1, 0, 1, 1, 2, 2, 1],
        vec![3, 0, 2, 1, 1, 1, 1],
        vec![3, 0, 0, 2, 2, 3, 1],
        vec![2, 2, 3, 2, 0, 2, 2],
        vec![3, 3, 1, 0, 3, 2, 3],
    ]);

    let cuts = Cuts::new();
    let mut greedy_game = greedy::GreedyGame::new(start_board.clone(), cuts, end_board.clone());
    let actinos = play(&mut greedy_game);
    println!("{:?}", actinos);
}
