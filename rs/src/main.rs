use std::process::exit;

use rs::board::action::Action;
use rs::board::board::Board;
use rs::board::cut::{Cut, Cuts};
use rs::client::{get, post};
use rs::search::down_fillone;
use rs::search::down_fillone_montecarlo;
use rs::utils::{self, validate_actions};
use rs::utils::{export_post_json, string_to_board};

#[tokio::main]
async fn main() {
    // let get_url = String::from("http://localhost:3000/problem");
    // let post_url = String::from("http://localhost:3000/answer");

    let get_url = String::from("http://172.29.1.2:80/problem");
    let post_url = String::from("http://172.29.1.2:80/answer");

    let token = "ichinoseki3984c30163ebc918a611915851c8a720a5f90924c5754e66020211".to_string();

    let data = get(get_url, token.clone()).await;
    let start = string_to_board(data.board.start);
    let end = string_to_board(data.board.goal);

    let fillone_score = {
        let actions = start.get_fillone_actions(&end, 0, 0, true);
        println!("fillone actions len {}", actions.len());
        let json = export_post_json(&actions);
        post(post_url.clone(), json, token.clone()).await;
        actions.len()
    };

    let size_h = data.board.height;
    let size_w = data.board.width;

    let path = "../data/formal_cuts.json".to_string();
    let mut cuts: Cuts = Cuts::new(path);
    for pat in data.general.patterns {
        let cut = pat.cells;
        let cut = Cut::parse_cut_string(&cut);
        cuts.push(cut);
    }

    // let mut legal_actions = utils::read_actions_by_size(size_w as usize, size_h as usize);
    let legal_actions = utils::get_general_actions(size_h as usize, size_w as usize, &cuts);
    println!("legal_actions len {:}", legal_actions.len());

    let actions = select_algorithm(size_h, size_w, &start, &end, &legal_actions, &cuts);
    {
        if !validate_actions(&start, &end, &actions, &cuts) {
            println!("new != end");
            exit(1);
        }
    }
    {
        if fillone_score < actions.len() {
            println!("fillone_score < actions.len()");
            exit(1);
        }
    }

    println!("actions len {}", actions.len());

    let json = export_post_json(&actions);
    // println!("{}", json);
    let res = post(post_url, json, token);
    println!("{:?}", res.await);
}

fn select_algorithm(
    h: u32,
    w: u32,
    start: &Board,
    end: &Board,
    legal_actions: &Vec<Action>,
    cuts: &Cuts,
) -> Vec<Action> {
    if h <= 128 && w <= 128 {
        return down_fillone::play(start, end, legal_actions, cuts, 1000);
    } else {
        return down_fillone_montecarlo::play(start, end, legal_actions, cuts, 1000, 1000000);
    }
}
