use super::super::level::{MapInfo, Tile, CURRENT_ROOM_TILE, NO_ROOM_TILE, ROOM_TILE};
use image::GenericImageView;
use rand::Rng;

pub struct Screen {
    pub width: u32,
    pub height: u32,
    x_offset: i32,
    y_offset: i32,
    pub canvas: image::RgbaImage,
}

impl Screen {
    pub fn new(width: u32, height: u32) -> Screen {
        let canvas = image::RgbaImage::new(width, height);
        let mut tiles = Vec::with_capacity(64 * 64);
        for _ in 0..(64 * 64) {
            tiles.push([
                rand::thread_rng().gen_range(0, 255),
                rand::thread_rng().gen_range(0, 255),
                rand::thread_rng().gen_range(0, 255),
                255,
            ]);
        }
        Screen {
            width,
            height,
            x_offset: 0,
            y_offset: 0,
            canvas,
        }
    }

    pub fn render_tile(&mut self, mut xp: i32, mut yp: i32, tile: &Tile) {
        xp -= self.x_offset;
        yp -= self.y_offset;
        for y in 0..tile.sprite.size {
            let ya = y as i32 + yp;
            for x in 0..tile.sprite.size {
                let xa = x as i32 + xp;
                if xa < 0 || xa >= self.width as i32 || ya < 0 || ya >= self.height as i32 {
                    continue;
                }
                self.canvas
                    .put_pixel(xa as u32, ya as u32, tile.sprite.view().get_pixel(x, y))
            }
        }
    }

    pub fn render_map<'a>(&mut self, map_info: MapInfo<'a>) {
        use image::GenericImage;
        let x_start = 41;
        let y_start = 0;
        for x in 0..9 {
            for y in 0..9 {
                if map_info.map_grid[x][y] {
                    if (x as i32, y as i32) == map_info.current_grid_pos {
                        self.canvas.copy_from(
                            &CURRENT_ROOM_TILE.sprite.view(),
                            x_start + (x as u32) << 2,
                            y_start + (y as u32) << 2,
                        );
                    } else {
                        self.canvas.copy_from(
                            &ROOM_TILE.sprite.view(),
                            x_start + (x as u32) << 2,
                            y_start + (y as u32) << 2,
                        );
                    }
                } else {
                    self.canvas.copy_from(
                        &NO_ROOM_TILE.sprite.view(),
                        x_start + (x as u32) << 2,
                        y_start + (y as u32) << 2,
                    );
                }
            }
        }
    }

    pub fn clear(&mut self) {
        for pixel in self.canvas.pixels_mut() {
            *pixel = image::Rgba([0, 0, 0, 255]);
        }
    }

    pub fn set_offset(&mut self, x_offset: i32, y_offset: i32) {
        self.x_offset = x_offset;
        self.y_offset = y_offset;
    }
}
