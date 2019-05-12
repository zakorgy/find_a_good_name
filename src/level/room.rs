use super::super::graphics::image;
use super::super::graphics::Screen;
use rand::Rng;
use std::convert::From;
use std::default::Default;
use std::path::PathBuf;

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

#[derive(Default)]
pub struct RoomBuilder {
    neighbours: [Neighbour; 4],
    room_type: RoomType,
    grid_pos: (i32, i32),
    path: PathBuf,
    pub id: RoomId,
}

impl RoomBuilder {
    pub fn new() -> RoomBuilder {
        RoomBuilder {
            neighbours: [Neighbour::Invalid; 4],
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
                        possible_door_positions.push((x,y));
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
                        if pos.1 < north_pos.1 {
                            north_pos = *pos;
                            idx = i;
                        }
                    }
                    /*let mut first_empty = load_info.doors.iter_mut().find(|i| i.is_none()).unwrap();
                    first_empty = &mut Some((north_pos, *id));*/
                    possible_door_positions.remove(idx);
                    tiles[(north_pos.1 * width + north_pos.0) as usize] = Tiles::Door;
                    load_info.doors[0] = Some((north_pos, *id));
                }
                Neighbour::East(id) => {
                    assert!(possible_door_positions.len() > 0);
                    let mut east_pos = possible_door_positions[0];
                    let mut idx = 0;
                    for (i, pos) in possible_door_positions.iter().enumerate() {
                        if pos.0 > east_pos.0 {
                            east_pos = *pos;
                            idx = i;
                        }
                    }
                    /*let mut first_empty = load_info.doors.iter_mut().find(|i| i.is_none()).unwrap();
                    first_empty = &mut Some((east_pos, *id));*/
                    possible_door_positions.remove(idx);
                    tiles[(east_pos.1 * width + east_pos.0) as usize] = Tiles::Door;
                    load_info.doors[1] = Some((east_pos, *id));
                }
                Neighbour::South(id) => {
                    assert!(possible_door_positions.len() > 0);
                    let mut south_pos = possible_door_positions[0];
                    let mut idx = 0;
                    for (i, pos) in possible_door_positions.iter().enumerate() {
                        if pos.1 > south_pos.1 {
                            south_pos = *pos;
                            idx = i;
                        }
                    }
                    /*let mut first_empty = load_info.doors.iter_mut().find(|i| i.is_none()).unwrap();
                    first_empty = &mut Some((south_pos, *id));*/
                    possible_door_positions.remove(idx);
                    tiles[(south_pos.1 * width + south_pos.0) as usize] = Tiles::Door;
                    load_info.doors[2] = Some((south_pos, *id));
                }
                Neighbour::West(id) => {
                    assert!(possible_door_positions.len() > 0);
                    let mut west_pos = possible_door_positions[0];
                    let mut idx = 0;
                    for (i, pos) in possible_door_positions.iter().enumerate() {
                        if pos.0 < west_pos.0 {
                            west_pos = *pos;
                            idx = i;
                        }
                    }
                    /*let mut first_empty = load_info.doors.iter_mut().find(|i| i.is_none()).unwrap();
                    first_empty = &mut Some((west_pos, *id));*/
                    possible_door_positions.remove(idx);
                    tiles[(west_pos.1 * width + west_pos.0) as usize] = Tiles::Door;
                    load_info.doors[3] = Some((west_pos, *id));
                }
            }
        }
        (
            self.id,
            Room {
                neighbours: self.neighbours,
                width: width as _,
                height: height as _,
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
    pub doors: [Option<((u32, u32), RoomId)>; 4],
}

pub struct Room {
    pub width: i32,
    pub height: i32,
    pub neighbours: [Neighbour; 4],
    pub tiles: Vec<Tiles>,
    pub room_type: RoomType,
    pub grid_pos: (i32, i32),
    pub load_info: LoadInfo,
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
        match self
            .tiles
            .get((x + y * self.width) as usize)
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

    pub fn dimensions(&self) -> (i32, i32) {
        (self.width * 8, self.height * 8)
    }

    pub fn spawn_point(&self) -> (f32, f32) {
        for (i, tile) in self.tiles.iter().enumerate() {
            if let Tiles::SpawnPoint(_) = tile {
                return (
                    (i as i32 % self.width) as f32 * 8f32,
                    (i as i32 / self.width) as f32 * 8f32,
                );
            }
        }
        return (0f32, 0f32);
    }
}
