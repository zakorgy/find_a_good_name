#[macro_use]
extern crate lazy_static;
extern crate piston;
extern crate piston_window;

mod graphics;
mod game;
mod input;
mod level;

const WIDTH: u32 = 128;
const HEIGHT: u32 = WIDTH / 16 * 9;
const SCALE: u32 = 8;

fn main() {
    let mut game = game::Game::new(WIDTH, HEIGHT, SCALE);
    game.start();
    game.run();
}
