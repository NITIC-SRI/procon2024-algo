use rs::board::board::Board;
use rs::board::cut::Cuts;
use rs::search::greedy;


fn main() {
    let mut start = Board::new(vec![
        vec![3, 2, 2, 3, 1, 0, 0, 3, 3, 1],
        vec![0, 2, 0, 2, 1, 3, 3, 0, 3, 0],
        vec![2, 0, 1, 2, 1, 3, 0, 1, 2, 0],
        vec![0, 1, 3, 1, 3, 2, 3, 3, 0, 2],
        vec![0, 1, 0, 1, 0, 2, 1, 1, 3, 0],
        vec![3, 0, 3, 3, 2, 3, 0, 2, 0, 1],
        vec![3, 0, 0, 1, 2, 3, 3, 2, 2, 2],
        vec![2, 0, 3, 0, 2, 1, 3, 1, 2, 0],
        vec![1, 2, 3, 3, 0, 2, 2, 2, 3, 2],
        vec![2, 0, 0, 0, 1, 3, 1, 1, 2, 3],
    ]);

    let end = Board::new(vec![
        vec![3, 1, 2, 2, 3, 3, 3, 0, 1, 3],
        vec![0, 1, 0, 2, 2, 1, 3, 1, 0, 2],
        vec![1, 0, 1, 2, 0, 3, 0, 3, 0, 0],
        vec![3, 3, 3, 1, 1, 3, 3, 3, 3, 1],
        vec![2, 3, 0, 2, 2, 1, 2, 2, 3, 1],
        vec![1, 0, 2, 0, 0, 0, 1, 0, 0, 0],
        vec![1, 0, 3, 0, 3, 3, 1, 0, 2, 0],
        vec![0, 2, 1, 1, 0, 3, 2, 2, 2, 2],
        vec![2, 2, 2, 3, 0, 3, 1, 2, 2, 3],
        vec![0, 2, 3, 0, 0, 1, 2, 3, 3, 2],
    ]);

    let path = "../data/formal_cuts.json".to_string();
    let cuts = Cuts::new(path);
    let mut greedy_game = greedy::GreedyGame::new(&mut start, &cuts, &end);
    let actinos = greedy::play(&mut greedy_game);
    for action in actinos.iter() {
        println!("{:?}", action);
    }
    println!("{:?}", actinos.len());
    println!("{:?}", start.absolute_distance(&end));
    println!("{}", start);
}
