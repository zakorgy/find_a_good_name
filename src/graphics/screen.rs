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

    pub fn render(&mut self, x_offset: i32, y_offset: i32) {
        use image::{GenericImage, GenericImageView};
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
    }

    pub fn clear(&mut self) {
        for pixel in self.canvas.pixels_mut() {
            *pixel = image::Rgba([0, 0, 0, 255]);
        };
    }
}
