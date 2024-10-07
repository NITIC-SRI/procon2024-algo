use rs::board::board::Board;
use rs::board::cut::Cuts;
use rs::search::fillone_greedy;
use rs::utils::{export_actions, 

fn main() {
    // let mut start = Board::new(vec![
    //     vec![3, 2, 2, 3, 1, 0, 0, 3, 3, 1],
    //     vec![0, 2, 0, 2, 1, 3, 3, 0, 3, 0],
    //     vec![2, 0, 1, 2, 1, 3, 0, 1, 2, 0],
    //     vec![0, 1, 3, 1, 3, 2, 3, 3, 0, 2],
    //     vec![0, 1, 0, 1, 0, 2, 1, 1, 3, 0],
    //     vec![3, 0, 3, 3, 2, 3, 0, 2, 0, 1],
    //     vec![3, 0, 0, 1, 2, 3, 3, 2, 2, 2],
    //     vec![2, 0, 3, 0, 2, 1, 3, 1, 2, 0],
    //     vec![1, 2, 3, 3, 0, 2, 2, 2, 3, 2],
    //     vec![2, 0, 0, 0, 1, 3, 1, 1, 2, 3],
    // ]);

    // let end = Board::new(vec![
    //     vec![3, 1, 2, 2, 3, 3, 3, 0, 1, 3],
    //     vec![0, 1, 0, 2, 2, 1, 3, 1, 0, 2],
    //     vec![1, 0, 1, 2, 0, 3, 0, 3, 0, 0],
    //     vec![3, 3, 3, 1, 1, 3, 3, 3, 3, 1],
    //     vec![2, 3, 0, 2, 2, 1, 2, 2, 3, 1],
    //     vec![1, 0, 2, 0, 0, 0, 1, 0, 0, 0],
    //     vec![1, 0, 3, 0, 3, 3, 1, 0, 2, 0],
    //     vec![0, 2, 1, 1, 0, 3, 2, 2, 2, 2],
    //     vec![2, 2, 2, 3, 0, 3, 1, 2, 2, 3],
    //     vec![0, 2, 3, 0, 0, 1, 2, 3, 3, 2],
    // ]);

    let mut start = Board::new(vec![
        vec![4, 3, 4, 3, 4, 0, 3, 0, 3, 2, 2, 3],
        vec![3, 1, 1, 1, 3, 4, 2, 0, 3, 0, 1, 0],
        vec![1, 3, 0, 2, 3, 3, 4, 1, 0, 1, 4, 0],
        vec![3, 2, 2, 1, 4, 4, 2, 4, 1, 2, 3, 4],
        vec![2, 0, 1, 0, 1, 2, 2, 0, 1, 3, 3, 0],
        vec![2, 3, 2, 2, 2, 4, 3, 4, 3, 0, 4, 0],
        vec![4, 3, 0, 3, 3, 3, 1, 0, 0, 2, 4, 0],
        vec![1, 2, 3, 3, 0, 2, 0, 4, 3, 2, 3, 1],
        vec![4, 3, 1, 1, 3, 3, 4, 3, 1, 3, 3, 0],
        vec![3, 4, 4, 1, 4, 1, 1, 2, 1, 3, 1, 4],
        vec![1, 0, 0, 4, 2, 2, 3, 4, 0, 0, 4, 3],
        vec![1, 0, 2, 0, 1, 3, 2, 4, 0, 0, 0, 3],
    ]);
    let end = Board::new(vec![
        vec![3, 2, 3, 3, 3, 0, 4, 4, 4, 2, 2, 3],
        vec![2, 1, 3, 0, 4, 3, 0, 4, 1, 3, 4, 0],
        vec![3, 3, 4, 3, 2, 3, 0, 0, 0, 1, 2, 2],
        vec![2, 0, 1, 0, 3, 3, 4, 4, 0, 1, 0, 3],
        vec![4, 3, 0, 2, 0, 4, 1, 1, 1, 0, 3, 2],
        vec![0, 3, 3, 3, 3, 3, 4, 1, 4, 1, 1, 1],
        vec![0, 4, 3, 1, 3, 0, 4, 3, 1, 1, 2, 0],
        vec![1, 1, 0, 4, 2, 0, 3, 1, 2, 0, 2, 4],
        vec![0, 0, 1, 2, 2, 3, 4, 3, 1, 0, 4, 1],
        vec![2, 3, 1, 1, 4, 4, 2, 2, 2, 4, 1, 0],
        vec![2, 3, 0, 0, 4, 3, 3, 3, 2, 3, 0, 1],
        vec![4, 4, 4, 3, 3, 2, 1, 2, 3, 0, 3, 0],
    ]);

    // let mut start = Board::new(rs::utils::random_board(12, 12));
    // let end = Board::new(rs::utils::shuffle_board(start.clone().board, 42));
    println!("{:?}", start);
    println!("{:?}", end);
    println!("{:?}", start.absolute_distance(&end));

    let path = "../data/formal_cuts.json".to_string();
    let cuts = Cuts::new(path);
    let legal_actions =
    let mut greedy_game = fillone_greedy::GreedyGame::new(&mut start, &cuts, &end);

    let actions = greedy::play(&mut greedy_game);
    for action in actions.iter() {
        println!("{:?}", action);
    }
    println!("{:?}", actions.len());
    println!("{:?}", start.absolute_distance(&end));
    let json = export_actions(actions);
    println!("{}", json);
}
