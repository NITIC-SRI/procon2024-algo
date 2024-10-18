use rs::board::action;
use rs::board::cut::Cuts;
use rs::search::down_fillone::play;
use rs::{board::board::Board, utils};

fn main() {
    let size = 32;
    // let start: Board<u8> = Board::new(vec![
    //     vec![0, 1, 0, 3, 1],
    //     vec![3, 1, 0, 0, 1],
    //     vec![2, 2, 3, 1, 1],
    //     vec![0, 1, 2, 3, 3],
    //     vec![1, 1, 3, 2, 0],
    // ]);
    // let end = Board::new(vec![
    //     vec![0, 1, 3, 1, 1],
    //     vec![1, 1, 1, 0, 3],
    //     vec![1, 2, 0, 0, 1],
    //     vec![2, 0, 2, 3, 0],
    //     vec![1, 3, 2, 3, 3],
    // ]);
    let start = Board::new(utils::random_board(size, size));
    let end = Board::new(utils::shuffle_board(start.board().clone(), 42));

    let path = "../data/formal_cuts.json".to_string();
    let cuts = Cuts::new(path);

    // let legal_actions = utils::get_actions(size as usize, size as usize, &cuts);
    let legal_actions = utils::read_actions_by_size(size as usize, size as usize);

    let actions = play(&start, &end, &legal_actions, &cuts);
    let json = utils::export_visualyzer_json(&start, &end, actions.clone());

    println!("{}", json);
    println!("len: {}", actions.len());

    let mut now = start.clone();
    for action in actions {
        now.operate(&action, &cuts);
    }

    println!("{}", now);
    println!("---");
    println!("{}", end);
    println!("score: {}", now.absolute_distance(&end));
}
