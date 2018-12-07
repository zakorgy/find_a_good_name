use super::super::graphics::{Screen, Sprite, GRASSES, GROUNDS, VOID, WALLS, WALL_TOPS};

lazy_static! {
    static ref GROUND_TILE0: Tile = Tile::new(GROUNDS[0], false);
    static ref GROUND_TILE1: Tile = Tile::new(GROUNDS[1], false);
    static ref WALL_TOP_TILE0: Tile = Tile::new(WALL_TOPS[0], true);
    static ref WALL_TOP_TILE1: Tile = Tile::new(WALL_TOPS[1], true);
    static ref WALL_TOP_TILE2: Tile = Tile::new(WALL_TOPS[2], true);
    static ref WALL_TILE0: Tile = Tile::new(WALLS[0], false);
    static ref WALL_TILE1: Tile = Tile::new(WALLS[1], false);
    static ref WALL_TILE2: Tile = Tile::new(WALLS[2], false);
    static ref GRASS_TILE0: Tile = Tile::new(GRASSES[0], false);
    static ref GRASS_TILE1: Tile = Tile::new(GRASSES[1], false);

    pub static ref VOID_TILE: Tile = Tile::new(&VOID, true);

    pub static ref WALL_TOP_TILES: Vec<&'static Tile> = vec![
        &WALL_TOP_TILE0,
        &WALL_TOP_TILE1,
        &WALL_TOP_TILE2,
    ];

    pub static ref WALL_TILES: Vec<&'static Tile> = vec![
        &WALL_TILE0,
        &WALL_TILE1,
        &WALL_TILE2,
    ];

    pub static ref GRASS_TILES: Vec<&'static Tile> = vec![
        &GRASS_TILE0,
        &GRASS_TILE1,
    ];

    pub static ref GROUND_TILES: Vec<&'static Tile> = vec![
        &GROUND_TILE0,
        &GROUND_TILE1,
    ];
}

pub struct Tile {
    //x: u32,
    //y: u32,
    pub solid: bool,
    pub sprite: &'static Sprite,
}

impl Tile {
    fn new(sprite: &'static Sprite, solid: bool) -> Tile {

        Tile {
            //x: 0, y: 0,
            solid,
            sprite
        }
    }

    pub fn render(&self, x: i32, y: i32, screen: &mut Screen) {
        screen.render_tile(x << 3, y << 3, &self);
    }
}
