mod minesweeper;
use minesweeper::board::Board;

fn main() {
    let mut board = Board::make_board(10, 10, 10);

    print!("{:#?}", board);
}
