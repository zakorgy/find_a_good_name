extern crate piston;

mod graphics;
mod game;

const WIDTH: u32 = 600;
const HEIGHT: u32 = WIDTH / 16 * 9;
const SCALE: u32 = 3;

fn main() {
    let mut game = game::Game::new(WIDTH, HEIGHT, SCALE);
    game.start();
    game.run();
}