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
        // Create a new board with the given dimensions and number of mines.
        // First, create a vector of random coords within the width and size to lay mines.
        let mut rng = thread_rng();
        let mut mine_coords = Vec::with_capacity(mines);
        while mine_coords.len() < mines {
            let x = rng.gen_range(0, width);
            let y = rng.gen_range(0, height);
            if !mine_coords.contains(&(x, y)) {
                mine_coords.push((x, y));
            }
        }

        // Then create a vector of vector of fields, corresponding to the coordinates.
        // Each field, you pass in whether it is a mine, and the amount of adjacent mines.
        // Then you return the board.
        let mut fields = Vec::with_capacity(height);
        for y in 0..height {
            let mut row = Vec::with_capacity(width);
            for x in 0..width {
                let has_mine = mine_coords.contains(&(x, y));
                let mut adjacent_mines = 0;
                for i in x.saturating_sub(1)..=x + 1 {
                    for j in y.saturating_sub(1)..=y + 1 {
                        if i < width && j < height {
                            if mine_coords.contains(&(i, j)) {
                                adjacent_mines += 1;
                            }
                        }
                    }
                }
                row.push(Field::new(has_mine, adjacent_mines));
            }
            fields.push(row);
        }
        Board {
            fields,
            width,
            height,
        }
    }
}