#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate bitflags;

mod entity;
mod game;
mod graphics;
mod input;
mod level;

const WIDTH: u32 = 270;
const HEIGHT: u32 = WIDTH / 15 * 9;
const SCALE: u32 = 6;

fn main() {
    let mut game = game::Game::new(WIDTH, HEIGHT, SCALE);
    game.run();
}
