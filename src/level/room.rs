use super::super::graphics::image;
use super::super::graphics::{Screen, SPRITE_SIZE_U32, SPRITE_SIZE_SHIFT_VALUE};
use rand::Rng;
use std::convert::From;
use std::default::Default;
use std::path::PathBuf;
use cgmath::Vector2;

const MAX_NEIGHBOUR: usize = 4;

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
    fn default() -> Self {
        Neighbour::Invalid
    }
}

pub enum RoomType {
    Start,
    Normal,
}

impl Default for RoomType {
    fn default() -> Self {
        RoomType::Normal
    }
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

pub struct RoomBuilder {
    neighbours: [Neighbour; MAX_NEIGHBOUR],
    room_type: RoomType,
    grid_pos: Vector2<i32>,
    path: PathBuf,
    pub id: RoomId,
}

impl Default for RoomBuilder {
    fn default() -> RoomBuilder {
        RoomBuilder {
            neighbours: Default::default(),
            room_type: Default::default(),
            grid_pos: (0, 0).into(),
            path: Default::default(),
            id: Default::default(),
        }
    }
}

impl RoomBuilder {
    pub fn new() -> RoomBuilder {
        RoomBuilder {
            neighbours: [Neighbour::Invalid; MAX_NEIGHBOUR],
            room_type: RoomType::Normal,
            grid_pos: (0, 0).into(),
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

    pub fn with_grid_pos(mut self, grid_pos: Vector2<i32>) -> RoomBuilder {
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
        let mut possible_door_positions = Vec::new();
        for y in 0..height {
            for x in 0..width {
                match image.get_pixel(x, y) {
                    image::Rgba {
                        data: [255, 0, 0, 255],
                    } => tiles.push(Tiles::Wall(rand::thread_rng().gen_range(0_usize, 3))),
                    image::Rgba {
                        data: [0, 0, 255, 255],
                    } => {
                        possible_door_positions.push(Vector2::new(x,y));
                        tiles.push(Tiles::WallTop(rand::thread_rng().gen_range(0_usize, 3)));
                    }
                    image::Rgba {
                        data: [0, 255, 255, 255],
                    } => tiles.push(Tiles::WallTop(rand::thread_rng().gen_range(0_usize, 3))),
                    image::Rgba {
                        data: [0, 255, 0, 255],
                    } => tiles.push(Tiles::Grass(rand::thread_rng().gen_range(0_usize, 2))),
                    image::Rgba {
                        data: [255, 255, 0, 255],
                    } => tiles.push(Tiles::SpawnPoint(rand::thread_rng().gen_range(0_usize, 2))),
                    _ => tiles.push(Tiles::Empty),
                }
            }
        }

        let mut load_info = LoadInfo::default();
        for neighbour in self.neighbours.iter() {
            match neighbour {
                Neighbour::Invalid => continue,
                Neighbour::North(id) => {
                    assert!(possible_door_positions.len() > 0);
                    let mut north_pos = possible_door_positions[0];
                    let mut idx = 0;
                    for (i, pos) in possible_door_positions.iter().enumerate() {
                        if pos.y < north_pos.y {
                            north_pos = *pos;
                            idx = i;
                        }
                    }
                    possible_door_positions.remove(idx);
                    tiles[(north_pos.y * width + north_pos.x) as usize] = Tiles::Door;
                    load_info.doors[0] = Some((north_pos, *id));
                }
                Neighbour::East(id) => {
                    assert!(possible_door_positions.len() > 0);
                    let mut east_pos = possible_door_positions[0];
                    let mut idx = 0;
                    for (i, pos) in possible_door_positions.iter().enumerate() {
                        if pos.x > east_pos.x {
                            east_pos = *pos;
                            idx = i;
                        }
                    }
                    possible_door_positions.remove(idx);
                    tiles[(east_pos.y * width + east_pos.x) as usize] = Tiles::Door;
                    load_info.doors[1] = Some((east_pos, *id));
                }
                Neighbour::South(id) => {
                    assert!(possible_door_positions.len() > 0);
                    let mut south_pos = possible_door_positions[0];
                    let mut idx = 0;
                    for (i, pos) in possible_door_positions.iter().enumerate() {
                        if pos.y > south_pos.y {
                            south_pos = *pos;
                            idx = i;
                        }
                    }
                    possible_door_positions.remove(idx);
                    tiles[(south_pos.y * width + south_pos.x) as usize] = Tiles::Door;
                    load_info.doors[2] = Some((south_pos, *id));
                }
                Neighbour::West(id) => {
                    assert!(possible_door_positions.len() > 0);
                    let mut west_pos = possible_door_positions[0];
                    let mut idx = 0;
                    for (i, pos) in possible_door_positions.iter().enumerate() {
                        if pos.x < west_pos.x {
                            west_pos = *pos;
                            idx = i;
                        }
                    }
                    possible_door_positions.remove(idx);
                    tiles[(west_pos.y * width + west_pos.x) as usize] = Tiles::Door;
                    load_info.doors[3] = Some((west_pos, *id));
                }
            }
        }
        (
            self.id,
            Room {
                neighbours: self.neighbours,
                dimensions: (width as _, height as _).into(),
                tiles,
                room_type: self.room_type,
                grid_pos: self.grid_pos,
                load_info,
            },
        )
    }
}

#[derive(Debug, Copy, Clone, Default)]
pub struct LoadInfo {
    pub doors: [Option<(Vector2<u32>, RoomId)>; MAX_NEIGHBOUR],
}

pub struct Room {
    pub dimensions: Vector2<i32>,
    pub neighbours: [Neighbour; MAX_NEIGHBOUR],
    pub tiles: Vec<Tiles>,
    pub room_type: RoomType,
    pub grid_pos: Vector2<i32>,
    pub load_info: LoadInfo,
}

fn right_shift_vec(vec: Vector2<i32>, value: u32) -> Vector2<i32> {
    [vec.x>> value, vec.y>> value].into()
}

impl Room {
    //pub fn update(&mut self) {}

    pub fn render(&self, offset: Vector2<i32>, screen: &mut Screen) {
        screen.set_offset(offset);
        let Vector2 {x: x0, y: y0} = right_shift_vec(offset, SPRITE_SIZE_SHIFT_VALUE);
        let Vector2 {x: x1, y: y1} = right_shift_vec(offset + screen.dimensions.cast().unwrap(), SPRITE_SIZE_SHIFT_VALUE);
        for y in y0..=y1 {
            for x in x0..=x1 {
                self.get_tile(x, y).render(x, y, screen);
            }
        }
    }

    pub fn get_tile(&self, x: i32, y: i32) -> &'static super::tile::Tile {
        if x < 0 || x >= self.dimensions.x || y < 0 || y >= self.dimensions.y {
            return &super::tile::VOID_TILE;
        }
        match self
            .tiles
            .get((x + y * self.dimensions.x) as usize)
            .expect("Out of bounds")
        {
            Tiles::Ground => &super::tile::GROUND_TILES[0],
            Tiles::Wall(i) => &super::tile::WALL_TILES[*i],
            Tiles::WallTop(i) => &super::tile::WALL_TOP_TILES[*i],
            Tiles::Grass(i) => &super::tile::GRASS_TILES[*i],
            Tiles::SpawnPoint(i) => &super::tile::GRASS_TILES[*i],
            Tiles::Door => &super::tile::DOOR_TILE,
            _ => &super::tile::VOID_TILE,
        }
    }

    pub fn dimensions(&self) -> Vector2<i32> {
        self.dimensions * SPRITE_SIZE_U32 as i32
    }

    pub fn middle_point(&self) -> Vector2<f32> {
        for (i, tile) in self.tiles.iter().enumerate() {
            if let Tiles::SpawnPoint(_) = tile {
                return (
                    (i as i32 % self.dimensions.x) as f32 * 8.,
                    (i as i32 / self.dimensions.x) as f32 * 8.,
                ).into();
            }
        }
        return (0., 0.).into();
    }
}
