use rand::Rng;
use std::convert::From;
use std::default::Default;
use std::path::PathBuf;
use super::super::graphics::Screen;
use super::super::graphics::image;

pub(crate) type RoomId = u8;

#[derive(Eq, PartialEq, Debug, Copy, Clone)]
pub enum Neighbour {
    North(RoomId),
    South(RoomId),
    East(RoomId),
    West(RoomId),
    Invalid,
}

impl Default for Neighbour {
    fn default() -> Self { Neighbour::Invalid }
}

pub enum RoomType {
    Start,
    Normal,
}

impl Default for RoomType {
    fn default() -> Self { RoomType::Normal }
}

pub enum Tiles {
    Empty,
    Grass(usize),
    Ground,
    Wall(usize),
    WallTop(usize),
    SpawnPoint(usize),
    Door,
}

impl From<u32> for Tiles {
    fn from(num: u32) -> Tiles {
        match num {
            1 => Tiles::Ground,
            2 => Tiles::Grass(0),
            3 => Tiles::Wall(0),
            4 => Tiles::WallTop(0),
            5 => Tiles::SpawnPoint(0),
            6 => Tiles::Door,
            _ => Tiles::Empty,
        }
    }
}

#[derive(Default)]
pub struct RoomBuilder {
    neighbours: [Neighbour; 4],
    room_type: RoomType,
    grid_pos: (i32, i32),
    path: PathBuf,
    id: RoomId,
}

impl RoomBuilder {
    pub fn new() -> RoomBuilder {
        RoomBuilder {
            neighbours:[Neighbour::Invalid; 4],
            room_type: RoomType::Normal,
            grid_pos: (0, 0),
            path: Default::default(),
            id: 0,
        }
    }

    pub fn add_neighbour(&mut self, neighbour: Neighbour) {
        for n in self.neighbours.iter_mut() {
            if *n == Neighbour::Invalid {
                *n = neighbour;
                break;
            }
        }
    }

    pub fn with_room_type(mut self, room_type: RoomType) -> RoomBuilder {
        self.room_type = room_type;
        self
    }

    pub fn with_grid_pos(mut self, grid_pos: (i32, i32)) -> RoomBuilder {
        self.grid_pos = grid_pos;
        self
    }

    pub fn with_path(mut self, path: &PathBuf) -> RoomBuilder {
        self.path = path.clone();
        self
    }

    pub fn with_id(mut self, id: RoomId) -> RoomBuilder {
        self.id = id;
        self
    }

    pub fn build(self) -> (RoomId, Room) {
        let image = match image::open(&self.path) {
            Ok(image) => image.to_rgba(),
            Err(err) => panic!("Error loading image: {:?} with path {:?}", err, &self.path),
        };
        let (width, height) = image.dimensions();
        let mut tiles = Vec::new();
        for y in 0..height {
            for x in 0..width {
                match image.get_pixel(x, y) {
                    image::Rgba{ data: [255, 0, 0, 255]} => tiles.push(Tiles::Wall(rand::thread_rng().gen_range(0_usize, 3))),
                    image::Rgba{ data: [0, 0, 255, 255]} => tiles.push(Tiles::Door),
                    image::Rgba{ data: [0, 255, 255, 255]} => tiles.push(Tiles::WallTop(rand::thread_rng().gen_range(0_usize, 3))),
                    image::Rgba{ data: [0, 255, 0, 255]} => tiles.push(Tiles::Grass(rand::thread_rng().gen_range(0_usize, 2))),
                    image::Rgba{ data: [255, 255, 0, 255]} => tiles.push(Tiles::SpawnPoint(rand::thread_rng().gen_range(0_usize, 2))),
                    _ => tiles.push(Tiles::Empty),
                }
            }
        }
        (self.id, Room {
            neighbours: self.neighbours,
            width: width as _,
            height: height as _,
            tiles,
            room_type: self.room_type,
            grid_pos: self.grid_pos,
        })
    }
}

pub struct Room {
    pub width : i32,
    pub height: i32,
    pub neighbours: [Neighbour; 4],
    pub tiles: Vec<Tiles>,
    pub room_type: RoomType,
    pub grid_pos: (i32, i32),
}

impl Room {

    pub fn update(&mut self) {}

    pub fn render(&self, x_scroll: i32, y_scroll: i32, screen: &mut Screen) {
        screen.set_offset(x_scroll, y_scroll);
        let x0 = x_scroll >> 3;
        let x1 = (x_scroll + screen.width as i32) >> 3;
        let y0 = y_scroll >> 3;
        let y1 = (y_scroll + screen.height as i32) >> 3;

        for y in y0..=y1 {
            for x in x0..=x1 {
                self.get_tile(x, y).render(x, y, screen);
            }
        }
    }

    pub fn get_tile(&self, x: i32, y: i32) -> &'static super::tile::Tile {
        if x < 0 || x >= self.width || y < 0 || y >= self.height {
            return &super::tile::VOID_TILE;
        }
        match self.tiles.get((x + y * self.width) as usize).expect("Out of bounds") {
            Tiles::Ground => &super::tile::GROUND_TILES[0],
            Tiles::Wall(i)  => &super::tile::WALL_TILES[*i],
            Tiles::WallTop(i) => &super::tile::WALL_TOP_TILES[*i],
            Tiles::Grass(i) => &super::tile::GRASS_TILES[*i],
            Tiles::SpawnPoint(i) => &super::tile::GRASS_TILES[*i],
            Tiles::Door => &super::tile::DOOR_TILE,
            _ => &super::tile::VOID_TILE,
        }
    }

    pub fn dimensions(&self) -> (i32, i32) {
        (self.width * 8, self.height * 8)
    }

    pub fn spawn_point(&self) -> (f32, f32) {
        for (i, tile) in self.tiles.iter().enumerate() {
            if let Tiles::SpawnPoint(_) = tile {
                return ((i as i32 % self.width) as f32 * 8f32, (i as i32 / self.width) as f32 * 8f32)
            }
        }
        return (0f32, 0f32)
    }
}
