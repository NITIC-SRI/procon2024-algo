use rs::client::{get, post};
use rs::utils::{export_post_json, string_to_board};
use rs::board::cut::Cuts;
use rs::utils;
use rs::search::down_fillone::play;

#[tokio::main]
async fn main() {
    let get_url = String::from("http://localhost:3000/problem");
    let post_url = String::from("http://localhost:3000/answer");
    let token = "token1".to_string();




    let data = get(get_url, token.clone()).await;
    let start = string_to_board(data.board.start);
    let end = string_to_board(data.board.goal);
    let size_h = data.board.height;
    let size_w = data.board.width;

    let path = "../data/formal_cuts.json".to_string();
    let cuts = Cuts::new(path);

    let legal_actions = utils::read_actions_by_size(size_w as usize, size_h as usize);

    let actions = play(&start, &end, &legal_actions, &cuts, 1000);

    // let actions = vec![Action::new(0, 2, 0, Direction::Up)];
    let json = export_post_json(&actions);
    println!("{}", json);
    let res = post(post_url, json, token);
    println!("{:?}", res.await);
}
