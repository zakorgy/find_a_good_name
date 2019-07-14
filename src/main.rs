#[macro_use]
extern crate lazy_static;

mod entity;
mod game;
mod graphics;
mod input;
mod level;

const WIDTH: u32 = 300;
const HEIGHT: u32 = WIDTH / 15 * 9;
const SCALE: u32 = 8;

fn main() {
    let mut game = game::Game::new(WIDTH, HEIGHT, SCALE);
    game.run();
}
