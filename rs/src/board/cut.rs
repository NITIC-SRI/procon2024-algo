use std::ops::Index;

pub struct Cut {
    pub cut: Vec<Vec<bool>>,
	width: usize,
	height: usize,
}

impl Cut {
    pub fn new(cut: Vec<Vec<bool>>) -> Cut {
        let height = cut.len();
		let width = cut[0].len();
		Cut { cut, width, height }
    }

    pub fn get_formal_cut(cut_num: u32) -> Cut {
        // TODO: jsonから読み込むようにする．
		Cut {
			cut: Vec::new(),
			width: 0,
			height: 0,
		}
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }
}

impl Index<usize> for Cut {
	type Output = Vec<bool>;

	fn index(&self, index: usize) -> &Self::Output {
		&self.cut[index]
	}
}

pub struct Cuts {
    cuts: Vec<Cut>,
}

impl Cuts {
    pub fn new() -> Cuts {
        let mut cuts = Vec::new();
        for i in 0..25 {
            cuts.push(Cut::get_formal_cut(i));
        }
        Cuts { cuts }
    }

	pub fn push(&mut self, cut: Cut) {
		self.cuts.push(cut);
	}
}

impl Index<u32> for Cuts {
	type Output = Cut;

	fn index(&self, index: u32) -> &Self::Output {
		&self.cuts[index as usize]
	}
}
