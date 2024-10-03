use rs::board::action::{Action, Direction};
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
	println!("{}", json);
}
