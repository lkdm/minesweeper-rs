mod minesweeper;
use minesweeper::Minesweeper;

fn main() {
    let mut minesweeper = Minesweeper::new(10, 10, 10);
    print!("{:#?}", minesweeper);
}
