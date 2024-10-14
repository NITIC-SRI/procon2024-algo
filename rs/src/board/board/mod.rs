mod base_impl;
mod fillone_impl;
mod operate_impl;
mod score_impl;
mod swap_impl;

use serde::{Deserialize, Serialize};
use std::convert::Into;
use std::fmt::Debug;

#[derive(Debug, Clone, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub struct Board<T = u8>
where
    T: Copy + PartialEq + Into<usize> + Debug,
{
    pub board: Vec<Vec<T>>,
    width: usize,
    height: usize,
}
