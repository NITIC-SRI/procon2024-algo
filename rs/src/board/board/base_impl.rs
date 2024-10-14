use super::Board;

use std::convert::Into;
use std::fmt::{Debug, Display};

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for row in &self.board {
            for cell in row {
                write!(f, "{}", cell)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl<T> Board<T>
where
    T: Copy + PartialEq + Into<usize> + Debug,
{
    pub fn new(board: Vec<Vec<T>>) -> Self {
        let height = board.len();
        let width = board[0].len();
        Board {
            board,
            width,
            height,
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn board(&self) -> &Vec<Vec<T>> {
        &self.board
    }
}
