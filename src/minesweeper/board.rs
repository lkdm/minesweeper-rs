use rand::prelude::*;
use super::field::Field;

#[derive(Debug)]
pub struct Board {
    pub fields: Vec<Vec<Field>>,
    width: usize,
    height: usize,
}

impl Board {
    pub fn new(width: usize, height: usize, mines: usize) -> Board {
        let mut fields = vec![vec![Field::Unrevealed; width]; height];
        let mut rng = rand::thread_rng();
        for _ in 0..mines {
            let x = rng.gen_range(0, width);
            let y = rng.gen_range(0, height);
            fields[y][x] = Field::Mine;
        }
        Board {
            fields,
            width,
            height,
        }
    }
    pub fn reveal(&mut self, x: usize, y: usize) {
        let mut adjacent_mines = 0;
        for i in x.saturating_sub(1)..=x + 1 {
            for j in y.saturating_sub(1)..=y + 1 {
                if i < self.width && j < self.height {
                    if let Field::Mine = self.fields[j][i] {
                        adjacent_mines += 1;
                    }
                }
            }
        }
        self.fields[y][x].reveal(adjacent_mines);
    }
}