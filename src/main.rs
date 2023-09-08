mod minesweeper;
use minesweeper::board::Board;

fn main() {
    let mut board = Board::new(10, 10, 1);
    println!("{:#?}", &board);
    // board.reveal(0, 0);
    // println!("{:#?}", &board.fields[0][0]);
}
