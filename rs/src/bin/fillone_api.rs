use rs::client::{get, post};
use rs::utils::{export_post_json, string_to_board};

#[tokio::main]
async fn main() {
    let get_url = String::from("http://localhost:3000/problem");
    let post_url = String::from("http://localhost:3000/answer");
    let data = get(get_url).await;
    let start = string_to_board(data.board.start);
    let end = string_to_board(data.board.goal);

    println!("{:?}", start);
    println!("{:?}", end);
    let actions = start.get_fillone_actions(&end, 0, 0, true);
    // let actions = vec![Action::new(0, 2, 0, Direction::Up)];
    let json = export_post_json(&actions);
    println!("{}", json);
    let res = post(post_url, json);
    println!("{:?}", res.await);
}
