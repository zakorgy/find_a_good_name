extern crate piston;
extern crate piston_window;

mod graphics;
mod game;
mod input;

const WIDTH: u32 = 256;
const HEIGHT: u32 = WIDTH / 16 * 9;
const SCALE: u32 = 4;

fn main() {
    let mut game = game::Game::new(WIDTH, HEIGHT, SCALE);
    game.start();
    game.run();
}
