use rs::board::cut::Cuts;
use rs::search::fillone_montecarlo;
use rs::search::game::{play, Game};
use rs::{board::board::Board, utils};

fn main() {
    let size = 10;
    let start = Board::new(utils::random_board(size, size));
    let end = Board::new(utils::shuffle_board(start.board.clone(), 42));

    let path = "../data/formal_cuts.json".to_string();
    let cuts = Cuts::new(path);
    let legal_actions = utils::get_actions(size as usize, size as usize, &cuts);

    let mut game = fillone_montecarlo::MontecarloGame::new(start, &cuts, &end, &legal_actions);
    let result = play(&mut game, 10, true);

    println!("{:?}", result);
}
