use rand::prelude::*;
use super::field::Field;

use Field::Mine;

#[derive(Debug)]
pub enum BoardState {
    Running,
    Won,
    Lost,
}

#[derive(Debug)]
pub struct Board {
    pub fields: Vec<Vec<Field>>,
    width: usize,
    height: usize,
    mines: usize,
    state: BoardState
}

impl Board {

    // Create a board from a Map of Fields.
    pub fn new(fields: Vec<Vec<Field>>) -> Board {
        let mines = fields
            .iter()
            .flat_map(|row| row.iter())
            .filter(|field| matches!(field, Mine { .. }))
            .count();
        let width = fields.len();
        let height = fields[0].len();

        Board {
            fields,
            width,
            height,
            mines,
            state: BoardState::Running,
        }
    }

    pub fn make_board(
        width: usize,
        height: usize,
        mines: usize
    ) -> Board {
        // Produce a list of locations for mines.
        let mut rng = thread_rng();
        let mut mine_locations = Vec::new();
        while mine_locations.len() < mines {
            let x = rng.gen_range(0, width);
            let y = rng.gen_range(0, height);
            if !mine_locations.contains(&(x, y)) {
                mine_locations.push((x, y));
            }
        }

        // For each Field, if it is a mine, set it to a mine, and increment the count of all adjacent fields.
        let mut fields = vec![vec![Field::new(false, 0); height]; width];
        for (x, y) in mine_locations {
            fields[x][y] = Field::new(true, 0);
            for (dx, dy) in &[
                (-1, -1), (0, -1), (1, -1),
                (-1,  0),          (1,  0),
                (-1,  1), (0,  1), (1,  1),
            ] {
                let x = x as isize + dx;
                let y = y as isize + dy;
                if x >= 0 && x < width as isize && y >= 0 && y < height as isize {
                    let x = x as usize;
                    let y = y as usize;
                    match fields[x][y] {
                        Field::Number { ref mut count, .. } => *count += 1,
                        _ => {}
                    }
                }
            }
        }

        Board::new(fields)
    }


}