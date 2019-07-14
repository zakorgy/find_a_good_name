use crate::entity::Direction;
use crate::graphics::{
    screen::Screen,
    sprite::{
        Sprite, CORNERS, CURRENT_ROOM, DOOR, GRASSES, NO_ROOM, ROOM, SPRITE_SIZE_SHIFT_VALUE, VOID,
        WALLS,
    },
};

lazy_static! {
    static ref GRASS_TILE0: Tile = Tile::new(GRASSES[0], false);
    static ref GRASS_TILE1: Tile = Tile::new(GRASSES[1], false);
    static ref GRASS_TILE2: Tile = Tile::new(GRASSES[2], false);
    static ref GRASS_TILE3: Tile = Tile::new(GRASSES[3], false);
    static ref GRASS_TILE4: Tile = Tile::new(GRASSES[4], false);
    static ref GRASS_TILE5: Tile = Tile::new(GRASSES[5], false);
    static ref CORNER_TILE0: Tile = Tile::new(CORNERS[0], true);
    static ref CORNER_TILE1: Tile = Tile::new(CORNERS[1], true);
    static ref CORNER_TILE2: Tile = Tile::new(CORNERS[2], true);
    static ref CORNER_TILE3: Tile = Tile::new(CORNERS[3], true);
    static ref WALL_TILE0: Tile = Tile::new(WALLS[0], true);
    static ref WALL_TILE1: Tile = Tile::new(WALLS[1], true);
    static ref WALL_TILE2: Tile = Tile::new(WALLS[2], true);
    pub static ref VOID_TILE: Tile = Tile::new(&VOID, true);
    pub static ref DOOR_TILE: Tile = Tile::new(&DOOR, false);
    pub static ref ROOM_TILE: Tile = Tile::new(&ROOM, false);
    pub static ref CURRENT_ROOM_TILE: Tile = Tile::new(&CURRENT_ROOM, false);
    pub static ref NO_ROOM_TILE: Tile = Tile::new(&NO_ROOM, false);
    pub static ref CORNER_TILES: Vec<&'static Tile> =
        vec![&CORNER_TILE0, &CORNER_TILE1, &CORNER_TILE2, &CORNER_TILE3];
    pub static ref WALL_TILES: Vec<&'static Tile> = vec![&WALL_TILE0, &WALL_TILE1, &WALL_TILE2,];
    pub static ref GRASS_TILES: Vec<&'static Tile> = vec![
        &GRASS_TILE0,
        &GRASS_TILE1,
        &GRASS_TILE2,
        &GRASS_TILE3,
        &GRASS_TILE4,
        &GRASS_TILE5
    ];
}

pub struct Tile {
    pub solid: bool,
    pub sprite: &'static Sprite,
}

impl Tile {
    fn new(sprite: &'static Sprite, solid: bool) -> Tile {
        Tile { solid, sprite }
    }

    pub fn render(&self, x: i32, y: i32, screen: &mut Screen, orientation: Direction) {
        screen.render_tile(
            (x << SPRITE_SIZE_SHIFT_VALUE, y << SPRITE_SIZE_SHIFT_VALUE).into(),
            &self,
            orientation,
        );
    }
}
