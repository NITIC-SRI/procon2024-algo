use crate::board::action::Action;
use crate::board::board::Board;
use crate::board::cut::{Cut, Cuts};

pub struct BeamSearchState {
	pub board: Board,
	pub score: i32,
	pub first_action: Action
}
