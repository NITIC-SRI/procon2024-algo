use rs::board::action::{Action, Direction};
use rs::board::board::Board;
use rs::utils::export_actions;
#[test]
fn test_export_actions() {
    let actions = vec![
        Action::new(1, 1, 1, Direction::Up),
        Action::new(2, 2, 2, Direction::Down),
        Action::new(3, 3, 3, Direction::Right),
        Action::new(4, 4, 4, Direction::Left),
    ];

    let json = export_actions(actions);
    let collect = r#"{"n":4,"ops":[{"p":1,"x":1,"y":1,"s":"0"},{"p":2,"x":2,"y":2,"s":"1"},{"p":3,"x":3,"y":3,"s":"2"},{"p":4,"x":4,"y":4,"s":"3"}]}"#.to_string();
    assert_eq!(json, collect);
}

#[test]
fn test_random_board() {
    let board = Board::new(rs::utils::random_board(10, 10));
    let shuffled_board = Board::new(rs::utils::shuffle_board(board.clone().board, 42));

    println!("{}", board);
    println!("{}", shuffled_board);
}
