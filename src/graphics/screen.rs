use image::{GenericImage, GenericImageView};
use rand::Rng;
use super::super::level::tile::Tile;

pub struct Screen {
    pub width: u32,
    pub height: u32,
    x_offset: i32,
    y_offset: i32,
    pub canvas: image::RgbaImage,
    tiles: Vec<[u8; 4]>,
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
            tiles,
        }
    }

    /*pub fn render(&mut self, x_offset: i32, y_offset: i32) {
        use super::sprite::GROUNDS;
        let pixels = GROUNDS[0].view();
        let mask = GROUNDS[0].size - 1;

        for h in 0..self.height {
            let hp = h as i32 + (y_offset / 2 );
            if hp < 0 || hp >= self.height as i32 {
                continue;
            }
            for w in 0..self.width {
                let wp = w as i32 + (x_offset / 2);
                if wp < 0 || wp >= self.width as i32 {
                    continue;
                }

                self.canvas.put_pixel(wp as _, hp as _, pixels.get_pixel(w & mask, h & mask));
            }
        }
    }*/

    pub fn render_tile(&mut self, mut xp: i32, mut yp: i32, tile: &Tile) {
        xp -= self.x_offset;
        yp -= self.y_offset;
        for y in 0..tile.sprite.size {
            let ya = y as i32 + yp;
            for x in 0..tile.sprite.size {
                let xa = x as i32 + xp;
                if xa < 0 || xa >= self.width as i32
                    || ya < 0 || ya >= self.height as i32 {
                    continue;
                }
                self.canvas.put_pixel(xa as u32, ya as u32, tile.sprite.view().get_pixel(x, y))
            }
        }
    }

    pub fn clear(&mut self) {
        for pixel in self.canvas.pixels_mut() {
            *pixel = image::Rgba([0, 0, 0, 255]);
        };
    }

    pub fn set_offset(&mut self, x_offset: i32, y_offset: i32) {
        self.x_offset = x_offset;
        self.y_offset = y_offset;
    }
}
