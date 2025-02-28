use serde::{Deserialize, Serialize};
use std::fs::File;
use std::ops::Index;

#[derive(Debug, Clone)]
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

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn delete_only_zero_bottom(&mut self) {
        let mut is_bottom_only_zero = true;
        for w in 0..self.width() {
            if self[self.height() - 1][w] {
                is_bottom_only_zero = false;
            }
        }

        if is_bottom_only_zero {
            self.height -= 1;
            self.cut.pop();
        }
    }

    pub fn parse_cut_string(cells: &Vec<String>) -> Self {
        let mut cut: Vec<Vec<bool>> = Vec::new();
        for cell in cells {
            let mut row = Vec::new();
            for c in cell.chars() {
                if c == '1' {
                    row.push(true);
                } else {
                    row.push(false);
                }
            }
            cut.push(row);
        }
        Cut::new(cut)
    }
}

impl Index<usize> for Cut {
    type Output = Vec<bool>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.cut[index]
    }
}

#[derive(Debug, Clone)]
pub struct Cuts {
    pub cuts: Vec<Cut>,
}

impl Cuts {
    pub fn new(path: String) -> Cuts {
        let json = get_json(path);
        let mut cuts = Vec::new();

        for cells in json.formal {
            let mut cut = Vec::new();
            for cell in cells.cells {
                let mut row = Vec::new();
                for c in cell.chars() {
                    if c == '1' {
                        row.push(true);
                    } else {
                        row.push(false);
                    }
                }
                cut.push(row);
            }
            let cut = Cut::new(cut);
            cuts.push(cut);
        }
        Cuts { cuts }
    }

    pub fn parse_cuts_string(&mut self, cells: &Vec<Vec<String>>) {
        for cell in cells {
            let cut = Cut::parse_cut_string(cell);
            self.push(cut);
        }
    }

    pub fn push(&mut self, cut: Cut) {
        self.cuts.push(cut);
    }

    pub fn len(&self) -> usize {
        self.cuts.len()
    }

    pub fn delete_only_zero_bottoms(&mut self) {
        for i in 0..self.len() {
            self.cuts[i].delete_only_zero_bottom();
        }
    }
}

impl Index<u32> for Cuts {
    type Output = Cut;

    fn index(&self, index: u32) -> &Self::Output {
        &self.cuts[index as usize]
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Json {
    formal: Vec<Cells>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Cells {
    p: u32,
    width: u32,
    height: u32,
    cells: Vec<String>,
}

pub fn get_json(path: String) -> Json {
    let file = File::open(path).unwrap();
    let data: Json = serde_json::from_reader(file).unwrap();
    data
}
