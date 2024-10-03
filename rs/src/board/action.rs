/// `Direction` 列挙型は，型抜き操作の方向を表す．
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}
#[derive(PartialEq, Eq)]
pub struct Action {
    x: i32,
    y: i32,
    cut_num: u16,
    direction: Direction,
}

/// `Action` 構造体は，型抜き操作を表す．
impl Action {
    /// 指定された座標、型番号、方向を持つ新しい `Action` を作成する．
    ///
    /// # Arguments
    ///
    /// * `x` - 型抜き操作の x 座標．
    /// * `y` - 型抜き操作の y 座標．
    /// * `cut_num` - 抜き型の番号．
    /// * `direction` - 型抜き操作の方向．
    ///
    /// # Returns
    ///
    /// 新しい `Action` 構造体．
    pub fn new(x: i32, y: i32, cut_num: u16, direction: Direction) -> Action {
        Action {
            x,
            y,
            cut_num,
            direction,
        }
    }

    /// 型抜き操作の x 座標を返す．
    ///
    /// # Returns
    ///
    /// 型抜き操作の x 座標．
    pub fn x(&self) -> i32 {
        self.x
    }

    /// 型抜き操作の y 座標を返す．
    ///
    /// # Returns
    ///
    /// 型抜き操作の y 座標．
    pub fn y(&self) -> i32 {
        self.y
    }

    /// 型抜き操作で行われた切断の数を返す．
    ///
    /// # Returns
    ///
    /// 型抜き操作で行われた抜き型の番号．
    pub fn cut_num(&self) -> u16 {
        self.cut_num
    }

    /// 型抜き操作の方向を返す．
    ///
    /// # Returns
    ///
    /// 型抜き操作の方向．
    pub fn direction(&self) -> Direction {
        self.direction
    }
}
