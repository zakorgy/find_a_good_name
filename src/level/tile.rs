use super::super::graphics::screen::Screen;
use super::super::graphics::sprite::Sprite;
use super::super::graphics::sprite::{GROUNDS, VOID};

lazy_static! {
    pub static ref GROUND_TILE0: Tile = Tile::new(GROUNDS[0]);
    pub static ref GROUND_TILE1: Tile = Tile::new(GROUNDS[1]);
    pub static ref VOID_TILE: Tile = Tile::new(&VOID);
}

pub struct Tile {
    x: u32,
    y: u32,
    pub solid: bool,
    pub sprite: &'static Sprite,
}

impl Tile {
    fn new(sprite: &'static Sprite) -> Tile {
        Tile {x: 0, y: 0, solid: false, sprite}
    }

    pub fn render(&self, x: i32, y: i32, screen: &mut Screen) {
        screen.render_tile(x << 3, y << 3, &self);
    }
}
