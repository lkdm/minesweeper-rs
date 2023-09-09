use std::{
    collections::HashSet,
    fmt::{Display, Write},
  };
use rand::Rng;

#[derive(Debug, PartialEq, Clone, Copy)]
enum Field {
    Mine,
    NoMine(u8)
}

pub type Position = (usize, usize);

#[derive(Debug)]
pub struct Minesweeper {
    board: Vec<Vec<Field>>,
    width: usize,
    height: usize,
    open_fields: HashSet<Position>,
    flagged_fields: HashSet<Position>
}

pub fn iter_neighbours(
    width: usize,
    height: usize,
    (x, y): Position
) -> impl Iterator<Item = Position> {
    let relative_locations = vec![
        (-1, -1), (0, -1), (1, -1),
        (-1,  0),          (1,  0),
        (-1,  1), (0,  1), (1,  1),
    ];

    relative_locations.into_iter()
        // Convert from relative locations to absolute Position
        .map(move |(dx, dy)| (x as isize + dx, y as isize + dy))
        // Filter out Positions that are out of bounds
        .filter(move |&(x, y)| x >= 0 && x < width as isize && y >= 0 && y < height as isize)
        // Convert to usize, which is used by Position
        .map(|(x, y)| (x as usize, y as usize))
}

impl Minesweeper {

    pub fn new(
        width: usize,
        height: usize,
        mine_count: usize
    ) -> Minesweeper {
        let mut rng = rand::thread_rng();
        let mut board = vec![vec![Field::NoMine(0); width]; height];

        // First lay mines
        for _ in 0..mine_count {
            let mut x = rng.gen_range(0, width);
            let mut y = rng.gen_range(0, height);

            // Keep trying to place mines. If there is already a mine, try again
            while board[y][x] == Field::Mine {
                x = rng.gen_range(0, width);
                y = rng.gen_range(0, height);
            }
            board[y][x] = Field::Mine;

            // Then use the mines to calculate the adjacent mine count
            iter_neighbours(width, height, (x, y))
                .for_each(|(x, y)| {
                    match board[y][x] {
                        Field::Mine => (),
                        Field::NoMine(ref mut count) => *count += 1
                    }
                });
        }

        Minesweeper {
            board,
            width,
            height,
            open_fields: HashSet::new(),
            flagged_fields: HashSet::new(),
        }
    }

    pub fn adjacent_mines(&self, pos: Position) -> u8 {
        iter_neighbours(self.width, self.height, pos)
            .filter(|&neighbor_pos| self.board[neighbor_pos.1][neighbor_pos.0] == Field::Mine)
            .count() as u8
    }

    pub fn open(&mut self, pos: Position) -> Option<Field> {
        if self.open_fields.contains(&pos) {
            return None;
        }

        let field = self.board[pos.1][pos.0];
        self.open_fields.insert(pos);

        //  Check if mine count is 0
        if let Field::NoMine(0) = field {
            // Open all adjacent fields
            iter_neighbours(self.width, self.height, pos)
                .for_each(|neighbor_pos| {
                    self.open(neighbor_pos);
                });
        }

        Some(field)
    }
}

