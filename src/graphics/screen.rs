pub struct Screen {
    width: u32,
    height: u32,
    pub canvas: image::ImageBuffer<image::Rgba<u8>, Vec<u8>>,
    tiles: Vec<[u8; 4]>,
}

impl Screen {
    pub fn new(width: u32, height: u32) -> Screen {
        use rand::Rng;

        let canvas = image::ImageBuffer::new(width, height);
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
        for h in 0..self.height {
            for w in 0..self.width {
                let index = ((w >> 4) + (h >> 4) * 64) as usize;
                //println!("{}", index);
                self.canvas.put_pixel(w, h, image::Rgba(*self.tiles.get(index).unwrap()));
            }
        }
    }

    pub fn clear(&mut self) {
        for pixel in self.canvas.pixels_mut() {
            *pixel = image::Rgba([0, 0, 0, 255]);
        };
    }
}
