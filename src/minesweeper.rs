use std::collections::HashSet;
use rand::Rng;

#[derive(Debug, PartialEq)]
pub enum GameState {
    NotReady,
    Ready,
    Playing,
    Won,
    Lost
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Field {
    Mine,
    NoMine(u8)
}

pub type Position = (usize, usize);

#[derive(Debug)]
pub struct Minesweeper {
    // board, width, and height are generated at the start of the game and never changed
    board: Vec<Vec<Field>>,
    width: usize,
    height: usize,
    mine_count: usize,
    // open_fields and flagged_fields are updated during the game
    open_fields: HashSet<Position>,
    flagged_fields: HashSet<Position>,
    game_state: GameState
}

fn iter_neighbours(
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
            mine_count,
            open_fields: HashSet::new(),
            flagged_fields: HashSet::new(),
            game_state: GameState::NotReady
        }
    }

    /**
     * Open a field. If the field is a mine, the game is lost.
     */
    pub fn open(&mut self, pos: Position) -> Option<Field> {
        // Guard; If the game is not in the playing state, do nothing
        if self.game_state != GameState::Playing {
            return None;
        }
        // Guard; If the field is already open, do nothing
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

        // If the field is a mine, the game is lost
        if let Field::Mine = field {
            self.game_state = GameState::Lost;
        }

        // If the number of closed fields is equal to the number of mines, the game is won
        if self.open_fields.len() == self.width * self.height - self.mine_count {
            self.game_state = GameState::Won;
        }

        Some(field)
    }

    /**
     * Flag a field. If the field is already flagged, unflag it.
     */
    pub fn flag(&mut self, pos: Position) {
        if self.game_state != GameState::Playing {
            return;
        }
        if self.flagged_fields.contains(&pos) {
            self.flagged_fields.remove(&pos);
        } else {
            self.flagged_fields.insert(pos);
        }
    }
}

