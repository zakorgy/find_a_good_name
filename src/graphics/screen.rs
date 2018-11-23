use rand::Rng;

pub struct Screen {
    width: u32,
    height: u32,
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
            canvas,
            tiles,
        }
    }

    pub fn render(&mut self) {
        use image::{GenericImage, GenericImageView};
        use super::sprite::GROUNDS;

        for h in (0..=self.height).step_by(8) {
            let pixels = GROUNDS[0].view();
            for w in (0..=self.width).step_by(8) {
                self.canvas.copy_from(&pixels, w, h);
            }
        }
    }

    pub fn clear(&mut self) {
        for pixel in self.canvas.pixels_mut() {
            *pixel = image::Rgba([0, 0, 0, 255]);
        };
    }
}
