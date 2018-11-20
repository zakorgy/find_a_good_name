extern crate image;

pub struct Screen {
    width: u32,
    height: u32,
    pub canvas: image::ImageBuffer<image::Rgba<u8>, Vec<u8>>,
}

impl Screen {
    pub fn new(width: u32, height: u32) -> Screen {
        let canvas = image::ImageBuffer::new(width, height);
        Screen {
            width,
            height,
            canvas,
        }
    }

    pub fn render(&mut self) {
        for y in 0..self.height {
            for x in 0..self.width {
                self.canvas.put_pixel(x, y, image::Rgba([255, 0, 255, 255]));
            }
        }
    }
}
