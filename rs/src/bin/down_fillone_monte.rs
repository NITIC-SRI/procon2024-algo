use rs::board::cut::Cuts;
use rs::search::down_fillone_montecarlo::play;
use rs::{board::board::Board, utils};

fn main() {
    let size = 256;
    // let start: Board<u8> = Board::new(vec![
    //     vec![3, 2, 3, 1],
    //     vec![0, 3, 0, 3],
    //     vec![1, 2, 2, 3],
    //     vec![3, 0, 0, 1],
    // ]);
    // let end = Board::new(vec![
    //     vec![3, 2, 0, 1],
    //     vec![3, 2, 1, 0],
    //     vec![3, 0, 3, 2],
    //     vec![0, 1, 3, 3],
    // ]);
    let start = Board::new(utils::random_board(size, size));
    let end = Board::new(utils::shuffle_board(start.board().clone(), 42));

    let path = "../data/formal_cuts.json".to_string();
    let cuts = Cuts::new(path);
    let legal_actions = utils::read_actions_by_size(size as usize, size as usize);

    let actions = play(&start, &end, &legal_actions, &cuts, 1000, 100000);
    let json = utils::export_visualyzer_json(&start, &end, actions.clone());

    // println!("{}", json);
    println!("len: {}", actions.len());

    let mut now = start.clone();
    for action in actions {
        now.operate(&action, &cuts);
    }

    assert_eq!(now, end);

    // println!("{}", now);
    // println!("---");
    // println!("{}", end);
    println!("score: {}", now.absolute_distance(&end));
}
