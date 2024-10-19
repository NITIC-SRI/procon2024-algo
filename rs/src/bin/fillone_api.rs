
use rs::board::cut::Cuts;
use rs::utils::{export_post_json, string_to_board, validate_actions};
use rs::client::{get,post};
use rs::board::action::{Action, Direction};

#[tokio::main]
async fn main(){
    let get_url  = String::from("http://localhost:3000/problem");
    let post_url = String::from("http://localhost:3000/answer");
    let data = get(get_url).await;
    let start = string_to_board(data.board.start);
    let end = string_to_board(data.board.goal);
    let cuts = Cuts::new("../data/formal_cuts.json".to_string());

    println!("{:?}",start);
    println!("{:?}",end);

    let actions = start.get_fillone_actions(&end, 0, 0, true);
    // let actions = vec![Action::new(0, 2, 0, Direction::Up)];
    let json = export_post_json(&actions);
    println!("{}", json);

    let res = post(post_url, json, token);
    println!("{:?}", res.await);
}
