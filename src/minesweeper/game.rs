use super::board::Board;

#[derive(Debug)]
pub enum Game {
    Running(Box<Board>),
    Won(Box<Board>),
    Lost(Box<Board>),
}

impl Game {
    pub fn new(width: usize, height: usize, mines: usize) -> Game {
        Game::Running(Box::new(Board::new(width, height, mines)))
    }

    pub fn flag(&mut self, x: usize, y: usize) {
        match self {
            Game::Running(board) => board.flag(x, y),
            _ => {}
        }
    }

    pub fn reveal(&mut self, x: usize, y: usize) {
        match self {
            Game::Running(board) => {
                board.reveal(x, y);
                // Check if board is won or lost.
            }
            _ => {}
        }
    }

}