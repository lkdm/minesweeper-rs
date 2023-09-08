mod minesweeper;
use minesweeper::game::Game;

fn main() {
    let mut game = Game::new(10, 10, 1);

    game.reveal(0, 0);
    game.reveal(1, 0);
    game.reveal(2, 0);
    game.reveal(3, 0);
    print!("{:#?}", game);
}
