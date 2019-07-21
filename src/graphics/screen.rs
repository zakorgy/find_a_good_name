use crate::entity::Direction;
use crate::level::{MapInfo, tile::Tile};

use cgmath::Vector2;
use image::GenericImageView;

pub struct Screen {
    pub dimensions: Vector2<u32>,
    offset: Vector2<i32>,
    canvas: image::RgbaImage,
}

impl Screen {
    pub fn new(width: u32, height: u32) -> Screen {
        let canvas = image::RgbaImage::new(width, height);
        Screen {
            dimensions: (width, height).into(),
            offset: (0, 0).into(),
            canvas,
        }
    }

    pub fn put_pixel(&mut self, x: u32, y: u32, pixel: image::Rgba<u8>) {
        if x >= self.dimensions.x || y >= self.dimensions.y {
            return;
        }
        self.canvas.put_pixel(x, y, pixel);
    }

    pub fn canvas(&self) -> &image::RgbaImage {
        &self.canvas
    }

    pub fn render_tile(&mut self, mut position: Vector2<i32>, tile: &Tile, orientation: Direction) {
        position -= self.offset;
        for y in 0..tile.sprite.size {
            let ya = y as i32 + position.y;
            for x in 0..tile.sprite.size {
                let xa = x as i32 + position.x;
                if xa < 0
                    || xa >= self.dimensions.x as i32
                    || ya < 0
                    || ya >= self.dimensions.y as i32
                {
                    continue;
                }
                let (pixel_x, pixel_y) = match orientation {
                    Direction::UP => (x, y),
                    Direction::DOWN => (x, tile.sprite.size - 1 - y),
                    Direction::LEFT => (y, x),
                    Direction::RIGHT => (y, tile.sprite.size - 1 - x),
                    _ => unimplemented!(),
                };
                self.canvas.put_pixel(
                    xa as u32,
                    ya as u32,
                    tile.sprite.view().get_pixel(pixel_x, pixel_y),
                )
            }
        }
    }

    pub fn render_map<'a>(&mut self, _map_info: MapInfo<'a>) {
        let _x_start = 41;
        let _y_start = 0;
        /*for x in 0..MAP_GRID_SIZE {
            for y in 0..MAP_GRID_SIZE {
                if map_info.map_grid[x][y] {
                    if (x as i32, y as i32) == map_info.current_grid_pos.into() {
                        self.canvas.copy_from(
                            &CURRENT_ROOM_TILE.sprite.view(),
                            x_start + (x as u32)<< HALF_SPRITE_SIZE_SHIFT_VALUE,
                            y_start + (y as u32)<< HALF_SPRITE_SIZE_SHIFT_VALUE,
                        );
                    } else {
                        self.canvas.copy_from(
                            &ROOM_TILE.sprite.view(),
                            x_start + (x as u32)<< HALF_SPRITE_SIZE_SHIFT_VALUE,
                            y_start + (y as u32)<< HALF_SPRITE_SIZE_SHIFT_VALUE,
                        );
                    }
                } else {
                    self.canvas.copy_from(
                        &NO_ROOM_TILE.sprite.view(),
                        x_start + (x as u32)<< HALF_SPRITE_SIZE_SHIFT_VALUE,
                        y_start + (y as u32)<< HALF_SPRITE_SIZE_SHIFT_VALUE,
                    );
                }
            }
        }*/
    }

    pub fn clear(&mut self) {
        for pixel in self.canvas.pixels_mut() {
            *pixel = image::Rgba([0, 0, 0, 255]);
        }
    }

    pub fn set_offset(&mut self, offset: Vector2<i32>) {
        self.offset = offset;
    }
}
