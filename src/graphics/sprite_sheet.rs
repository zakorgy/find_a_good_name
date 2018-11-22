use image::{RgbaImage, DynamicImage};

pub struct SpriteSheet {
    path: String,
    width: u32,
    height: u32,
    image: RgbaImage,
}

impl SpriteSheet {
    pub fn new(path: String, width: u32, height: u32) -> SpriteSheet {
        let image = match image::open(&path) {
            Ok(image) => image.to_rgba(),
            Err(err) => panic!("Error loading image: {:?}", err),
        };

        SpriteSheet {path, width, height, image}
    }
}
