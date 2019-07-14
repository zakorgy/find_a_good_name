#[macro_use]
extern crate lazy_static;

mod entity;
mod game;
mod graphics;
mod input;
mod level;

const WIDTH: u32 = 200;
const HEIGHT: u32 = WIDTH / 16 * 9;
const SCALE: u32 = 6;

fn main() {
    let mut game = game::Game::new(WIDTH, HEIGHT, SCALE);
    game.run();
}
