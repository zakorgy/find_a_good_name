use rand::Rng;
use std::convert::From;
use std::path::PathBuf;
use super::super::graphics::screen::Screen;

pub struct Level {
    pub width : i32,
    pub height: i32,
    tiles: Vec<Tiles>,
}

pub enum Tiles {
    Empty,
    Ground,
    Wall,
    Door,
}

impl From<u32> for Tiles {
    fn from(num: u32) -> Tiles {
        match num {
            1 => Tiles::Ground,
            2 => Tiles::Wall,
            3 => Tiles::Door,
            _ => Tiles::Empty,
        }
    }
}

impl Level {

    pub fn new(width: i32, height: i32) -> Level {
        let mut level = Level {
            width,
            height,
            tiles: Vec::new(),
        };

        level.generate_level();
        level
    }

    pub fn load_level(path: PathBuf) -> Level {
        unimplemented!()
    }

    fn generate_level(&mut self) {
        for _ in 0..self.height {
            for _ in 0..self.width {
                self.tiles.push(rand::thread_rng().gen_range(0_u32, 4).into())
            }
        }
    }

    pub fn update(&mut self) {

    }

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

    fn get_tile(&self, x: i32, y: i32) -> &'static super::tile::Tile {
        if x < 0 || x >= self.width || y < 0 || y >= self.height {
            return &super::tile::VOID_TILE;
        }
        match self.tiles.get((x + y * self.width) as usize).expect("Out of bounds") {
            Tiles::Ground | Tiles::Wall => &super::tile::GROUND_TILE0,
            _ => &super::tile::GROUND_TILE1,
        }
    }
}
