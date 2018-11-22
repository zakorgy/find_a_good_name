use super::sprite::{SpriteSheet, SpriteView};

pub struct Screen {
    width: u32,
    height: u32,
    pub canvas: image::RgbaImage,
    tiles: Vec<[u8; 4]>,
}

impl Screen {
    pub fn new(width: u32, height: u32) -> Screen {
        use rand::Rng;

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
            canvas,
            tiles,
        }
    }

    pub fn render(&mut self, sheet: &SpriteSheet, sprite: &SpriteView) {
        use image::GenericImageView;

        let pixels = sprite.view(&sheet);
        let mask = sprite.size - 1;
        for h in 0..self.height {
            for w in 0..self.width {
                self.canvas.put_pixel(w, h, pixels.get_pixel(w & mask, h & mask));
            }
        }
    }

    pub fn clear(&mut self) {
        for pixel in self.canvas.pixels_mut() {
            *pixel = image::Rgba([0, 0, 0, 255]);
        };
    }
}
