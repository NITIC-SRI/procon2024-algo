use rs::board::cut::Cuts;
use rs::utils::{export_actions, get_actions};

use std::fs::File;
use std::io::Write;

fn main() {
    let cuts = Cuts::new("../data/formal_cuts.json".to_string());
    for size in vec![32, 40, 50, 64, 80, 100, 128, 256] {
        let actions = get_actions(size, size, &cuts);
        let json_str = export_actions(actions);
        let path_dir = format!("../data/compress_actions");
        std::fs::create_dir_all(&path_dir).unwrap();
        let mut file = File::create(format!("{}/{}*{}.json", path_dir, size, size)).unwrap();
        file.write_all(json_str.as_bytes()).unwrap();
        println!("end {}", size)
    }
}
