use rand::Rng;
use std::convert::From;
use std::path::PathBuf;
use super::super::graphics::Screen;
use super::super::graphics::image;

pub(crate) type RoomId = u8;

#[derive(Default)]
pub(crate) struct Neighbours {
    pub north: Option<RoomId>,
    pub east: Option<RoomId>,
    pub south: Option<RoomId>,
    pub west: Option<RoomId>,
}

pub struct Room {
    neighbours: Neighbours,
    pub width : i32,
    pub height: i32,
    pub tiles: Vec<Tiles>,
}

pub enum Tiles {
    Empty,
    Grass(usize),
    Ground,
    Wall(usize),
    WallTop(usize),
    Door,
}

impl From<u32> for Tiles {
    fn from(num: u32) -> Tiles {
        match num {
            1 => Tiles::Ground,
            2 => Tiles::Grass(0),
            3 => Tiles::Wall(0),
            4 => Tiles::WallTop(0),
            5 => Tiles::Door,
            _ => Tiles::Empty,
        }
    }
}

impl Room {
    pub fn load_room(path: &PathBuf) -> Room {
        let image = match image::open(&path) {
            Ok(image) => image.to_rgba(),
            Err(err) => panic!("Error loading image: {:?}", err),
        };
        let (width, height) = image.dimensions();
        let mut tiles = Vec::new();
        for y in 0..height {
            for x in 0..width {
                match image.get_pixel(x, y) {
                    image::Rgba{ data: [255, 0, 0, 255]} => tiles.push(Tiles::Wall(rand::thread_rng().gen_range(0_usize, 3))),
                    image::Rgba{ data: [0, 255, 255, 255]} => tiles.push(Tiles::WallTop(rand::thread_rng().gen_range(0_usize, 3))),
                    image::Rgba{ data: [0, 255, 0, 255]} => tiles.push(Tiles::Grass(rand::thread_rng().gen_range(0_usize, 2))),
                    _ => tiles.push(Tiles::Empty),
                }
            }
        }

        Room {
            neighbours: Default::default(),
            width: width as _,
            height: height as _,
            tiles,
        }
    }

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
            _ => &super::tile::VOID_TILE,
        }
    }

    pub fn dimensions(&self) -> (i32, i32) {
        (self.width * 8, self.height * 8)
    }
}
