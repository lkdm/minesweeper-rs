use super::board::Board;

enum Game {
    Running(Board),
    Won(Board),
    Lost(Board),
}

impl Game {
    fn new(width: usize, height: usize, mines: usize) -> Game {
        Game::Running(Board::new(width, height, mines))
    }
    fn reveal(&mut self, x: usize, y: usize) {
        match self {
            Game::Running(board) => {
                board.reveal(x, y);
                if board.is_won() {
                    *self = Game::Won(board.clone());
                }
            }
            _ => {}
        }
    }
    fn flag(&mut self, x: usize, y: usize) {
        match self {
            Game::Running(board) => board.flag(x, y),
            _ => {}
        }
    }
}