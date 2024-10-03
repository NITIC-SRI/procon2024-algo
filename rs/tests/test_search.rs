use rs::board::board::Board;
use rs::board::cut::Cuts;
use rs::search::greedy;

#[test]
fn test_greedy_play() {
    println!("start");

    let mut start_board = Board::new(vec![
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

    println!("start score: {}", start_board.absolute_distance(&end_board));
    let path = "../data/formal_cuts.json".to_string();
    let cuts = Cuts::new(path);
    let mut greedy_game = greedy::GreedyGame::new(&mut start_board, &cuts, &end_board);
    let actinos = greedy::play(&mut greedy_game);
    for action in actinos.iter() {
        println!("{:?}", action);
    }
    println!("{:?}", actinos.len());
}
