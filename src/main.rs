extern crate piston;

mod game;

fn main() {
    let mut game = game::Game::new(300, 300, 3);
    game.run();
}